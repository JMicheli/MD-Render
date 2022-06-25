use cgmath::{Matrix4, Rad, Vector3};

use super::{MdrMesh, Vertex};

pub struct MdrTransform {
  pub position: Vector3<f32>,
  pub rotation: Vector3<f32>,
  pub scale: Vector3<f32>,
}

impl MdrTransform {
  pub fn new() -> Self {
    Self {
      position: Vector3::new(0.0, 0.0, 0.0),
      rotation: Vector3::new(0.0, 0.0, 0.0),
      scale: Vector3::new(1.0, 1.0, 1.0),
    }
  }

  pub fn to_matrix(&self) -> Matrix4<f32> {
    let translate = Matrix4::from_translation(self.position);
    let rotate = Matrix4::from_angle_x(Rad(self.rotation.x))
      * Matrix4::from_angle_y(Rad(self.rotation.y))
      * Matrix4::from_angle_z(Rad(self.rotation.z));
    let scale = Matrix4::from_nonuniform_scale(self.scale.x, self.scale.y, self.scale.z);

    scale * rotate * translate
  }
}

pub struct MdrSceneObject {
  pub mesh: MdrMesh,
  pub transform: MdrTransform,
}

impl MdrSceneObject {
  pub fn new(mesh: MdrMesh) -> Self {
    Self {
      mesh,
      transform: MdrTransform::new(),
    }
  }

  pub fn empty() -> Self {
    Self {
      mesh: MdrMesh::new(),
      transform: MdrTransform::new(),
    }
  }

  pub fn from_obj(file_path: &str) -> Self {
    let mut new_object = Self::empty();
    new_object.mesh = MdrMesh::load_obj(file_path);

    new_object
  }

  pub fn test_triangle() -> Self {
    Self {
      mesh: MdrMesh {
        vertices: vec![
          Vertex {
            position: [-0.5, 0.5, 0.0],
            normal: [0.0, 0.0, 0.0],
            color: [0.0, 0.0, 1.0, 1.0],
          },
          Vertex {
            position: [0.5, 0.5, 0.0],
            normal: [0.0, 0.0, 0.0],
            color: [0.0, 1.0, 0.0, 1.0],
          },
          Vertex {
            position: [0.0, -0.5, 0.0],
            normal: [0.0, 0.0, 0.0],
            color: [1.0, 0.0, 0.0, 1.0],
          },
        ],
        indices: vec![0, 1, 2],
      },
      transform: MdrTransform::new(),
    }
  }
}
