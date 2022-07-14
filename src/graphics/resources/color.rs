use super::MdrMaterial;

#[derive(Copy, Clone)]
pub struct MdrColor {
  pub r: f32,
  pub g: f32,
  pub b: f32,
}

impl MdrColor {
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

impl From<MdrColor> for [f32; 3] {
  fn from(color: MdrColor) -> Self {
    [color.r, color.g, color.b]
  }
}

impl MdrMaterial {
  pub fn red() -> Self {
    Self {
      diffuse_color: MdrColor {
        r: 0.8,
        g: 0.0,
        b: 0.0,
      },
      alpha: 1.0,
      specular_color: MdrColor::white(),
      shininess: 20.0,
    }
  }

  pub fn green() -> Self {
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

  pub fn blue() -> Self {
    Self {
      diffuse_color: MdrColor {
        r: 0.0,
        g: 0.0,
        b: 0.8,
      },
      alpha: 1.0,
      specular_color: MdrColor::white(),
      shininess: 20.0,
    }
  }

  pub fn grey() -> Self {
    Self {
      diffuse_color: MdrColor {
        r: 0.3,
        g: 0.3,
        b: 0.3,
      },
      alpha: 1.0,
      specular_color: MdrColor::white(),
      shininess: 5.0,
    }
  }
}
