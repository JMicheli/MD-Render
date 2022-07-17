#[derive(Copy, Clone)]
pub struct MdrRgb {
  pub r: f32,
  pub g: f32,
  pub b: f32,
}

impl MdrRgb {
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

impl From<MdrRgb> for [f32; 3] {
  fn from(color: MdrRgb) -> Self {
    [color.r, color.g, color.b]
  }
}

impl From<[f32; 3]> for MdrRgb {
  fn from(rgb: [f32; 3]) -> Self {
    Self {
      r: rgb[0],
      g: rgb[1],
      b: rgb[2],
    }
  }
}

pub struct MdrRgba {
  pub r: f32,
  pub g: f32,
  pub b: f32,
  pub a: f32,
}

impl From<MdrRgba> for [f32; 4] {
  fn from(color: MdrRgba) -> Self {
    [color.r, color.g, color.b, color.a]
  }
}

impl From<[f32; 4]> for MdrRgba {
  fn from(rgba: [f32; 4]) -> Self {
    Self {
      r: rgba[0],
      g: rgba[1],
      b: rgba[2],
      a: rgba[3],
    }
  }
}

/// How the GPU will interpret a color value
pub enum MdrColorType {
  /// A Standardized RGBA color, with pre-gamma RGB values and an alpha channel.
  SRGBA,

  /// A Standardized RGB color, with pre-gamma RGB values.
  SRGB,

  /// Raw RGB value data not intended to be directly rendered to a viewer.
  NonColorData,
}
