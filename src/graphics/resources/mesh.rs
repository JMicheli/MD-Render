use log::{debug, error};

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
}
