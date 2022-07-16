use std::sync::Arc;

use vulkano::{buffer::cpu_pool::CpuBufferPoolChunk, memory::pool::StdMemoryPool};

use super::color::MdrColor;

pub use crate::graphics::shaders::basic_fragment_shader::ty::MdrMaterialUniformData;

#[derive(Debug)]
pub struct MdrMaterial {
  pub name: String,
}

pub struct MdrMaterialCreateInfo {
  pub diffuse_color: MdrColor,
  pub alpha: f32,

  pub specular_color: MdrColor,
  pub shininess: f32,
}

pub struct MdrGpuMaterialHandle {
  pub(crate) material_chunk: Arc<CpuBufferPoolChunk<MdrMaterialUniformData, Arc<StdMemoryPool>>>,
}
