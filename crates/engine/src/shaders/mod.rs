use std::sync::Arc;

use vulkano::{device::Device, shader::ShaderModule};

pub mod basic_vertex_shader {
  vulkano_shaders::shader! {
    ty: "vertex",
    path: "src/shaders/basic.vert",
    types_meta: {
      use bytemuck::{Pod, Zeroable};

      #[derive(Clone, Copy, Zeroable, Pod)]
    },
  }
}

pub mod basic_fragment_shader {
  vulkano_shaders::shader! {
    ty: "fragment",
    path: "src/shaders/basic.frag",
  }
}

pub fn load_basic_shaders(logical_device: &Arc<Device>) -> (Arc<ShaderModule>, Arc<ShaderModule>) {
  // Vertex shader
  let vs = match basic_vertex_shader::load(logical_device.clone()) {
    Ok(value) => value,
    Err(e) => {
      panic!("Failed to load vertex shader module: {}", e);
    }
  };

  // Fragment shader
  let fs = match basic_fragment_shader::load(logical_device.clone()) {
    Ok(value) => value,
    Err(e) => {
      panic!("Failed to load fragment shader module: {}", e);
    }
  };

  (vs, fs)
}

pub mod triangle_vertex_shader {
  vulkano_shaders::shader! {
    ty: "vertex",
    path: "src/shaders/triangle.vert",
    types_meta: {
      #[derive(Clone, Copy, Zeroable, Pod)]
    },
  }
}

pub mod triangle_fragment_shader {
  vulkano_shaders::shader! {
    ty: "fragment",
    path: "src/shaders/triangle.frag",
  }
}

pub fn load_triangle_shaders(
  logical_device: &Arc<Device>,
) -> (Arc<ShaderModule>, Arc<ShaderModule>) {
  // Vertex shader
  let vs = match triangle_vertex_shader::load(logical_device.clone()) {
    Ok(value) => value,
    Err(e) => {
      panic!("Failed to load vertex shader module: {}", e);
    }
  };

  // Fragment shader
  let fs = match triangle_fragment_shader::load(logical_device.clone()) {
    Ok(value) => value,
    Err(e) => {
      panic!("Failed to load fragment shader module: {}", e);
    }
  };

  (vs, fs)
}
