use super::{camera::MdrCamera, object::MdrSceneObject};

pub struct MdrScene {
  camera: MdrCamera,
  scene_objects: Vec<MdrSceneObject>,
}

impl MdrScene {
  pub fn new() -> Self {
    Self {
      camera: MdrCamera::new(),
      scene_objects: Vec::<MdrSceneObject>::new(),
    }
  }
}
