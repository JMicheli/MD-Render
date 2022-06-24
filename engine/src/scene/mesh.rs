use bytemuck::{Pod, Zeroable};

#[repr(C)]
#[derive(Default, Copy, Clone, Zeroable, Pod)]
pub struct Vertex {
  pub position: [f32; 3],
  pub normal: [f32; 3],
  pub color: [f32; 4],
}

vulkano::impl_vertex!(Vertex, position, normal, color);

pub struct MdrMesh {
  pub vertices: Vec<Vertex>,
  pub indices: Vec<u32>,
}

impl MdrMesh {
  pub fn new() -> Self {
    Self {
      vertices: Vec::new(),
      indices: Vec::new(),
    }
  }
}
