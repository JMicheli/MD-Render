use std::sync::Arc;

use vulkano::{
  buffer::{BufferUsage, CpuAccessibleBuffer},
  device::Device,
};

use super::color::MdrColor;
use crate::graphics::shaders::basic_fragment_shader::ty::MaterialUniformData;

/// Represents a material resource to be sent to the GPU.
pub struct MdrMaterial {
  pub diffuse_color: MdrColor,
  pub alpha: f32,

  pub specular_color: MdrColor,
  pub shininess: f32,
}

impl MdrMaterial {
  pub fn upload_to_gpu(&self, logical_device: &Arc<Device>) -> MdrMaterialBuffer {
    let material_data = CpuAccessibleBuffer::from_data(
      logical_device.clone(),
      BufferUsage::uniform_buffer(),
      false,
      MaterialUniformData {
        diffuse_color: self.diffuse_color.into(),
        alpha: self.alpha,

        specular_color: self.specular_color.into(),
        shininess: self.shininess,
      },
    )
    .unwrap();

    MdrMaterialBuffer { material_data }
  }
}

impl Default for MdrMaterial {
  fn default() -> Self {
    Self {
      diffuse_color: MdrColor {
        r: 0.0,
        g: 0.8,
        b: 0.0,
      },
      alpha: 1.0,
      specular_color: MdrColor::white(),
      shininess: 20.0,
    }
  }
}

pub struct MdrMaterialBuffer {
  pub material_data: Arc<CpuAccessibleBuffer<MaterialUniformData>>,
}
