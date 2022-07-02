use nalgebra::{Matrix4, Rotation3, Scale3, Translation3};

use super::{material::MdrMaterial, MdrMesh, Vertex};

pub struct MdrTransform {
  pub translation: Translation3<f32>,
  pub rotation: Rotation3<f32>,
  pub scale: Scale3<f32>,
}

impl MdrTransform {
  pub fn matrix(&self) -> Matrix4<f32> {
    let trans = self.translation.to_homogeneous();
    let rot = self.rotation.to_homogeneous();
    let scale = self.scale.to_homogeneous();

    trans * rot * scale
  }

  pub fn inverse_matrix(&self) -> Matrix4<f32> {
    let trans_inv = self.translation.inverse().to_homogeneous();
    let rot_inv = self.rotation.inverse().to_homogeneous();
    let scale_inv = self.scale.try_inverse().unwrap().to_homogeneous();

    scale_inv * rot_inv * trans_inv
  }

  pub fn identity() -> Self {
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
      transform: MdrTransform::identity(),
      material: MdrMaterial::default(),
    }
  }

  pub fn empty() -> Self {
    Self {
      mesh: MdrMesh::default(),
      transform: MdrTransform::identity(),
      material: MdrMaterial::default(),
    }
  }

  pub fn from_obj(file_path: &str) -> Self {
    Self::new(MdrMesh::load_obj(file_path))
  }
}
