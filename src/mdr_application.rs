use std::sync::Arc;

use vulkano::{
  format::Format,
  instance::{Instance, InstanceCreateInfo},
  render_pass::RenderPass,
};
use winit::{
  event::{Event, WindowEvent},
  event_loop::ControlFlow,
};

use crate::{
  mdr_device::MdrDevice,
  mdr_pipeline::MdrPipeline,
  mdr_window::{MdrWindow, MdrWindowOptions},
};

pub struct MdrApp {
  instance: Arc<Instance>,
  device: Arc<MdrDevice>,
  window: MdrWindow,
}

impl MdrApp {
  pub fn new(name: &str) -> Self {
    let required_extensions = vulkano_win::required_extensions();
    let instance = Instance::new(InstanceCreateInfo {
      enabled_extensions: required_extensions,
      ..Default::default()
    })
    .expect("Failed to create Vulkan instance.");

    let mut window = MdrWindow::new(
      instance.clone(),
      MdrWindowOptions {
        width: 800,
        height: 600,
        title: name,
      },
    );

    // Create device
    let device = MdrDevice::new(instance.clone(), window.surface());
    // Initialize window swapchain and retrieve buffer format
    window.initialize_swapchain(device.clone());
    let image_format = window.swapchain_image_format();
    // Create viewport and render pass
    let viewport = window.create_viewport();
    let render_pass = Self::create_render_pass(&device, image_format);
    // Create pipeline
    let pipeline = MdrPipeline::new(&device, render_pass, viewport);
    // Pipeline layout
    // Command buffers

    Self {
      instance,
      device,
      window,
    }
  }

  pub fn run(self) {
    self
      .window
      .event_loop
      .run(|event, _, control_flow| match event {
        Event::WindowEvent {
          event: WindowEvent::CloseRequested,
          ..
        } => {
          *control_flow = ControlFlow::Exit;
        }
        _ => (),
      })
  }

  fn create_render_pass(device: &Arc<MdrDevice>, image_format: Format) -> Arc<RenderPass> {
    return vulkano::single_pass_renderpass!(
      device.logical_device(),
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

  fn create_pipeline_layout(&self) {}

  fn create_pipeline(&self) {}
}
