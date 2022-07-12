use std::sync::Arc;

use image::{io::Reader, Rgb, RgbImage, Rgba, RgbaImage};
use vulkano::{
  command_buffer::{CommandBufferExecFuture, PrimaryAutoCommandBuffer},
  device::Queue,
  format::Format,
  image::{view::ImageView, ImageDimensions, ImmutableImage, MipmapsCount},
  sampler::Sampler,
  sync::NowFuture,
};

pub struct MdrImage {
  image_data: Vec<u8>,
  image_height: u32,
  image_width: u32,
  format: MdrImageFormat,

  pub(crate) image_view: Option<Arc<ImageView<ImmutableImage>>>,
  pub(crate) sampler: Option<Arc<Sampler>>,
  pub(crate) has_image_view: bool,
}

impl MdrImage {
  pub fn from_file(path: &str, format: MdrImageFormat) -> Self {
    let image = Reader::open(path).unwrap().decode().unwrap();

    match format {
      MdrImageFormat::RGB | MdrImageFormat::SRGB => Self {
        image_data: image.as_rgb8().unwrap().as_raw().to_owned(),
        image_height: image.height(),
        image_width: image.width(),
        format,

        image_view: None,
        sampler: None,
        has_image_view: false,
      },
      MdrImageFormat::RGBA | MdrImageFormat::SRGBA => Self {
        image_data: image.as_rgba8().unwrap().as_raw().to_owned(),
        image_height: image.height(),
        image_width: image.width(),
        format,

        image_view: None,
        sampler: None,
        has_image_view: false,
      },
    }
  }

  pub fn solid_rgba(r: u8, g: u8, b: u8, a: u8) -> Self {
    let image = RgbaImage::from_pixel(1, 1, Rgba::from([r, g, b, a]));
    let image_data = image.as_raw().to_owned();

    Self {
      image_data,
      image_height: 1,
      image_width: 1,
      format: MdrImageFormat::RGBA,

      image_view: None,
      sampler: None,
      has_image_view: false,
    }
  }

  pub fn solid_rgb(r: u8, g: u8, b: u8) -> Self {
    let image = RgbImage::from_pixel(1, 1, Rgb::from([r, g, b]));
    let image_data = image.as_raw().to_owned();

    Self {
      image_data,
      image_height: 1,
      image_width: 1,
      format: MdrImageFormat::RGB,

      image_view: None,
      sampler: None,
      has_image_view: false,
    }
  }

  pub fn upload_to_gpu(
    &mut self,
    queue: &Arc<Queue>,
    sampler: Arc<Sampler>,
  ) -> CommandBufferExecFuture<NowFuture, PrimaryAutoCommandBuffer> {
    let dimensions = ImageDimensions::Dim2d {
      width: self.image_width,
      height: self.image_height,
      array_layers: 1,
    };

    let (image, future) = ImmutableImage::from_iter(
      self.image_data.clone(),
      dimensions,
      MipmapsCount::One,
      self.format.into(),
      queue.clone(),
    )
    .unwrap();

    self.image_view = Some(ImageView::new_default(image).unwrap());
    self.sampler = Some(sampler);
    self.has_image_view = true;
    future
  }
}

#[derive(Copy, Clone)]
pub enum MdrImageFormat {
  SRGBA,
  RGBA,
  SRGB,
  RGB,
}

impl From<MdrImageFormat> for Format {
  fn from(format: MdrImageFormat) -> Self {
    match format {
      MdrImageFormat::SRGBA => Format::R8G8B8A8_SRGB,
      MdrImageFormat::SRGB => Format::R8G8B8_SRGB,
      MdrImageFormat::RGBA => Format::R8G8B8A8_SNORM,
      MdrImageFormat::RGB => Format::R8G8B8_SNORM,
    }
  }
}

#[cfg(test)]
mod test {
  use super::MdrImage;

  #[test]
  pub fn create_solid_rgba() {
    MdrImage::solid_rgba(255, 0, 255, 255);
  }

  #[test]
  pub fn create_solid_rgb() {
    MdrImage::solid_rgb(255, 0, 255);
  }
}
