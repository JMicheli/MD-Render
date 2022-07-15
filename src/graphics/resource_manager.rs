use std::{collections::HashMap, sync::Arc};

use log::{debug, error, warn};
use vulkano::{
  buffer::{BufferUsage, CpuBufferPool},
  device::{Device, Queue},
};

use crate::resources::{MdrGpuMeshHandle, MdrMesh, MdrMeshData, MdrVertex};

pub struct MdrResourceManager {
  logical_device: Arc<Device>,
  queue: Arc<Queue>,

  vertex_buffer_pool: CpuBufferPool<MdrVertex>,
  index_buffer_pool: CpuBufferPool<u32>,
  mesh_library: HashMap<String, MdrGpuMeshHandle>,
}

impl MdrResourceManager {
  pub fn new(logical_device: Arc<Device>, queue: Arc<Queue>) -> Self {
    // Mesh memory handler initialization
    let vertex_buffer_pool =
      CpuBufferPool::<MdrVertex>::new(logical_device.clone(), BufferUsage::vertex_buffer());
    let index_buffer_pool =
      CpuBufferPool::<u32>::new(logical_device.clone(), BufferUsage::index_buffer());
    let mesh_library = HashMap::<String, MdrGpuMeshHandle>::new();

    Self {
      logical_device,
      queue,

      vertex_buffer_pool,
      index_buffer_pool,
      mesh_library,
    }
  }

  pub fn load_mesh_obj<'a>(
    &mut self,
    path: &str,
    name: &'a str,
  ) -> Result<MdrMesh, MdrResourceError> {
    // Check that the mesh name isn't already in use
    if self.mesh_library.contains_key(name) {
      error!("Mesh library already contains name: {}", name);
      return Err(MdrResourceError::DuplicateMeshName);
    }

    // Load data from disk
    let options = tobj::GPU_LOAD_OPTIONS;
    let load_result = tobj::load_obj(path, &options);
    let (models, _) = match load_result {
      Ok(value) => value,
      Err(e) => {
        error!("Failed to load obj file: {}, reason: {}", path, e);
        return Err(MdrResourceError::ObjLoadError);
      }
    };

    // Take only the first model
    let model = &models[0];

    // Get positions, indices, and normals for each vertex
    let positions = &model.mesh.positions;
    let indices = &model.mesh.indices;
    let normals = &model.mesh.normals;

    // Loop over vertices
    let vertex_count = positions.len() / 3;
    let mut vertices = Vec::with_capacity(vertex_count);
    for vertex_index in 0..vertex_count {
      let index = 3 * vertex_index;
      vertices.push(MdrVertex {
        a_position: [positions[index], positions[index + 1], positions[index + 2]],
        a_normal: [normals[index], normals[index + 1], normals[index + 2]],
      });
    }

    debug!("Loaded obj file: {}", path);
    let mesh = MdrMeshData {
      vertices,
      indices: indices.clone(),
    };

    let handle = self.upload_mesh_to_gpu(mesh);
    self.mesh_library.insert(String::from(name), handle);

    Ok(MdrMesh {
      name: String::from(name),
    })
  }

  pub fn retrieve_mesh(&self, name: &str) -> Result<MdrMesh, MdrResourceError> {
    if !self.mesh_library.contains_key(name) {
      return Err(MdrResourceError::MeshNotFound);
    }

    Ok(MdrMesh {
      name: String::from(name),
    })
  }

  pub(crate) fn get_mesh_handle(&self, mesh: &MdrMesh) -> &MdrGpuMeshHandle {
    match self.mesh_library.get_key_value(&mesh.name) {
      Some((_, V)) => V,
      None => {
        panic!("Could not find mesh {} in mesh library", mesh.name);
      }
    }
  }

  pub fn unload_mesh(&mut self, name: &str) {
    if !self.mesh_library.contains_key(name) {
      warn!(
        "Cannot unload mesh {} because it is not in the library",
        name
      );
      return;
    }

    self.mesh_library.remove(&String::from(name));
  }

  fn upload_mesh_to_gpu(&mut self, mesh: MdrMeshData) -> MdrGpuMeshHandle {
    let index_count = mesh.indices.len() as u32;
    MdrGpuMeshHandle {
      vertex_subbuffer: self.vertex_buffer_pool.chunk(mesh.vertices).unwrap(),
      index_subbuffer: self.index_buffer_pool.chunk(mesh.indices).unwrap(),
      index_count,
    }
  }
}

#[derive(Debug)]
/// Error emitted by `MdrResourceManager`.
pub enum MdrResourceError {
  /// Emitted when the resource manager fails to load an .obj file.
  ObjLoadError,

  /// Emitted when the resource manager cannot find a mesh with a given name in its
  /// mesh library.
  MeshNotFound,
  /// Emitted when the resource manager attempts to add a mesh with a name that is
  /// already present in the mesh library.
  DuplicateMeshName,
}
