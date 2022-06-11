use std::sync::Arc;

use vulkano::{
  device::Device,
  format::Format,
  image::{view::ImageView, AttachmentImage, ImageAccess},
  instance::{
    debug::{DebugCallback, MessageSeverity, MessageType},
    layers_list, Instance, InstanceCreateInfo, InstanceExtensions,
  },
  pipeline::graphics::viewport::Viewport,
  render_pass::{Framebuffer, FramebufferCreateInfo, RenderPass},
  swapchain::AcquireError,
  sync::{self, FenceSignalFuture, FlushError, GpuFuture},
};
use winit::{
  event::{Event, WindowEvent},
  event_loop::{ControlFlow, EventLoop},
};

use super::{
  mdr_command_buffer::MdrCommandBuffer,
  mdr_device::MdrDevice,
  mdr_pipeline::MdrPipeline,
  mdr_swapchain::MdrSwapchain,
  mdr_window::{MdrWindow, MdrWindowOptions},
};
use crate::mdr_scene::{
  mdr_mesh::{MdrMesh, Vertex},
  MdrScene,
};

pub struct MdrEngineOptions {
  pub name: String,
  pub debug: bool,
}

impl Default for MdrEngineOptions {
  fn default() -> Self {
    Self {
      name: "MD Renderer".to_string(),
      debug: false,
    }
  }
}

pub struct MdrEngine {
  instance: Arc<Instance>,
  debug_callback: Option<DebugCallback>,
  device: Arc<MdrDevice>,
  window: Arc<MdrWindow>,
  swapchain: MdrSwapchain,
  render_pass: Arc<RenderPass>,
  pipeline: Arc<MdrPipeline>,
  viewport: Viewport,
  framebuffers: Vec<Arc<Framebuffer>>,

  // Renderer state
  should_recreate_swapchain: bool,
  frame_fences: Vec<Option<Box<dyn GpuFuture>>>,
  previous_frame_index: usize,
}

impl MdrEngine {
  pub fn new(event_loop: &EventLoop<()>, options: MdrEngineOptions) -> Arc<Self> {
    // Create a Vulkan instance with the required extensions.
    let instance = Self::create_instance(options.debug);
    // Register debug callback if necessary
    let debug_callback = {
      if options.debug {
        let callback = Self::register_debug_callback(&instance);
        Some(callback)
      } else {
        None
      }
    };

    // Create window
    let window = MdrWindow::new(
      &instance,
      &event_loop,
      MdrWindowOptions {
        width: 800,
        height: 600,
        title: options.name.as_str(),
        resizable: true,
      },
    );

    // Create device
    let device = MdrDevice::new(&instance, &window);
    // Create swapchan
    let swapchain = MdrSwapchain::new(&device, &window);
    // Create render pass and viewport
    let render_pass = Self::create_render_pass(&device, swapchain.image_format());
    let viewport = window.create_viewport();
    // Create pipeline and framebuffers
    let pipeline = MdrPipeline::new(&device, &render_pass, &viewport);
    let framebuffers = Self::create_frame_buffers(&swapchain, &render_pass);

    // Set up frames in flight
    let frames_in_flight = Self::set_up_frames_in_flight(&swapchain);

    Arc::new(Self {
      instance,
      debug_callback,
      device,
      window,
      swapchain,
      render_pass,
      pipeline,
      viewport,
      framebuffers,

      should_recreate_swapchain: false,
      frame_fences: frames_in_flight,
      previous_frame_index: 0,
    })
  }

