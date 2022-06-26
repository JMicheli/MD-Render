use cgmath::Vector3;

pub struct MdrMaterial {
  pub diffuse_color: Vector3<f32>,
  pub alpha: f32,

  pub specular_color: Vector3<f32>,
  pub shininess: f32,
}

impl MdrMaterial {
  pub const fn red() -> Self {
    Self {
      diffuse_color: Vector3::new(0.8, 0.0, 0.0),
      alpha: 1.0,
      specular_color: Vector3::new(1.0, 1.0, 1.0),
      shininess: 20.0,
    }
  }

  pub const fn green() -> Self {
    Self {
      diffuse_color: Vector3::new(0.0, 0.8, 0.0),
      alpha: 1.0,
      specular_color: Vector3::new(1.0, 1.0, 1.0),
      shininess: 20.0,
    }
  }

  pub const fn blue() -> Self {
    Self {
      diffuse_color: Vector3::new(0.0, 0.0, 0.8),
      alpha: 1.0,
      specular_color: Vector3::new(1.0, 1.0, 1.0),
      shininess: 20.0,
    }
  }

  pub const fn grey() -> Self {
    Self {
      diffuse_color: Vector3::new(0.3, 0.3, 0.3),
      alpha: 1.0,
      specular_color: Vector3::new(1.0, 1.0, 1.0),
      shininess: 5.0,
    }
  }
}

impl Default for MdrMaterial {
  fn default() -> Self {
    Self {
      diffuse_color: Vector3::new(0.0, 0.8, 0.0),
      alpha: 1.0,
      specular_color: Vector3::new(1.0, 1.0, 1.0),
      shininess: 20.0,
    }
  }
}
