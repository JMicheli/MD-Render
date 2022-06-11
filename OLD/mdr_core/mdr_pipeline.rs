use std::sync::Arc;

use cgmath::{Matrix3, Matrix4, Point3, Vector3};
use vulkano::{
  buffer::{BufferUsage, CpuBufferPool},
  descriptor_set::{layout::DescriptorSetLayout, PersistentDescriptorSet, WriteDescriptorSet},
  pipeline::{
    graphics::{
      depth_stencil::DepthStencilState,
      input_assembly::InputAssemblyState,
      vertex_input::BuffersDefinition,
      viewport::{Viewport, ViewportState},
    },
    GraphicsPipeline, Pipeline,
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
    types_meta: {
      use bytemuck::{Pod, Zeroable};

      #[derive(Clone, Copy, Zeroable, Pod)]
    },
  }
}
mod fragment_shader {
  vulkano_shaders::shader! {
    ty: "fragment",
    path: "src/assets/shaders/basic.frag",
  }
}

use vertex_shader::ty::UniformBufferObject;

pub struct MdrPipeline {
  pub vk_graphics_pipeline: Arc<GraphicsPipeline>,
  vk_uniform_buffer_pool: CpuBufferPool<UniformBufferObject>,
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
      .depth_stencil_state(DepthStencilState::simple_depth_test())
      .render_pass(Subpass::from(render_pass.clone(), 0).unwrap())
      .build(device.vk_logical_device.clone())
      .unwrap();

    // Create buffer pool

    let vk_uniform_buffer_pool = CpuBufferPool::<UniformBufferObject>::new(
      device.vk_logical_device.clone(),
      BufferUsage::all(),
    );

    return Arc::new(Self {
      vk_graphics_pipeline,
      vk_uniform_buffer_pool,
    });
  }

  pub fn upload_descriptor_set(
    &self,
    aspect_ratio: f32,
    rotatation_angle: f32,
  ) -> Arc<PersistentDescriptorSet> {
    // Data, hard-coded for now
    let rotation = Matrix3::from_angle_y(cgmath::Rad(rotatation_angle));
    let proj = cgmath::perspective(
      cgmath::Rad(std::f32::consts::FRAC_PI_2),
      aspect_ratio,
      0.01,
      100.0,
    );
    let view = Matrix4::<f32>::look_at_rh(
      Point3::new(0.3, 0.3, 1.0),
      Point3::new(0.0, 0.0, 0.0),
      Vector3::new(0.0, 1.0, 0.0),
    );
    let scale = Matrix4::from_scale(0.5);
    let uniform_data = UniformBufferObject {
      model: Matrix4::from(rotation).into(),
      view: (view * scale).into(),
      proj: proj.into(),
    };
    let uniform_buffer = self.vk_uniform_buffer_pool.next(uniform_data).unwrap();

    let layout = self
      .vk_graphics_pipeline
      .layout()
      .set_layouts()
      .get(0)
      .unwrap();

    let set = PersistentDescriptorSet::new(
      layout.clone(),
      [WriteDescriptorSet::buffer(0, uniform_buffer)],
    )
    .unwrap();

    return set;
  }

  fn load_shaders(device: &MdrDevice) -> (Arc<ShaderModule>, Arc<ShaderModule>) {
    let vertex_shader_module =
      vertex_shader::load(device.vk_logical_device.clone()).expect("Failed to load vertex shader");
    let fragment_shader_module = fragment_shader::load(device.vk_logical_device.clone())
      .expect("Failed to load fragment shader");

    return (vertex_shader_module, fragment_shader_module);
  }
}
