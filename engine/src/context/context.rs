use log::{debug, error, info, trace};
use std::sync::Arc;
use winit::{event_loop::EventLoop, window::Window};

use vulkano::{
  buffer::{BufferUsage, CpuAccessibleBuffer},
  command_buffer::{
    AutoCommandBufferBuilder, CommandBufferUsage, PrimaryAutoCommandBuffer, RenderPassBeginInfo,
    SubpassContents,
  },
  device::{
    physical::{PhysicalDevice, PhysicalDeviceType, QueueFamily},
    Device, DeviceCreateInfo, DeviceExtensions, Queue, QueueCreateInfo,
  },
  format::{ClearValue, Format},
  image::{view::ImageView, AttachmentImage, ImageAccess, ImageUsage, SwapchainImage},
  instance::{Instance, InstanceCreateInfo, InstanceExtensions},
  pipeline::graphics::viewport::Viewport,
  render_pass::{Framebuffer, FramebufferCreateInfo, RenderPass},
  shader::ShaderModule,
  swapchain::{self, AcquireError, Surface, Swapchain, SwapchainCreateInfo},
  sync::{self, FlushError, GpuFuture},
};

use crate::{
  context::{
    pipeline::MdrPipeline,
    window::{MdrWindow, MdrWindowOptions},
  },
  scene::{MdrScene, MdrSceneObject, Vertex},
  shaders,
};

/// A Vulkan graphics context, contains Vulkano members.
pub struct MdrGraphicsContext {
  pub(crate) window: Arc<MdrWindow>,

  logical_device: Arc<Device>,
  queue: Arc<Queue>,
  swapchain: Arc<Swapchain<Window>>,
  swapchain_images: Vec<Arc<SwapchainImage<Window>>>,
  render_pass: Arc<RenderPass>,
  viewport: Viewport,
  pipeline: Arc<MdrPipeline>,
  framebuffers: Vec<Arc<Framebuffer>>,
  command_buffers: Option<Vec<Arc<PrimaryAutoCommandBuffer>>>,

  window_was_resized: bool,
  should_recreate_swapchain: bool,
  frame_futures: Vec<Option<Box<dyn GpuFuture>>>,
  previous_frame_index: usize,
  vs: Arc<ShaderModule>,
  fs: Arc<ShaderModule>,
}

impl MdrGraphicsContext {
  /// Create a new MD Renderer Graphics context with optional debug.
  pub fn new(event_loop: &EventLoop<()>, debug_enabled: bool) -> Self {
    debug!("Creating graphics context");

    // Create instance containing Vulkan function pointers
    let instance = Self::create_instance(debug_enabled);
    debug!("Created vulkan instance");

    // Create window
    let window_options = MdrWindowOptions {
      width: 800,
      height: 600,
      resizable: true,
      title: "MD Renderer",
    };
    let window = MdrWindow::new(&instance, &event_loop, window_options);
    debug!("Created window");

    // Select physical device and queue
    let (physical_device, queue_family) =
      Self::pick_physical_device(&instance, window.surface.clone());
    info!(
      "Using device: {} (type: {:?})",
      physical_device.properties().device_name,
      physical_device.properties().device_type,
    );

    // Create logical device
    let device_extensions = DeviceExtensions {
      khr_swapchain: true,
      ..DeviceExtensions::none()
    };
    let (logical_device, queue) =
      Self::create_logical_device(physical_device, device_extensions, queue_family);
    debug!("Created logical device");

    // Create swapchain
    let (swapchain, swapchain_images) =
      Self::create_swapchain(&window, &logical_device, &physical_device);
    debug!("Created swapchain");

    // Create render pass
    let render_pass = Self::create_render_pass(&logical_device, swapchain.image_format());
    debug!("Created render pass");

    // Create viewport
    let viewport = window.create_viewport();
    debug!("Created viewport");

    // Load shaders to logical device
    let (vs, fs) = shaders::load_triangle_shaders(&logical_device);
    // Create pipeline
    let pipeline = MdrPipeline::new(&logical_device, &vs, &fs, &render_pass, &viewport);
    debug!("Created pipeline");

    // Create framebuffers
    let framebuffers = Self::create_framebuffers(&logical_device, &swapchain_images, &render_pass);
    debug!("Created framebuffers");

    // Create vector of futures corresponding to each swapchain image
    let frame_futures = Self::set_up_frame_futures(swapchain_images.len());

    Self {
      window,

      logical_device,
      queue,
      swapchain,
      swapchain_images,
      render_pass,
      viewport,
      pipeline,
      framebuffers,
      command_buffers: None,

      window_was_resized: false,
      should_recreate_swapchain: false,
      frame_futures,
      previous_frame_index: 0,
      vs,
      fs,
    }
  }

