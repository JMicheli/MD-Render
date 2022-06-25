mod camera;
mod material;
mod mesh;
mod object;

pub use camera::MdrCamera;
pub use mesh::{MdrMesh, Vertex};
pub use object::{MdrSceneObject, MdrTransform};

pub struct MdrScene {
  pub camera: MdrCamera,
  pub scene_objects: Vec<MdrSceneObject>,
}

impl MdrScene {
  pub(crate) fn new() -> Self {
    Self {
      camera: MdrCamera::default(),
      scene_objects: Vec::<MdrSceneObject>::new(),
    }
  }

  pub fn add_object(&mut self, object: MdrSceneObject) {
    self.scene_objects.push(object);
  }
}
