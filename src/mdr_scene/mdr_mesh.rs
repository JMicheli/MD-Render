use std::sync::Arc;

use vulkano::{
  buffer::{BufferUsage, CpuAccessibleBuffer},
  device::Device,
};

pub use super::mdr_vertex::Vertex;

pub struct MdrMesh {
  pub vertex_buffer: Arc<CpuAccessibleBuffer<[Vertex]>>,
  pub index_buffer: Arc<CpuAccessibleBuffer<[u32]>>,
}

impl MdrMesh {
  pub fn from_obj(device: &Arc<Device>, file_path: &str) -> Arc<MdrMesh> {
    let options = tobj::LoadOptions::default();
    let (models, _) = tobj::load_obj(file_path, &options).expect("Failed to load obj file.");
    // Take only the first model
    let model = &models[0];

    // Load positions, indices, and normals
    let positions = &model.mesh.positions;
    let indices = &model.mesh.indices;
    let normals = &model.mesh.normals;

    // Loop over vertices
    let vertex_count = positions.len() / 3;
    let mut vertices = Vec::with_capacity(vertex_count);
    for vertex_index in 0..vertex_count {
      let index = vertex_index * 3;
      vertices.push(Vertex {
        position: [
          positions[index + 0],
          positions[index + 1],
          positions[index + 2],
        ],
        normal: [normals[index + 0], normals[index + 0], normals[index + 0]],
        color: [0.0, 1.0, 0.0, 1.0],
      });
    }

    // Create buffers
    let vertex_buffer = CpuAccessibleBuffer::from_iter(
      device.clone(),
      BufferUsage::vertex_buffer(),
      false,
      vertices.iter().cloned(),
    )
    .unwrap();
    let index_buffer = CpuAccessibleBuffer::from_iter(
      device.clone(),
      BufferUsage::index_buffer(),
      false,
      indices.iter().cloned(),
    )
    .unwrap();
    // Create mesh
    return Arc::new(MdrMesh {
      vertex_buffer,
      index_buffer,
    });
  }
}
