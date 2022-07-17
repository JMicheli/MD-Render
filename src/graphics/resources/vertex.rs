use bytemuck::{Pod, Zeroable};

#[repr(C)]
#[derive(Default, Copy, Clone, Zeroable, Pod)]
pub struct MdrVertex {
  pub a_position: [f32; 3],
  pub a_normal: [f32; 3],
  pub a_uv: [f32; 2],
}

vulkano::impl_vertex!(MdrVertex, a_position, a_normal, a_uv);