  pub fn draw(&mut self, scene: &MdrScene) {
    trace!("Starting draw");

    // Skip draw for minimized windows
    if self.window.is_minimized() {
      trace!("Window minimized");
      return;
    }

    self.size_dependent_recreations();
    // Recreate command buffers if necessary
    if self.command_buffers.is_none() {
      let new_command_buffers = Self::create_command_buffers(
        &self.logical_device,
        &self.queue,
        &self.pipeline,
        &self.framebuffers,
        scene,
      );

      self.command_buffers = Some(new_command_buffers);
    }

    // First, we acquire the index of the image to draw to
    let (image_index, is_suboptimal, acquire_future) =
      match swapchain::acquire_next_image(self.swapchain.clone(), None) {
        Ok(r) => r,
        Err(AcquireError::OutOfDate) => {
          debug!("Swapchain out of date, flagging for recreation");
          self.should_recreate_swapchain = true;
          return;
        }
        Err(e) => panic!("Failed to acquire next swapchain image: {:?}", e),
      };

    // The swapchain can be suboptimal but not out of date
    if is_suboptimal {
      trace!("Swapchain suboptimal, flagging for recreation");
      // We'll use it but recreate the swapchain on the next loop
      self.should_recreate_swapchain = true;
    }

    // Get last frame's end-of-command future and clean it up
    let mut previous_frame_end = match self.frame_futures[self.previous_frame_index].take() {
      Some(value) => value,
      None => sync::now(self.logical_device.clone()).boxed(),
    };
    previous_frame_end.cleanup_finished();

    let cmd_buffer = self.command_buffers.as_ref().unwrap()[image_index].clone();
    let future = previous_frame_end
      .join(acquire_future)
      .then_execute(self.queue.clone(), cmd_buffer)
      .unwrap()
      .then_swapchain_present(self.queue.clone(), self.swapchain.clone(), image_index)
      .then_signal_fence_and_flush();

    let end_of_frame_future = match future {
      Ok(future) => future.boxed(),
      Err(FlushError::OutOfDate) => {
        self.should_recreate_swapchain = true;
        sync::now(self.logical_device.clone()).boxed()
      }
      Err(e) => {
        error!("Failed to flush future: {}", e);
        sync::now(self.logical_device.clone()).boxed()
      }
    };

    // Store as previous frame
    self.frame_futures[image_index] = Some(end_of_frame_future);
    self.previous_frame_index = image_index;
    trace!("Completed draw")
  }

  fn size_dependent_recreations(&mut self) {
    if self.window_was_resized || self.should_recreate_swapchain {
      self.should_recreate_swapchain = false;

      // Recreate swapchain and framebuffers
      trace!("Recreating swapchain");
      let mut recreate_info = self.swapchain.create_info();
      recreate_info.image_extent = self.window.dimensions().into();
      (self.swapchain, self.swapchain_images) = self.swapchain.recreate(recreate_info).unwrap();
      self.framebuffers = Self::create_framebuffers(
        &self.logical_device,
        &self.swapchain_images,
        &self.render_pass,
      );

      if self.window_was_resized {
        self.window_was_resized = false;

        // Recreate viewport and pipeline
        trace!("Window resized, recreating pipeline");
        self.viewport.dimensions = self.window.dimensions().into();
        self.pipeline = MdrPipeline::new(
          &self.logical_device,
          &self.vs,
          &self.fs,
          &self.render_pass,
          &self.viewport,
        );
      }

      self.invalidate_command_buffers();
    }
  }

  /// Set context to trigger size-dependent reinitialization
  pub fn notify_resized(&mut self) {
    self.window_was_resized = true;
  }

  fn invalidate_command_buffers(&mut self) {
    self.command_buffers = None;
  }

