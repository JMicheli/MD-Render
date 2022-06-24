use cgmath::{Quaternion, Vector3};

use super::{MdrMesh, Vertex};

pub struct MdrTransform {
  position: Vector3<f32>,
  rotation: Quaternion<f32>,
  scale: Vector3<f32>,
}

impl MdrTransform {
  pub fn new() -> Self {
    Self {
      position: Vector3::new(0.0, 0.0, 0.0),
      rotation: Quaternion::new(0.0, 0.0, 0.0, 0.0),
      scale: Vector3::new(1.0, 1.0, 1.0),
    }
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
