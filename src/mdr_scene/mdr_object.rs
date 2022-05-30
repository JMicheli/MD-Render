use std::sync::Arc;

use cgmath::Matrix4;

use super::mdr_mesh::MdrMesh;

pub struct MdrObject {
  mesh: Arc<MdrMesh>,
  transform: Matrix4<f32>,
}

impl MdrObject {
  pub fn new(mesh: Arc<MdrMesh>, transform: Matrix4<f32>) -> Arc<Self> {
    Arc::new(Self { mesh, transform })
  }
}
