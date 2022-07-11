use nalgebra::{Matrix4, Perspective3};

use super::transform::MdrTransform;

// TODO Reimplement as trait and split out ortho/perspective cameras
pub struct MdrCamera {
  pub transform: MdrTransform,

  pub field_of_view: f32,
  pub aspect_ratio: f32,
  pub near_plane: f32,
  pub far_plane: f32,
}

impl MdrCamera {
  pub fn get_view_matrix(&self) -> Matrix4<f32> {
    let translation_matrix = self.transform.translation.matrix();
    let rotation_matrix = self.transform.rotation.matrix();

    rotation_matrix * translation_matrix
  }

  pub fn get_projection_matrix(&self) -> Matrix4<f32> {
    Perspective3::new(
      self.aspect_ratio,
      self.field_of_view,
      self.near_plane,
      self.far_plane,
    )
    .to_homogeneous()
  }
}

impl Default for MdrCamera {
  fn default() -> Self {
    Self {
      transform: MdrTransform::identity(),

      field_of_view: std::f32::consts::FRAC_PI_2,
      near_plane: 0.01,
      aspect_ratio: 1.0,
      far_plane: 1000.0,
    }
  }
}
