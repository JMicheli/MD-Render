use bytemuck::{Pod, Zeroable};

#[repr(C)]
#[derive(Default, Copy, Clone, Zeroable, Pod)]
pub struct MdrVertex_pos {
  pub a_position: [f32; 3],
}
vulkano::impl_vertex!(MdrVertex_pos, a_position);

#[repr(C)]
#[derive(Default, Copy, Clone, Zeroable, Pod)]
pub struct MdrVertex_norm {
  pub a_normal: [f32; 3],
}
vulkano::impl_vertex!(MdrVertex_norm, a_normal);

#[repr(C)]
#[derive(Default, Copy, Clone, Zeroable, Pod)]
pub struct MdrVertex_uv {
  pub a_uv: [f32; 2],
}
vulkano::impl_vertex!(MdrVertex_uv, a_uv);

#[repr(C)]
#[derive(Default, Copy, Clone, Zeroable, Pod)]
pub struct MdrVertex_tan {
  pub a_tangent: [f32; 3],
}
vulkano::impl_vertex!(MdrVertex_tan, a_tangent);
