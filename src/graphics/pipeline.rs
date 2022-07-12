use std::sync::Arc;

use vulkano::{
  device::Device,
  pipeline::{
    graphics::{
      depth_stencil::DepthStencilState,
      input_assembly::InputAssemblyState,
      vertex_input::BuffersDefinition,
      viewport::{Viewport, ViewportState},
    },
    GraphicsPipeline,
  },
  render_pass::{RenderPass, Subpass},
  shader::ShaderModule,
};

use super::resources::MdrVertex;

pub struct MdrPipeline {
  pub graphics_pipeline: Arc<GraphicsPipeline>,
}

impl MdrPipeline {
  pub fn new(
    logical_device: &Arc<Device>,
    vs: &Arc<ShaderModule>,
    fs: &Arc<ShaderModule>,
    render_pass: &Arc<RenderPass>,
    viewport: &Viewport,
  ) -> Arc<Self> {
    let graphics_pipeline = GraphicsPipeline::start()
      .vertex_input_state(BuffersDefinition::new().vertex::<MdrVertex>())
      .vertex_shader(vs.entry_point("main").unwrap(), ())
      .input_assembly_state(InputAssemblyState::new())
      .viewport_state(ViewportState::viewport_fixed_scissor_irrelevant([
        viewport.clone()
      ]))
      .fragment_shader(fs.entry_point("main").unwrap(), ())
      .depth_stencil_state(DepthStencilState::simple_depth_test())
      .render_pass(Subpass::from(render_pass.clone(), 0).unwrap())
      .build(logical_device.clone())
      .unwrap();

    Arc::new(Self { graphics_pipeline })
  }
}
