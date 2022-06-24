use cgmath::{Matrix3, Matrix4, Point3, Rad, Vector3};

pub struct MdrCamera {
  rotation_angle: f32,

  field_of_view: f32,
  near_plane: f32,
  far_plane: f32,
}

impl MdrCamera {
  pub fn new() -> Self {
    Self {
      rotation_angle: 0.0,

      field_of_view: std::f32::consts::FRAC_PI_2,
      near_plane: 0.01,
      far_plane: 100.0,
    }
  }

  pub fn as_wvp(&self, aspect_ratio: f32) -> (Matrix4<f32>, Matrix4<f32>, Matrix4<f32>) {
    let rotation = Matrix3::from_angle_y(Rad(self.rotation_angle));
    let world = Matrix4::from(rotation);

    let view = {
      let look_at = Matrix4::<f32>::look_at_rh(
        Point3::new(0.3, 0.3, 1.0),
        Point3::new(0.0, 0.0, 0.0),
        Vector3::new(0.0, 1.0, 0.0),
      );
      let scale = Matrix4::<f32>::from_scale(0.5);

      look_at * scale
    };

    let projection = cgmath::perspective(
      Rad(self.field_of_view),
      aspect_ratio,
      self.near_plane,
      self.far_plane,
    );

    (world, view, projection)
  }
}
