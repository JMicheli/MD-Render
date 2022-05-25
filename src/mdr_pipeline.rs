use bytemuck::{Pod, Zeroable};
use std::sync::Arc;

use vulkano::{
  device::Device,
  pipeline::{
    graphics::{
      input_assembly::InputAssemblyState,
      vertex_input::BuffersDefinition,
      viewport::{Viewport, ViewportState},
    },
    GraphicsPipeline,
  },
  render_pass::{RenderPass, Subpass},
  shader::{ShaderCreationError, ShaderModule},
};

use crate::mdr_device::MdrDevice;

mod vertex_shader {
  vulkano_shaders::shader! {
    ty: "vertex",
    path: "src/shaders/basic.vert",
  }
}
mod fragment_shader {
  vulkano_shaders::shader! {
    ty: "fragment",
    path: "src/shaders/basic.frag",
  }
}

#[repr(C)]
#[derive(Default, Copy, Clone, Zeroable, Pod)]
pub struct Vertex {
  position: [f32; 2],
}

vulkano::impl_vertex!(Vertex, position);

pub struct MdrPipeline {
  logical_device: Arc<Device>,
  vertex_shader: Arc<ShaderModule>,
  fragment_shader: Arc<ShaderModule>,

  graphics_pipeline: Arc<GraphicsPipeline>,
}

impl MdrPipeline {
  pub fn new(device: &MdrDevice, render_pass: Arc<RenderPass>, viewport: Viewport) -> Self {
    // Load shaders
    let (vs, fs) = Self::load_shaders(device);
    // Get graphics pipeline
    let graphics_pipeline = Self::get_graphics_pipeline(
      device.logical_device(),
      vs.clone(),
      fs.clone(),
      render_pass,
      viewport,
    );

    Self {
      logical_device: device.logical_device(),
      vertex_shader: vs,
      fragment_shader: fs,
      graphics_pipeline,
    }
  }

  fn load_shaders(device: &MdrDevice) -> (Arc<ShaderModule>, Arc<ShaderModule>) {
    let vertex_shader_module =
      vertex_shader::load(device.logical_device()).expect("Failed to load vertex shader");
    let fragment_shader_module =
      fragment_shader::load(device.logical_device()).expect("Failed to load fragment shader");

    return (vertex_shader_module, fragment_shader_module);
  }

  fn get_graphics_pipeline(
    device: Arc<Device>,
    vs: Arc<ShaderModule>,
    fs: Arc<ShaderModule>,
    render_pass: Arc<RenderPass>,
    viewport: Viewport,
  ) -> Arc<GraphicsPipeline> {
    GraphicsPipeline::start()
      .vertex_input_state(BuffersDefinition::new().vertex::<Vertex>())
      .vertex_shader(vs.entry_point("main").unwrap(), ())
      .input_assembly_state(InputAssemblyState::new())
      .viewport_state(ViewportState::viewport_fixed_scissor_irrelevant([viewport]))
      .fragment_shader(fs.entry_point("main").unwrap(), ())
      .render_pass(Subpass::from(render_pass.clone(), 0).unwrap())
      .build(device.clone())
      .unwrap()
  }
}
