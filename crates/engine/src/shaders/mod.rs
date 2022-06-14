pub mod vertex_shader {
  vulkano_shaders::shader! {
    ty: "vertex",
    path: "src/shaders/basic.vert",
    types_meta: {
      use bytemuck::{Pod, Zeroable};

      #[derive(Clone, Copy, Zeroable, Pod)]
    },
  }
}

pub mod fragment_shader {
  vulkano_shaders::shader! {
    ty: "fragment",
    path: "src/shaders/basic.frag",
  }
}
