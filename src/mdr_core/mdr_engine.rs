use std::sync::Arc;

use vulkano::{
  buffer::{BufferUsage, CpuAccessibleBuffer, TypedBufferAccess},
  command_buffer::{
    AutoCommandBufferBuilder, CommandBufferUsage, PrimaryAutoCommandBuffer, SubpassContents,
  },
  format::Format,
  image::view::ImageView,
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
  mdr_device::MdrDevice,
  mdr_pipeline::{MdrPipeline, Vertex},
  mdr_swapchain::MdrSwapchain,
  mdr_window::{MdrWindow, MdrWindowOptions},
};

pub struct MdrEngine {
  instance: Arc<Instance>,
  debug_callback: Option<DebugCallback>,
  device: Arc<MdrDevice>,
  event_loop: EventLoop<()>,
  window: MdrWindow,
  swapchain: MdrSwapchain,
  render_pass: Arc<RenderPass>,
  pipeline: Arc<MdrPipeline>,
  viewport: Viewport,
  vertex_buffer: Arc<CpuAccessibleBuffer<[Vertex]>>,
}

impl MdrEngine {
  pub fn new(debug_enabled: bool, name: Option<&str>) -> Self {
    // Create a Vulkan instance with the required extensions.
    let instance = Self::create_instance(debug_enabled);
    // Register debug callback if necessary
    let debug_callback = {
      if debug_enabled {
        let callback = Self::register_debug_callback(&instance);
        Some(callback)
      } else {
        None
      }
    };

    // Begin by creating the event loop
    let event_loop = EventLoop::new();
    // Set up window options and create window
    let window_options = MdrWindowOptions {
      width: 800,
      height: 600,
      title: name.unwrap_or("MD Renderer"),
      resizable: true,
    };
    let window = MdrWindow::new(&instance, &event_loop, &window_options);

    // Create device
    let device = MdrDevice::new(&instance, &window);

    // Create swapchan
    let swapchain = MdrSwapchain::new(&device, &window);

    // Create render pass and viewport
    let render_pass = Self::create_render_pass(&device, swapchain.image_format());
    let viewport = window.create_viewport();

    // Create pipeline
    let pipeline = MdrPipeline::new(&device, &render_pass, &viewport);

    // Generate triangle vertex data
    vulkano::impl_vertex!(Vertex, position, color);
    let v1 = Vertex {
      position: [-0.5, 0.5],
      color: [1.0, 0.0, 0.0, 1.0],
    };
    let v2 = Vertex {
      position: [0.0, -0.5],
      color: [0.0, 1.0, 0.0, 1.0],
    };
    let v3 = Vertex {
      position: [0.5, 0.5],
      color: [0.0, 0.0, 1.0, 1.0],
    };
    let vertex_buffer = CpuAccessibleBuffer::from_iter(
      device.vk_logical_device.clone(),
      BufferUsage::vertex_buffer(),
      false,
      vec![v1, v2, v3].into_iter(),
    )
    .unwrap();

    Self {
      instance,
      debug_callback,
      device,
      event_loop,
      window,
      swapchain,
      render_pass,
      pipeline,
      viewport,
      vertex_buffer,
    }
  }

