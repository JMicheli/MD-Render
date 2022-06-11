use std::sync::Arc;

use vulkano::{
  buffer::TypedBufferAccess,
  command_buffer::{
    AutoCommandBufferBuilder, CommandBufferUsage, PrimaryAutoCommandBuffer, SubpassContents,
  },
  descriptor_set::PersistentDescriptorSet,
  pipeline::{Pipeline, PipelineBindPoint},
  render_pass::Framebuffer,
};

use crate::mdr_scene::MdrScene;

use super::{mdr_device::MdrDevice, mdr_pipeline::MdrPipeline};

pub struct MdrCommandBuffer {
  pub vk_cmd_buffers: Vec<Arc<PrimaryAutoCommandBuffer>>,
}

impl MdrCommandBuffer {
  pub fn new(
    device: &Arc<MdrDevice>,
    pipeline: &Arc<MdrPipeline>,
    framebuffers: &Vec<Arc<Framebuffer>>,
    scene: &MdrScene,
    set: Arc<PersistentDescriptorSet>,
  ) -> Self {
    // Generate command buffers
    let vk_cmd_buffers: Vec<Arc<PrimaryAutoCommandBuffer>> = framebuffers
      .iter()
      .map(|framebuffer| {
        let mut builder = AutoCommandBufferBuilder::primary(
          device.vk_logical_device.clone(),
          device.queue_family(),
          CommandBufferUsage::OneTimeSubmit,
        )
        .unwrap();

        // Clear color used when drawing bacground
        let clear_color = vec![[0.1, 0.1, 0.1, 1.0].into(), 1f32.into()];

        // Initial builder setup
        builder
          .begin_render_pass(framebuffer.clone(), SubpassContents::Inline, clear_color)
          .unwrap()
          .bind_pipeline_graphics(pipeline.vk_graphics_pipeline.clone())
          .bind_descriptor_sets(
            PipelineBindPoint::Graphics,
            pipeline.vk_graphics_pipeline.layout().clone(),
            0,
            set.clone(),
          );

        // Loop over scene objects and draw them
        for render_obj in scene.get_render_objects() {
          let vertex_buffer = render_obj.get_vertex_buffer();
          let index_buffer = render_obj.get_vertex_buffer();
          let index_count = index_buffer.len() as u32;

          builder
            .bind_vertex_buffers(0, vertex_buffer.clone())
            .bind_index_buffer(index_buffer.clone())
            .draw_indexed(index_count, 1, 0, 0, 0)
            .unwrap();
        }

        // End render pass
        builder.end_render_pass().unwrap();

        Arc::new(builder.build().unwrap())
      })
      .collect();

    Self { vk_cmd_buffers }
  }

  pub fn get_primary(&self, index: usize) -> Arc<PrimaryAutoCommandBuffer> {
    return self.vk_cmd_buffers[index].clone();
  }
}
