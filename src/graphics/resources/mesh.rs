use std::sync::Arc;

use vulkano::{buffer::cpu_pool::CpuBufferPoolChunk, memory::pool::StdMemoryPool};

use super::MdrVertex;

#[derive(Default)]
pub struct MdrMeshData {
  pub vertices: Vec<MdrVertex>,
  pub indices: Vec<u32>,
}

#[derive(Debug)]
pub struct MdrMesh {
  pub name: String,
}

pub struct MdrGpuMeshHandle {
  pub(crate) vertex_subbuffer: Arc<CpuBufferPoolChunk<MdrVertex, Arc<StdMemoryPool>>>,
  pub(crate) index_subbuffer: Arc<CpuBufferPoolChunk<u32, Arc<StdMemoryPool>>>,
  pub(crate) index_count: u32,
}
