use std::sync::Arc;

use cgmath::Matrix4;
use vulkano::{
  buffer::CpuAccessibleBuffer,
  memory::pool::{PotentialDedicatedAllocation, StdMemoryPoolAlloc},
};

use super::mdr_mesh::{MdrMesh, Vertex};

pub struct MdrObject {
  mesh: Arc<MdrMesh>,
  transform: Matrix4<f32>,
}

impl MdrObject {
  pub fn new(mesh: Arc<MdrMesh>, transform: Matrix4<f32>) -> Arc<Self> {
    Arc::new(Self { mesh, transform })
  }

  pub fn get_vertex_buffer(
    &self,
  ) -> &Arc<CpuAccessibleBuffer<[Vertex], PotentialDedicatedAllocation<StdMemoryPoolAlloc>>> {
    return &self.mesh.vertex_buffer;
  }

  pub fn get_index_buffer(
    &self,
  ) -> &Arc<CpuAccessibleBuffer<[u32], PotentialDedicatedAllocation<StdMemoryPoolAlloc>>> {
    return &self.mesh.index_buffer;
  }
}
