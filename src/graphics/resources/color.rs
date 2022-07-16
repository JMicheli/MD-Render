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

impl From<[f32; 3]> for MdrColor {
  fn from(rgb: [f32; 3]) -> Self {
    Self {
      r: rgb[0],
      g: rgb[1],
      b: rgb[2],
    }
  }
}
