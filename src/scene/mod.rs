mod camera;
mod lighting;
mod object;
pub mod transform;

pub use camera::MdrCamera;
pub use lighting::MdrLight;
pub use object::MdrRenderObject;

use self::lighting::MdrLightSet;

pub struct MdrScene {
  pub camera: MdrCamera,
  pub lights: MdrLightSet,
  pub scene_objects: Vec<MdrRenderObject>,
}

impl MdrScene {
  pub(crate) fn new() -> Self {
    Self {
      camera: MdrCamera::default(),
      lights: MdrLightSet::new(),
      scene_objects: Vec::<MdrRenderObject>::new(),
    }
  }

  pub fn add_object(&mut self, object: MdrRenderObject) {
    self.scene_objects.push(object);
  }
}
