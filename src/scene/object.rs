use crate::graphics::resources::{MdrMaterial, MdrMesh};

use super::transform::MdrTransform;

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
