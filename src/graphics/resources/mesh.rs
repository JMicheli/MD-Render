use std::sync::Arc;

use log::{debug, error};
use vulkano::{
  buffer::{BufferUsage, CpuAccessibleBuffer},
  device::Device,
};

use super::MdrVertex;

#[derive(Default)]
pub struct MdrMesh {
  pub vertices: Vec<MdrVertex>,
  pub indices: Vec<u32>,
}

impl MdrMesh {
  pub fn load_obj(file_path: &str) -> Self {
    let options = tobj::GPU_LOAD_OPTIONS;
    let load_result = tobj::load_obj(file_path, &options);

    let (models, _) = match load_result {
      Ok(value) => value,
      Err(e) => {
        error!("Failed to load obj file: {}, reason: {}", file_path, e);
        // Return empty mesh
        return Self::default();
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

    debug!("Loaded obj file: {}", file_path);
    Self {
      vertices,
      indices: indices.clone(),
    }
  }

  pub fn upload_to_gpu(&self, logical_device: &Arc<Device>) -> MdrMeshBuffer {
    let vertex_data = CpuAccessibleBuffer::from_iter(
      logical_device.clone(),
      BufferUsage::vertex_buffer(),
      false,
      self.vertices.clone(),
    )
    .unwrap();

    let index_data = CpuAccessibleBuffer::from_iter(
      logical_device.clone(),
      BufferUsage::index_buffer(),
      false,
      self.indices.clone(),
    )
    .unwrap();

    let index_count = self.indices.len() as u32;

    MdrMeshBuffer {
      vertex_data,
      index_data,
      index_count,
    }
  }
}

pub struct MdrMeshBuffer {
  pub vertex_data: Arc<CpuAccessibleBuffer<[MdrVertex]>>,
  pub index_data: Arc<CpuAccessibleBuffer<[u32]>>,
  pub index_count: u32,
}
