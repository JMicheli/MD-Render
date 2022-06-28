use cgmath::{InnerSpace, Matrix4, Point3, Rad, Vector3};

pub struct MdrCamera {
  pub pos: Vector3<f32>,
  pub radius: f32,
  pub theta: f32,
  pub phi: f32,

  pub field_of_view: f32,
  pub near_plane: f32,
  pub far_plane: f32,
  pub scale: f32,
}

impl MdrCamera {
  pub fn get_scene_position(&self) -> Vector3<f32> {
    let theta_rad = Rad(self.theta).0;
    let phi_rad = Rad(self.phi).0;

    let x = self.pos.x + self.radius * f32::sin(phi_rad) * f32::cos(theta_rad);
    let y = self.pos.z + self.radius * f32::cos(phi_rad);
    let z = self.pos.y + self.radius * f32::sin(phi_rad) * f32::sin(theta_rad);

    Vector3::new(x, y, z)
  }

  pub fn normalized_position(&self) -> Vector3<f32> {
    let position = self.get_scene_position();
    let norm_position = position.magnitude();

    (1.0 / norm_position) * position
  }

  pub fn get_view_proj(&self, aspect_ratio: f32) -> (Matrix4<f32>, Matrix4<f32>) {
    let view = {
      let position = self.normalized_position();

      let eye_position = Point3::new(position[0], position[1], position[2]);
      let center_position = Point3::new(self.pos.x, self.pos.y, self.pos.z);

      let look_at =
        Matrix4::<f32>::look_at_rh(eye_position, center_position, Vector3::new(0.0, 1.0, 0.0));
      let scale = Matrix4::<f32>::from_scale(self.scale);

      look_at * scale
    };

    let projection = cgmath::perspective(
      Rad(self.field_of_view),
      aspect_ratio,
      self.near_plane,
      self.far_plane,
    );

    (view, projection)
  }
}

impl Default for MdrCamera {
  fn default() -> Self {
    Self {
      pos: Vector3::new(0.0, 0.0, 0.0),
      radius: 1.0,
      theta: 0.0,
      phi: 90.0,

      field_of_view: std::f32::consts::FRAC_PI_2,
      near_plane: 0.01,
      far_plane: 100.0,
      scale: 0.5,
    }
  }
}
