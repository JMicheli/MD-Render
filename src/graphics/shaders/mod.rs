use std::sync::Arc;

use vulkano::{device::Device, shader::ShaderModule};

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
  let vs = match mesh_vertex_shader::load(logical_device.clone()) {
    Ok(value) => value,
    Err(e) => {
      panic!("Failed to load vertex shader module: {}", e);
    }
  };

  // Fragment shader
  let fs = match mesh_fragment_shader::load(logical_device.clone()) {
    Ok(value) => value,
    Err(e) => {
      panic!("Failed to load fragment shader module: {}", e);
    }
  };

  (vs, fs)
}
