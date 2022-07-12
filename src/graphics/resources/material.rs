use std::sync::Arc;

use vulkano::{
  buffer::{BufferUsage, CpuAccessibleBuffer},
  device::Device,
};

use crate::graphics::shaders::basic_vertex_shader::ty::MaterialUniformData;

pub struct MdrMaterial {
  pub diffuse_color: Color,
  pub alpha: f32,

  pub specular_color: Color,
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

  pub fn red() -> Self {
    Self {
      diffuse_color: Color::new(0.8, 0.0, 0.0),
      alpha: 1.0,
      specular_color: Color::white(),
      shininess: 20.0,
    }
  }

  pub fn green() -> Self {
    Self {
      diffuse_color: Color::new(0.0, 0.8, 0.0),
      alpha: 1.0,
      specular_color: Color::white(),
      shininess: 20.0,
    }
  }

  pub fn blue() -> Self {
    Self {
      diffuse_color: Color::new(0.0, 0.0, 0.8),
      alpha: 1.0,
      specular_color: Color::white(),
      shininess: 20.0,
    }
  }

  pub fn grey() -> Self {
    Self {
      diffuse_color: Color::new(0.3, 0.3, 0.3),
      alpha: 1.0,
      specular_color: Color::white(),
      shininess: 5.0,
    }
  }
}

impl Default for MdrMaterial {
  fn default() -> Self {
    Self {
      diffuse_color: Color::new(0.0, 0.8, 0.0),
      alpha: 1.0,
      specular_color: Color::white(),
      shininess: 20.0,
    }
  }
}

#[derive(Copy, Clone)]
pub struct Color {
  pub r: f32,
  pub g: f32,
  pub b: f32,
}

impl Color {
  pub fn new(r: f32, g: f32, b: f32) -> Self {
    Self { r, g, b }
  }

  pub const fn white() -> Self {
    Self {
      r: 1.0,
      g: 1.0,
      b: 1.0,
    }
  }

  pub const fn red() -> Self {
    Self {
      r: 1.0,
      g: 0.0,
      b: 0.0,
    }
  }

  pub const fn green() -> Self {
    Self {
      r: 0.0,
      g: 1.0,
      b: 0.0,
    }
  }

  pub const fn blue() -> Self {
    Self {
      r: 0.0,
      g: 0.0,
      b: 1.0,
    }
  }

  pub const fn black() -> Self {
    Self {
      r: 0.0,
      g: 0.0,
      b: 0.0,
    }
  }
}

impl From<Color> for [f32; 3] {
  fn from(color: Color) -> Self {
    [color.r, color.g, color.b]
  }
}

pub struct MdrMaterialBuffer {
  pub material_data: Arc<CpuAccessibleBuffer<MaterialUniformData>>,
}
