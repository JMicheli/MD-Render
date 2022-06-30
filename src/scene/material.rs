pub struct MdrMaterial {
  pub diffuse_color: Color,
  pub alpha: f32,

  pub specular_color: Color,
  pub shininess: f32,
}

impl MdrMaterial {
  pub const fn red() -> Self {
    Self {
      diffuse_color: Color::new(0.8, 0.0, 0.0),
      alpha: 1.0,
      specular_color: Color::white(),
      shininess: 20.0,
    }
  }

  pub const fn green() -> Self {
    Self {
      diffuse_color: Color::new(0.0, 0.8, 0.0),
      alpha: 1.0,
      specular_color: Color::white(),
      shininess: 20.0,
    }
  }

  pub const fn blue() -> Self {
    Self {
      diffuse_color: Color::new(0.0, 0.0, 0.8),
      alpha: 1.0,
      specular_color: Color::white(),
      shininess: 20.0,
    }
  }

  pub const fn grey() -> Self {
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
