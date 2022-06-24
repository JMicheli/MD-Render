use rustc_hash::FxHashMap;

use crate::scene::{MdrMesh, MdrMeshId};

pub struct MdrResourceManager {
  mesh_map: FxHashMap<MdrMeshId, MdrMesh>,
  current_mesh_id: MdrMeshId,
}

impl MdrResourceManager {
  pub fn new() -> Self {
    Self {
      mesh_map: FxHashMap::default(),
      current_mesh_id: 0,
    }
  }

  pub fn store_mesh(&mut self, mesh: MdrMesh) -> MdrMeshId {
    let mesh_id = self.current_mesh_id;
    self.current_mesh_id += 1;

    self.mesh_map.insert(mesh_id, mesh);

    mesh_id
  }

  pub fn get_mesh(&self, id: MdrMeshId) -> &MdrMesh {
    self.mesh_map.get(&id).unwrap()
  }
}
