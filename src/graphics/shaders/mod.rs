use std::sync::Arc;

use vulkano::{
  device::Device,
  shader::{ShaderCreationError, ShaderModule},
};

pub mod mesh_vertex_shader {
  vulkano_shaders::shader! {
    ty: "vertex",
    path: "src/graphics/shaders/mesh.vert",
    types_meta: {
      use bytemuck::{Pod, Zeroable};

      #[derive(Clone, Copy, Zeroable, Pod)]
    },
  }
}

pub mod mesh_fragment_shader {
  vulkano_shaders::shader! {
    ty: "fragment",
    path: "src/graphics/shaders/mesh.frag",
    types_meta: {
      use bytemuck::{Pod, Zeroable};

      #[derive(Clone, Copy, Zeroable, Pod)]
    },
  }
}

pub fn load_mesh_shaders(logical_device: &Arc<Device>) -> (Arc<ShaderModule>, Arc<ShaderModule>) {
  // Vertex shader
  let vs = validate_load_result(mesh_vertex_shader::load(logical_device.clone()));
  // Fragment shader
  let fs = validate_load_result(mesh_fragment_shader::load(logical_device.clone()));

  (vs, fs)
}

pub mod light_vertex_shader {
  vulkano_shaders::shader! {
    ty: "vertex",
    path: "src/graphics/shaders/light.vert",
    types_meta: {
      use bytemuck::{Pod, Zeroable};

      #[derive(Clone, Copy, Zeroable, Pod)]
    },
  }
}

pub mod light_fragment_shader {
  vulkano_shaders::shader! {
    ty: "fragment",
    path: "src/graphics/shaders/light.frag",
  }
}

pub fn load_light_shaders(logical_device: &Arc<Device>) -> (Arc<ShaderModule>, Arc<ShaderModule>) {
  // Vertex shader
  let vs = validate_load_result(light_vertex_shader::load(logical_device.clone()));
  // Fragment shader
  let fs = validate_load_result(light_fragment_shader::load(logical_device.clone()));

  (vs, fs)
}

fn validate_load_result(
  output: Result<Arc<ShaderModule>, ShaderCreationError>,
) -> Arc<ShaderModule> {
  match output {
    Ok(value) => value,
    Err(e) => {
      panic!("Failed to load shader module: {}", e);
    }
  }
}