  pub fn create_command_buffers(
    logical_device: &Arc<Device>,
    queue: &Arc<Queue>,
    pipeline: &Arc<MdrPipeline>,
    framebuffers: &Vec<Arc<Framebuffer>>,
    scene: &MdrScene,
  ) -> Vec<Arc<PrimaryAutoCommandBuffer>> {
    let command_buffers = framebuffers
      .iter()
      .map(|framebuffer| {
        // Create command buffer builder
        let mut builder = AutoCommandBufferBuilder::primary(
          logical_device.clone(),
          queue.family(),
          CommandBufferUsage::MultipleSubmit,
        )
        .unwrap();

        // Clear color used when drawing bacground
        let clear_color_value = ClearValue::Float([0.1, 0.1, 0.1, 1.0]);
        let clear_depth_value = ClearValue::Depth(1.0);

        // Build command buffer
        let mut begin_render_pass_info = RenderPassBeginInfo::framebuffer(framebuffer.clone());
        begin_render_pass_info.clear_values =
          vec![Some(clear_color_value), Some(clear_depth_value)];
        // Begin render pass
        builder
          .begin_render_pass(begin_render_pass_info, SubpassContents::Inline)
          .unwrap();

        // Draw
        builder.bind_pipeline_graphics(pipeline.graphics_pipeline.clone());
        for object in scene.scene_objects.iter() {
          let (vertex_buffer, index_buffer, index_count) =
            Self::upload_scene_object(&logical_device, object);

          builder
            .bind_vertex_buffers(0, vertex_buffer.clone())
            .bind_index_buffer(index_buffer)
            .draw_indexed(index_count, 1, 0, 0, 0)
            .unwrap();
        }

        // End render pass
        builder.end_render_pass().unwrap();

        // Build
        Arc::new(builder.build().unwrap())
      })
      .collect();

    trace!("Created command buffers");
    command_buffers
  }

  fn upload_scene_object(
    logical_device: &Arc<Device>,
    object: &MdrSceneObject,
  ) -> (
    Arc<CpuAccessibleBuffer<[Vertex]>>,
    Arc<CpuAccessibleBuffer<[u32]>>,
    u32,
  ) {
    let vertex_buffer = CpuAccessibleBuffer::from_iter(
      logical_device.clone(),
      BufferUsage::vertex_buffer(),
      false,
      object.mesh.vertices.clone(),
    )
    .unwrap();

    let index_buffer = CpuAccessibleBuffer::from_iter(
      logical_device.clone(),
      BufferUsage::index_buffer(),
      false,
      object.mesh.indices.clone(),
    )
    .unwrap();

    (
      vertex_buffer,
      index_buffer,
      object.mesh.indices.len() as u32,
    )
  }

  /// Create a Vulkan instance with optional debug extensions.
  fn create_instance(debug_enabled: bool) -> Arc<Instance> {
    let required_extensions = {
      let mut extensions = vulkano_win::required_extensions();

      // If debugging is enabled, add the debug utility extension
      if debug_enabled {
        info!("Debug enabled");
        let debug_extensions = InstanceExtensions {
          ext_debug_utils: true,
          ..InstanceExtensions::none()
        };
        extensions = extensions.union(&debug_extensions);
      }

      extensions
    };

    // Enable layers
    let enabled_layers = {
      let mut output_layers: Vec<String> = vec![];

      // Ignore layers if not in debug mode
      if debug_enabled {
        // Print out available layers
        debug!("Available debugging layers:");
        let mut available_layers = vulkano::instance::layers_list().unwrap();
        while let Some(layer) = available_layers.next() {
          debug!("\t{}", layer.name());
        }

        // Push validation layer
        output_layers.push("VK_LAYER_KHRONOS_validation".to_owned());
        debug!("Enabled layer: VK_LAYER_KHRONOS_validation")
      }

      output_layers
    };

    match Instance::new(InstanceCreateInfo {
      enabled_extensions: required_extensions,
      enabled_layers,
      ..Default::default()
    }) {
      Ok(instance) => instance,
      Err(e) => {
        panic!("Failed to create instance: {}", e);
      }
    }
  }

  /// Select a physical device to use. Returns the device and associated queue family.
  fn pick_physical_device(
    instance: &Arc<Instance>,
    surface: Arc<Surface<Window>>,
  ) -> (PhysicalDevice, QueueFamily) {
    let device_extensions = DeviceExtensions {
      khr_swapchain: true,
      ..DeviceExtensions::none()
    };

    let device_creation_results = PhysicalDevice::enumerate(&instance)
      .filter(|&p| p.supported_extensions().is_superset_of(&device_extensions))
      .filter_map(|p| {
        p.queue_families()
          .find(|&q| q.supports_graphics() && q.supports_surface(&surface).unwrap_or(false))
          .map(|q| (p, q))
      })
      .min_by_key(|(p, _)| match p.properties().device_type {
        PhysicalDeviceType::DiscreteGpu => 0,
        PhysicalDeviceType::IntegratedGpu => 1,
        PhysicalDeviceType::VirtualGpu => 2,
        PhysicalDeviceType::Cpu => 3,
        PhysicalDeviceType::Other => 4,
      });

    match device_creation_results {
      Some(value) => value,
      None => {
        panic!("Failed to find physical device and queue family.");
      }
    }
  }

