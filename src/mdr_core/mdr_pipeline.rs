use std::sync::Arc;

use vulkano::{
  pipeline::{
    graphics::{
      input_assembly::InputAssemblyState,
      vertex_input::BuffersDefinition,
      viewport::{Viewport, ViewportState},
    },
    GraphicsPipeline,
  },
  render_pass::{RenderPass, Subpass},
  shader::ShaderModule,
};

use super::mdr_device::MdrDevice;
use crate::mdr_scene::mdr_mesh::Vertex;

mod vertex_shader {
  vulkano_shaders::shader! {
    ty: "vertex",
    path: "src/assets/shaders/basic.vert",
  }
}
mod fragment_shader {
  vulkano_shaders::shader! {
    ty: "fragment",
    path: "src/assets/shaders/basic.frag",
  }
}

pub struct MdrPipeline {
  pub vk_graphics_pipeline: Arc<GraphicsPipeline>,
}

impl MdrPipeline {
  pub fn new(device: &MdrDevice, render_pass: &Arc<RenderPass>, viewport: &Viewport) -> Arc<Self> {
    // Load shaders
    let (vs, fs) = Self::load_shaders(device);

    // Get graphics pipeline
    let vk_graphics_pipeline = GraphicsPipeline::start()
      .vertex_input_state(BuffersDefinition::new().vertex::<Vertex>())
      .vertex_shader(vs.entry_point("main").unwrap(), ())
      .input_assembly_state(InputAssemblyState::new())
      .viewport_state(ViewportState::viewport_fixed_scissor_irrelevant([
        viewport.clone()
      ]))
      .fragment_shader(fs.entry_point("main").unwrap(), ())
      .render_pass(Subpass::from(render_pass.clone(), 0).unwrap())
      .build(device.vk_logical_device.clone())
      .unwrap();

    return Arc::new(Self {
      vk_graphics_pipeline,
    });
  }

  fn load_shaders(device: &MdrDevice) -> (Arc<ShaderModule>, Arc<ShaderModule>) {
    let vertex_shader_module =
      vertex_shader::load(device.vk_logical_device.clone()).expect("Failed to load vertex shader");
    let fragment_shader_module = fragment_shader::load(device.vk_logical_device.clone())
      .expect("Failed to load fragment shader");

    return (vertex_shader_module, fragment_shader_module);
  }

  // TODO
  pub fn regenerate_graphics_pipeline(&self) {}
}
