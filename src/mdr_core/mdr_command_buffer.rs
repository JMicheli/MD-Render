use std::sync::Arc;

use vulkano::{
  buffer::TypedBufferAccess,
  command_buffer::{
    AutoCommandBufferBuilder, CommandBufferUsage, PrimaryAutoCommandBuffer, SubpassContents,
  },
  render_pass::Framebuffer,
};

use crate::mdr_scene::mdr_mesh::MdrMesh;

use super::{mdr_device::MdrDevice, mdr_pipeline::MdrPipeline};

pub struct MdrCommandBuffer {
  pub vk_cmd_buffers: Vec<Arc<PrimaryAutoCommandBuffer>>,
}

impl MdrCommandBuffer {
  pub fn new(
    device: &Arc<MdrDevice>,
    pipeline: &Arc<MdrPipeline>,
    framebuffers: &Vec<Arc<Framebuffer>>,
    mesh: &MdrMesh,
  ) -> Self {
    // Generate command buffers
    let vk_cmd_buffers: Vec<Arc<PrimaryAutoCommandBuffer>> = framebuffers
      .iter()
      .map(|framebuffer| {
        let mut builder = AutoCommandBufferBuilder::primary(
          device.vk_logical_device.clone(),
          device.queue_family(),
          CommandBufferUsage::MultipleSubmit,
        )
        .unwrap();

        // Clear color used when drawing bacground
        let clear_color = vec![[0.1, 0.1, 0.1, 1.0].into(), 1f32.into()];

        builder
          .begin_render_pass(framebuffer.clone(), SubpassContents::Inline, clear_color)
          .unwrap()
          .bind_pipeline_graphics(pipeline.vk_graphics_pipeline.clone())
          .bind_vertex_buffers(0, mesh.vertex_buffer.clone())
          .bind_index_buffer(mesh.index_buffer.clone())
          .draw_indexed(mesh.index_buffer.len() as u32, 1, 0, 0, 0)
          .unwrap()
          .end_render_pass()
          .unwrap();

        Arc::new(builder.build().unwrap())
      })
      .collect();

    Self { vk_cmd_buffers }
  }

  pub fn get_primary(&self, index: usize) -> Arc<PrimaryAutoCommandBuffer> {
    return self.vk_cmd_buffers[index].clone();
  }
}
