use std::sync::Arc;

use super::{camera::MdrCamera, object::MdrSceneObject};

pub struct MdrScene {
  pub camera: MdrCamera,
  pub scene_objects: Vec<MdrSceneObject>,
}

impl MdrScene {
  pub(crate) fn new() -> Self {
    Self {
      camera: MdrCamera::new(),
      scene_objects: Vec::<MdrSceneObject>::new(),
    }
  }

  pub fn add_object(&mut self, object: MdrSceneObject) {
    self.scene_objects.push(object);
  }
}