  pub fn run(mut self) {
    // Create framebuffers and command buffers
    let mut framebuffers = Self::create_frame_buffers(&self.swapchain, &self.render_pass);
    let mut command_buffers = Self::create_command_buffers(
      &self.device,
      &self.pipeline,
      &framebuffers,
      &self.vertex_buffer,
    );

    // Loop state variables
    let mut window_was_resized = false;
    let mut should_recreate_swapchain = false;

    // Frames in flight setup
    let image_count = self.swapchain.vk_images.len();
    let mut previous_frame_index = 0;
    let mut frame_fences: Vec<Option<Arc<FenceSignalFuture<_>>>> = Vec::new();
    for _ in 0..image_count {
      frame_fences.push(None);
    }

    self
      .event_loop
      .run(move |event, _, control_flow| match event {
        Event::WindowEvent {
          event: WindowEvent::CloseRequested,
          ..
        } => {
          *control_flow = ControlFlow::Exit;
        }
        Event::WindowEvent {
          event: WindowEvent::Resized(_),
          ..
        } => {
          window_was_resized = true;
        }
        Event::MainEventsCleared => {
          // Don't render if window is minimized
          let screen_dimensions = self.window.dimensions();
          if screen_dimensions.width == 0 && screen_dimensions.height == 0 {
            return;
          }

          // Resize/swapchain recreation logic
          if window_was_resized || should_recreate_swapchain {
            should_recreate_swapchain = false;

            // Recreate swapchain and framebuffers
            self.swapchain.recreate();
            framebuffers = Self::create_frame_buffers(&self.swapchain, &self.render_pass);

            if window_was_resized {
              window_was_resized = false;

              // Set new viewport dimensions, recreate pipeline and command buffers
              self.viewport.dimensions = self.window.dimensions().into();
              self.pipeline = MdrPipeline::new(&self.device, &self.render_pass, &self.viewport);
              command_buffers = Self::create_command_buffers(
                &self.device,
                &self.pipeline,
                &framebuffers,
                &self.vertex_buffer,
              );
            }
          }

          // Drawing
          // First, we acquire the index of the image to draw to
          let (image_index, is_suboptimal, acquire_future) =
            match self.swapchain.acquire_next_image() {
              Ok(r) => r,
              Err(AcquireError::OutOfDate) => {
                should_recreate_swapchain = true;
                return;
              }
              Err(e) => panic!("Failed to acquire next image: {:?}", e),
            };

          // The swapchain can be suboptimal but not out of date
          if is_suboptimal {
            // We'll use it but recreate the swapchain on the next loop
            should_recreate_swapchain = true;
          }

          // Coreograph interaction with GPU
          // Wait for the acquired image's fence to finish if applicable
          if let Some(image_fence) = &frame_fences[image_index] {
            image_fence.wait(None).unwrap();
          }
          //
          let previous_frame_future = match frame_fences[previous_frame_index].clone() {
            // If empty, create
            None => {
              let mut now = sync::now(self.device.vk_logical_device.clone());
              now.cleanup_finished();

              now.boxed()
            }
            Some(fence) => fence.boxed(),
          };
          //
          let queue = &self.device.vk_queue;
          let swapchain = &self.swapchain.vk_swapchain;
          let future = previous_frame_future
            .join(acquire_future)
            .then_execute(queue.clone(), command_buffers[image_index].clone())
            .unwrap()
            .then_swapchain_present(queue.clone(), swapchain.clone(), image_index)
            .then_signal_fence_and_flush();
          // Store fence for later access
          frame_fences[image_index] = match future {
            Ok(value) => Some(Arc::new(value)),
            Err(FlushError::OutOfDate) => {
              should_recreate_swapchain = true;
              None
            }
            Err(e) => {
              println!("Failed to flush GPU future: {:?}", e);
              None
            }
          };

          previous_frame_index = image_index;
        }
        _ => (),
      })
  }

  fn create_instance(debug_enabled: bool) -> Arc<Instance> {
    // Get extensions needed to run a window
    let required_extensions = {
      let mut extensions = vulkano_win::required_extensions();

      // If debugging is enabled, add the debug utility extension
      if debug_enabled {
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
        let mut available_layers = layers_list().unwrap();
        println!("Available debugging layers:");
        while let Some(layer) = available_layers.next() {
          println!("\t{}", layer.name());
        }

        // Os-specific layers
        #[cfg(not(target_os = "macos"))]
        output_layers.push("VK_LAYER_KHRONOS_validation".to_owned());
        #[cfg(target_os = "macos")]
        output_layers.push("VK_LAYER_KHRONOS_validation".to_owned());
      }

      output_layers
    };

    return Instance::new(InstanceCreateInfo {
      enabled_extensions: required_extensions,
      enabled_layers,
      ..Default::default()
    })
    .expect("Failed to create Vulkan instance.");
  }

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
        }
      },
      pass: {
        color: [color],
        depth_stencil: {}
      }
    )
    .unwrap();
  }

  fn create_frame_buffers(
    swapchain: &MdrSwapchain,
    render_pass: &Arc<RenderPass>,
  ) -> Vec<Arc<Framebuffer>> {
    return swapchain
      .vk_images
      .iter()
      .map(|image| {
        let view = ImageView::new_default(image.clone()).unwrap();
        Framebuffer::new(
          render_pass.clone(),
          FramebufferCreateInfo {
            attachments: vec![view],
            ..Default::default()
          },
        )
        .unwrap()
      })
      .collect::<Vec<_>>();
  }

  fn create_command_buffers(
    device: &Arc<MdrDevice>,
    pipeline: &Arc<MdrPipeline>,
    framebuffers: &Vec<Arc<Framebuffer>>,
    vertex_buffer: &Arc<CpuAccessibleBuffer<[Vertex]>>,
  ) -> Vec<Arc<PrimaryAutoCommandBuffer>> {
    return framebuffers
      .iter()
      .map(|framebuffer| {
        let mut builder = AutoCommandBufferBuilder::primary(
          device.vk_logical_device.clone(),
          device.queue_family(),
          CommandBufferUsage::MultipleSubmit,
        )
        .unwrap();

        let clear_color = vec![[0.0, 0.0, 0.0, 1.0].into()];
        builder
          .begin_render_pass(framebuffer.clone(), SubpassContents::Inline, clear_color)
          .unwrap()
          .bind_pipeline_graphics(pipeline.vk_graphics_pipeline.clone())
          .bind_vertex_buffers(0, vertex_buffer.clone())
          .draw(vertex_buffer.len() as u32, 1, 0, 0)
          .unwrap()
          .end_render_pass()
          .unwrap();

        Arc::new(builder.build().unwrap())
      })
      .collect();
  }
}
