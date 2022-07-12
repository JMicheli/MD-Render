use std::sync::Arc;

use vulkano::{
  device::Device,
  pipeline::{
    graphics::{
      color_blend::ColorBlendState,
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

use crate::scene::Vertex;

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
    let subpass = Subpass::from(render_pass.clone(), 0).unwrap();
    let graphics_pipeline = GraphicsPipeline::start()
      .vertex_input_state(BuffersDefinition::new().vertex::<Vertex>())
      .vertex_shader(vs.entry_point("main").unwrap(), ())
      .input_assembly_state(InputAssemblyState::new())
      .viewport_state(ViewportState::viewport_fixed_scissor_irrelevant([
        viewport.clone()
      ]))
      .fragment_shader(fs.entry_point("main").unwrap(), ())
      .depth_stencil_state(DepthStencilState::simple_depth_test())
      //TODO Fix blending
      .color_blend_state(ColorBlendState::new(subpass.num_color_attachments()).blend_alpha())
      .render_pass(subpass)
      .build(logical_device.clone())
      .unwrap();

    Arc::new(Self { graphics_pipeline })
  }
}