  /// Create a Vulkan logical device and queue.
  fn create_logical_device(
    physical_device: PhysicalDevice,
    device_extensions: DeviceExtensions,
    queue_family: QueueFamily,
  ) -> (Arc<Device>, Arc<Queue>) {
    let device_creation_results = Device::new(
      physical_device,
      DeviceCreateInfo {
        enabled_extensions: physical_device
          .required_extensions()
          .union(&device_extensions),
        queue_create_infos: vec![QueueCreateInfo::family(queue_family)],
        ..Default::default()
      },
    );

    match device_creation_results {
      Ok(mut value) => {
        let device = value.0;
        let queues = value.1.next().unwrap();
        (device, queues)
      }
      Err(e) => {
        panic!("Failed to create logical device: {}", e);
      }
    }
  }

  fn create_swapchain(
    window: &Arc<MdrWindow>,
    logical_device: &Arc<Device>,
    physical_device: &PhysicalDevice,
  ) -> (Arc<Swapchain<Window>>, Vec<Arc<SwapchainImage<Window>>>) {
    // Retrieve surface capabilities with respect to the physical device
    let surface = &window.surface;
    let surface_capabilities = physical_device
      .surface_capabilities(surface, Default::default())
      .expect("Failed to retrieve surface capabilities.");
    // Get other settings
    let dimensions = window.dimensions();
    let vk_image_format = Some(
      physical_device
        .surface_formats(surface, Default::default())
        .unwrap()[0]
        .0,
    );

    let swapchain_result = Swapchain::new(
      logical_device.clone(),
      surface.clone(),
      SwapchainCreateInfo {
        min_image_count: surface_capabilities.min_image_count + 1,
        image_format: vk_image_format,
        image_extent: dimensions.into(),
        image_usage: ImageUsage::color_attachment(),
        composite_alpha: surface_capabilities
          .supported_composite_alpha
          .iter()
          .next()
          .unwrap(),
        ..Default::default()
      },
    );

    match swapchain_result {
      Ok(value) => value,
      Err(e) => {
        panic!("Failed to generate swapchain: {}", e);
      }
    }
  }

  fn create_render_pass(logical_device: &Arc<Device>, image_format: Format) -> Arc<RenderPass> {
    return vulkano::single_pass_renderpass!(
      logical_device.clone(),
      attachments: {
        color: {
          load: Clear,
          store: Store,
          format: image_format,
          samples: 1,
        },
        depth: {
          load: Clear,
          store: DontCare,
          format: Format::D16_UNORM,
          samples: 1,
        }
      },
      pass: {
        color: [color],
        depth_stencil: {depth}
      }
    )
    .unwrap();
  }

  fn create_framebuffers(
    logical_device: &Arc<Device>,
    swapchain_images: &Vec<Arc<SwapchainImage<Window>>>,
    render_pass: &Arc<RenderPass>,
  ) -> Vec<Arc<Framebuffer>> {
    let dimensions = swapchain_images[0].dimensions().width_height();
    // Create depth buffer
    let depth_buffer_image =
      AttachmentImage::transient(logical_device.clone(), dimensions, Format::D16_UNORM).unwrap();
    let depth_buffer_view = ImageView::new_default(depth_buffer_image).unwrap();

    // Create and return framebuffers
    return swapchain_images
      .iter()
      .map(|image| {
        let color_view = ImageView::new_default(image.clone()).unwrap();
        Framebuffer::new(
          render_pass.clone(),
          FramebufferCreateInfo {
            // Attach color and depth view
            attachments: vec![color_view, depth_buffer_view.clone()],
            ..Default::default()
          },
        )
        .unwrap()
      })
      .collect::<Vec<_>>();
  }

  fn set_up_frame_futures(frame_count: usize) -> Vec<Option<Box<dyn GpuFuture>>> {
    // Frames in flight setup
    let mut frame_futures: Vec<Option<Box<dyn GpuFuture>>> = Vec::new();
    for _ in 0..frame_count {
      frame_futures.push(None);
    }

    return frame_futures;
  }
}
