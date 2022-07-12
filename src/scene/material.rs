use crate::graphics_context::image::MdrImage;

pub struct MdrMaterial {
  pub diffuse_map: MdrImage,

  pub shininess: f32,
}

impl MdrMaterial {
  pub fn red() -> Self {
    Self {
      diffuse_map: MdrImage::solid_rgba(205, 0, 0, 255),
      shininess: 20.0,
    }
  }

  pub fn green() -> Self {
    Self {
      diffuse_map: MdrImage::solid_rgba(0, 205, 0, 255),
      shininess: 20.0,
    }
  }

  pub fn blue() -> Self {
    Self {
      diffuse_map: MdrImage::solid_rgba(0, 0, 205, 255),
      shininess: 20.0,
    }
  }

  pub fn grey() -> Self {
    Self {
      diffuse_map: MdrImage::solid_rgba(77, 77, 77, 255),
      shininess: 5.0,
    }
  }
}

impl Default for MdrMaterial {
  fn default() -> Self {
    Self {
      diffuse_map: MdrImage::solid_rgba(255, 0, 255, 255),
      shininess: 20.0,
    }
  }
}
