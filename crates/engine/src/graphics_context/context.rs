use log::{debug, info, trace};
use std::sync::Arc;
use winit::{event_loop::EventLoop, window::Window};

use vulkano::{
  device::{
    physical::{PhysicalDevice, PhysicalDeviceType, QueueFamily},
    Device, DeviceCreateInfo, DeviceExtensions, Queue, QueueCreateInfo,
  },
  format::Format,
  image::{view::ImageView, AttachmentImage, ImageAccess, ImageUsage, SwapchainImage},
  instance::{Instance, InstanceCreateInfo, InstanceExtensions},
  pipeline::graphics::viewport::Viewport,
  render_pass::{Framebuffer, FramebufferCreateInfo, RenderPass},
  shader::ShaderModule,
  swapchain::{Surface, Swapchain, SwapchainCreateInfo},
};

use crate::graphics_context::{
  pipeline::{load_shaders, MdrPipeline},
  window::{MdrWindow, MdrWindowOptions},
};

/// A Vulkan graphics context, contains Vulkano members.
pub struct MdrGraphicsContext {
  instance: Arc<Instance>,
  window: Arc<MdrWindow>,
  logical_device: Arc<Device>,
  swapchain: Arc<Swapchain<Window>>,
  swapchain_images: Vec<Arc<SwapchainImage<Window>>>,
  render_pass: Arc<RenderPass>,
  viewport: Viewport,
  pipeline: Arc<MdrPipeline>,
  framebuffers: Vec<Arc<Framebuffer>>,

  window_was_resized: bool,
  should_recreate_swapchain: bool,
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
    let (logical_device, mut queues) =
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
    let (vs, fs) = load_shaders(&logical_device);
    // Create pipeline
    let pipeline = MdrPipeline::new(&logical_device, &vs, &fs, &render_pass, &viewport);
    debug!("Created pipeline");

    // Create framebuffers
    let framebuffers = Self::create_framebuffers(&logical_device, &swapchain_images, &render_pass);
    debug!("Created framebuffers");

    Self {
      instance,
      window,
      logical_device,
      swapchain,
      swapchain_images,
      render_pass,
      viewport,
      pipeline,
      framebuffers,

      window_was_resized: false,
      should_recreate_swapchain: false,
      vs,
      fs,
    }
  }

  pub fn draw(&mut self) {
    trace!("Starting draw");

    // Skip for minimized windows
    if self.window.is_minimized() {
      trace!("Window minimized");
      return;
    }

    if self.window_was_resized || self.should_recreate_swapchain {
      self.should_recreate_swapchain = false;

      // Recreate swapchain and framebuffers
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
        self.viewport.dimensions = self.window.dimensions().into();
        self.pipeline = MdrPipeline::new(
          &self.logical_device,
          &self.vs,
          &self.fs,
          &self.render_pass,
          &self.viewport,
        )
      }
    };
  }

  /// Set context to trigger size-dependent reinitialization
  pub fn notify_resized(&mut self) {
    self.window_was_resized = true;
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
}
