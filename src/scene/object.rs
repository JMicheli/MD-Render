use nalgebra::{Matrix4, Rotation3, Scale3, Translation3};

use super::{material::MdrMaterial, MdrMesh, Vertex};

pub struct MdrTransform {
  pub translation: Translation3<f32>,
  pub rotation: Rotation3<f32>,
  pub scale: Scale3<f32>,
}

impl MdrTransform {
  pub fn get_matrix_and_inverse(&self) -> (Matrix4<f32>, Matrix4<f32>) {
    // Translation homogenous matrices
    let trans = self.translation.to_homogeneous();
    let trans_inv = self.translation.inverse().to_homogeneous();
    // Rotation homogenous matrices
    let rot = self.rotation.to_homogeneous();
    let rot_inv = self.rotation.inverse().to_homogeneous();
    // Scale homogenous matrices
    let scale = self.scale.to_homogeneous();
    let scale_inv = self.scale.try_inverse().unwrap().to_homogeneous();

    let transform_matrix = trans * rot * scale;
    let inverse_transform_matrix = scale_inv * rot_inv * trans_inv;

    (transform_matrix, inverse_transform_matrix)
  }
}

impl Default for MdrTransform {
  fn default() -> Self {
    Self {
      translation: Translation3::identity(),
      rotation: Rotation3::identity(),
      scale: Scale3::identity(),
    }
  }
}

pub struct MdrSceneObject {
  pub mesh: MdrMesh,
  pub transform: MdrTransform,
  pub material: MdrMaterial,
}

impl MdrSceneObject {
  pub fn new(mesh: MdrMesh) -> Self {
    Self {
      mesh,
      transform: MdrTransform::default(),
      material: MdrMaterial::default(),
    }
  }

  pub fn empty() -> Self {
    Self {
      mesh: MdrMesh::default(),
      transform: MdrTransform::default(),
      material: MdrMaterial::default(),
    }
  }

  pub fn from_obj(file_path: &str) -> Self {
    Self::new(MdrMesh::load_obj(file_path))
  }
}
