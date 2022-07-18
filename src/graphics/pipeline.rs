use std::sync::Arc;

use vulkano::{
  device::Device,
  pipeline::{
    graphics::{
      depth_stencil::DepthStencilState,
      input_assembly::InputAssemblyState,
      rasterization::{CullMode, FrontFace, RasterizationState},
      vertex_input::BuffersDefinition,
      viewport::{Viewport, ViewportState},
    },
    GraphicsPipeline,
  },
  render_pass::{RenderPass, Subpass},
  shader::ShaderModule,
};

use super::resources::MdrVertex;

/// The pipeline used for mesh drawing.
pub struct MdrPipeline {
  pub graphics_pipeline: Arc<GraphicsPipeline>,
}

impl MdrPipeline {
  pub fn new(
    logical_device: &Arc<Device>,
    vertex_shader: &Arc<ShaderModule>,
    fragment_shader: &Arc<ShaderModule>,
    render_pass: &Arc<RenderPass>,
    viewport: &Viewport,
  ) -> Arc<Self> {
    let graphics_pipeline = GraphicsPipeline::start()
      // Define what vertex structure the pipeline will expect
      .vertex_input_state(BuffersDefinition::new().vertex::<MdrVertex>())
      // Link the vertex shader
      .vertex_shader(vertex_shader.entry_point("main").unwrap(), ())
      // Input assembly settings (we use the defaults)
      .input_assembly_state(InputAssemblyState::new())
      // Define the viewport to be used for this render
      .viewport_state(ViewportState::viewport_fixed_scissor_irrelevant([
        viewport.clone()
      ]))
      // Fixed functions of the rasterizer
      .rasterization_state(
        RasterizationState::new()
          // Clockwise-winding faces will be treated as front-facing
          .front_face(FrontFace::Clockwise)
          // We cull back-facing faces to avoid unnecessary fragment threads
          .cull_mode(CullMode::Back),
      )
      // Link the fragment shader
      .fragment_shader(fragment_shader.entry_point("main").unwrap(), ())
      // Settings for depth testing (to ensure correct ordering of fragments)
      .depth_stencil_state(DepthStencilState::simple_depth_test())
      // The render pass to use for this pipeline
      .render_pass(Subpass::from(render_pass.clone(), 0).unwrap())
      // Build and unwrap to get the pipeline object
      .build(logical_device.clone())
      .unwrap();

    Arc::new(Self { graphics_pipeline })
  }
}