  pub fn render(&mut self, scene: Arc<MdrScene>) {
    // Don't render if window is minimized
    if self.window.is_minimized() {
      return;
    }

    // Resize/swapchain recreation logic
    if self.window.was_resized || self.should_recreate_swapchain {
      self.should_recreate_swapchain = false;

      // Recreate swapchain and framebuffers
      self.swapchain.recreate();
      self.framebuffers = Self::create_frame_buffers(&self.swapchain, &self.render_pass);

      if self.window.was_resized {
        self.window.was_resized = false;
        // Set new viewport dimensions, recreate pipeline and command buffers
        self.viewport.dimensions = self.window.dimensions().into();
        self.pipeline = MdrPipeline::new(&self.device, &self.render_pass, &self.viewport);
      }
    }

    // Upload descriptor sets
    let aspect_ratio =
      self.window.dimensions().width as f32 / self.window.dimensions().height as f32;
    let rotation: f32 = 0.0;
    let set = self.pipeline.upload_descriptor_set(aspect_ratio, rotation);

    // Drawing
    // First, we acquire the index of the image to draw to
    let (image_index, is_suboptimal, acquire_future) = match self.swapchain.acquire_next_image() {
      Ok(r) => r,
      Err(AcquireError::OutOfDate) => {
        self.should_recreate_swapchain = true;
        return;
      }
      Err(e) => panic!("Failed to acquire next image: {:?}", e),
    };

    // The swapchain can be suboptimal but not out of date
    if is_suboptimal {
      // We'll use it but recreate the swapchain on the next loop
      self.should_recreate_swapchain = true;
    }

    // Coreograph interaction with GPU
    // Create command buffers for submission to the GPU
    let command_buffers = MdrCommandBuffer::new(
      &self.device,
      &self.pipeline,
      &self.framebuffers,
      &scene,
      set,
    );

    // Future stuff
    if let Some(image_fence) = &mut self.frame_fences[image_index] {
      image_fence.flush().unwrap();
      image_fence.cleanup_finished();
    }
    // Handle None case for frame_fences
    match self.frame_fences[self.previous_frame_index] {
      None => {
        self.frame_fences[self.previous_frame_index] =
          Some(sync::now(self.device.vk_logical_device.clone()).boxed());
      }
      _ => (),
    }

    // Take previous frame off vector
    let mut previous_frame_future = self.frame_fences[self.previous_frame_index].take().unwrap();
    previous_frame_future.cleanup_finished();

    let queue = &self.device.vk_queue;
    let swapchain = &self.swapchain.vk_swapchain;
    let future = previous_frame_future
      .join(acquire_future)
      .then_execute(queue.clone(), command_buffers.get_primary(image_index))
      .unwrap()
      .then_swapchain_present(queue.clone(), swapchain.clone(), image_index)
      .then_signal_fence_and_flush();
    // Store fence for later access
    self.frame_fences[image_index] = match future {
      Ok(value) => Some(value.boxed()),
      Err(FlushError::OutOfDate) => {
        self.should_recreate_swapchain = true;
        None
      }
      Err(e) => {
        println!("Failed to flush GPU future: {:?}", e);
        None
      }
    };

    self.previous_frame_index = image_index;
  }

  pub fn get_device(&self) -> Arc<MdrDevice> {
    return self.device.clone();
  }

  pub fn get_window(&self) -> Arc<MdrWindow> {
    return self.window.clone();
  }

  fn create_instance(debug_enabled: bool) -> Arc<Instance> {}

  fn register_debug_callback(instance: &Arc<Instance>) -> DebugCallback {
    let severity = MessageSeverity {
      error: true,
      warning: true,
      information: true,
      verbose: true,
    };

    let ty = MessageType::all();

    return DebugCallback::new(instance, severity, ty, |message| {
      let severity = if message.severity.error {
        "error"
      } else if message.severity.warning {
        "warning"
      } else if message.severity.information {
        "information"
      } else if message.severity.verbose {
        "verbose"
      } else {
        panic!("No implementation for message severity");
      };

      let ty = if message.ty.general {
        "general"
      } else if message.ty.validation {
        "validation"
      } else if message.ty.performance {
        "performance"
      } else {
        panic!("No implementation for message type");
      };

      println!(
        "{} {} {}: {}",
        message.layer_prefix.unwrap_or("Unknown Layer"),
        ty,
        severity,
        message.description
      );
    })
    .unwrap();
  }

  fn create_render_pass(device: &MdrDevice, image_format: Format) -> Arc<RenderPass> {
    return vulkano::single_pass_renderpass!(
      device.vk_logical_device.clone(),
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

  fn create_frame_buffers(
    swapchain: &MdrSwapchain,
    render_pass: &Arc<RenderPass>,
  ) -> Vec<Arc<Framebuffer>> {
    let logical_device = &swapchain.vk_logical_device;
    let dimensions = swapchain.vk_images[0].dimensions().width_height();
    // Create depth buffer
    let depth_buffer_image =
      AttachmentImage::transient(logical_device.clone(), dimensions, Format::D16_UNORM).unwrap();
    let depth_buffer_view = ImageView::new_default(depth_buffer_image).unwrap();

    // Create and return framebuffers
    return swapchain
      .vk_images
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

  fn set_up_frames_in_flight(swapchain: &MdrSwapchain) -> Vec<Option<Box<dyn GpuFuture>>> {
    // Frames in flight setup
    let image_count = swapchain.vk_images.len();
    let mut frame_fences: Vec<Option<Box<dyn GpuFuture>>> = Vec::new();
    for _ in 0..image_count {
      frame_fences.push(None);
    }

    return frame_fences;
  }
}
