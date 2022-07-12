use bytemuck::{Pod, Zeroable};
use log::{debug, error};

#[repr(C)]
#[derive(Default, Copy, Clone, Zeroable, Pod)]
pub struct Vertex {
  pub a_position: [f32; 3],
  pub a_normal: [f32; 3],
  pub a_tex_coord: [f32; 2],
}

vulkano::impl_vertex!(Vertex, a_position, a_normal, a_tex_coord);

#[derive(Default)]
pub struct MdrMesh {
  pub vertices: Vec<Vertex>,
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
    let tex_coords = &model.mesh.texcoords;

    // Loop over vertices
    let vertex_count = positions.len() / 3;
    let mut vertices = Vec::with_capacity(vertex_count);
    for vertex_index in 0..vertex_count {
      let index = 3 * vertex_index;
      let uv_index = 2 * vertex_index;
      vertices.push(Vertex {
        a_position: [positions[index], positions[index + 1], positions[index + 2]],
        a_normal: [normals[index], normals[index + 1], normals[index + 2]],
        a_tex_coord: [tex_coords[uv_index], tex_coords[uv_index + 1]],
      });
    }

    debug!("Loaded obj file: {}", file_path);
    Self {
      vertices,
      indices: indices.clone(),
    }
  }
}
