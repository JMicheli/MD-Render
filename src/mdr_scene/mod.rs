pub mod mdr_camera;
pub mod mdr_mesh;
pub mod mdr_object;
pub mod mdr_vertex;

use std::sync::Arc;

use cgmath::{Matrix4, Vector3};
use mdr_camera::MdrCamera;
use mdr_mesh::MdrMesh;
use mdr_object::MdrObject;

use crate::mdr_core::MdrEngine;

pub struct MdrScene {
  engine: Arc<MdrEngine>,

  objects: Vec<Arc<MdrObject>>,
  camera: MdrCamera,
}

impl MdrScene {
  pub fn new(engine: &Arc<MdrEngine>) -> Arc<Self> {
    Arc::new(Self {
      engine: engine.clone(),

      objects: vec![],
      camera: MdrCamera::new(),
    })
  }

  pub fn load_obj(&self, file_path: &str) -> Arc<MdrObject> {
    let vk_logical_device = self.engine.get_device().vk_logical_device.clone();
    let mesh = MdrMesh::from_obj(&vk_logical_device, file_path);
    let transform = Matrix4::from_translation(Vector3::new(0.0, 0.0, 0.0));

    return MdrObject::new(mesh, transform);
  }

  pub fn add_object(&mut self, object: Arc<MdrObject>) {
    self.objects.push(object);
  }

  pub fn get_render_objects(&self) -> &Vec<Arc<MdrObject>> {
    return &self.objects;
  }
}
