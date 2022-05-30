pub mod mdr_camera;
pub mod mdr_mesh;
pub mod mdr_object;

use std::sync::Arc;

use mdr_camera::MdrCamera;
use mdr_object::MdrObject;

pub struct MdrScene {
  objects: Vec<Arc<MdrObject>>,
  camera: MdrCamera,
}

impl MdrScene {
  pub fn new(objects: Vec<Arc<MdrObject>>) -> Self {
    Self {
      objects: vec![],
      camera: MdrCamera::new(),
    }
  }
}
