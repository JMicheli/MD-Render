pub mod color;
pub mod material;
pub mod mesh;
pub mod texture;
pub mod vertex;

use fxhash::{FxBuildHasher, FxHashMap};
use image::{io::Reader as ImageReader, DynamicImage};
use log::{debug, error, warn};
use std::{collections::HashMap, sync::Arc};
use vulkano::{
  buffer::{BufferUsage, CpuBufferPool},
  command_buffer::{CommandBufferExecFuture, PrimaryAutoCommandBuffer},
  device::{Device, Queue},
  format::Format,
  image::{view::ImageView, ImageDimensions, ImmutableImage, MipmapsCount},
  sampler::{Filter, Sampler, SamplerAddressMode, SamplerCreateInfo},
  sync::{GpuFuture, NowFuture},
};

pub use color::{MdrColorType, MdrRgb, MdrRgba};
pub use material::{
  MdrGpuMaterialHandle, MdrMaterial, MdrMaterialCreateInfo, MdrMaterialUniformData,
};
pub use mesh::{MdrGpuMeshHandle, MdrMesh, MdrMeshData};
pub use texture::{MdrGpuTextureHandle, MdrTexture};
pub use vertex::MdrVertex;

use self::texture::{MdrSamplerMode, MdrTextureCreateInfo};

/// Manages resources on the GPU by storing meshes, textures, and materials into libraries which
/// can be accessed by key. Objects in the scene only store these keys rather than maintaining
/// references to the buffers in which their data is stored.
pub struct MdrResourceManager {
  logical_device: Arc<Device>,
  queue: Arc<Queue>,

  vertex_buffer_pool: CpuBufferPool<MdrVertex>,
  index_buffer_pool: CpuBufferPool<u32>,
  mesh_library: HashMap<String, MdrGpuMeshHandle, FxBuildHasher>,

  material_buffer_pool: CpuBufferPool<MdrMaterialUniformData>,
  material_library: HashMap<String, MdrGpuMaterialHandle, FxBuildHasher>,

  texture_load_futures: Option<Box<dyn GpuFuture>>,
  sampler_palette: HashMap<MdrSamplerMode, Arc<Sampler>, FxBuildHasher>,
  texture_library: HashMap<String, MdrGpuTextureHandle, FxBuildHasher>,
}

impl MdrResourceManager {
  pub fn new(logical_device: Arc<Device>, queue: Arc<Queue>) -> Self {
    // Mesh memory handler initialization
    let vertex_buffer_pool =
      CpuBufferPool::<MdrVertex>::new(logical_device.clone(), BufferUsage::vertex_buffer());
    let index_buffer_pool =
      CpuBufferPool::<u32>::new(logical_device.clone(), BufferUsage::index_buffer());
    let mesh_library = FxHashMap::<String, MdrGpuMeshHandle>::default();

    // Material memory handler initialization
    let material_buffer_pool = CpuBufferPool::<MdrMaterialUniformData>::new(
      logical_device.clone(),
      BufferUsage::uniform_buffer(),
    );
    let material_library = FxHashMap::<String, MdrGpuMaterialHandle>::default();

    let sampler_palette = FxHashMap::<MdrSamplerMode, Arc<Sampler>>::default();
    let texture_library = FxHashMap::<String, MdrGpuTextureHandle>::default();

    Self {
      logical_device,
      queue,

      vertex_buffer_pool,
      index_buffer_pool,
      mesh_library,

      material_buffer_pool,
      material_library,

      texture_load_futures: None,
      sampler_palette,
      texture_library,
    }
  }

  // /////////////
  // Mesh handling
  // /////////////

  /// Load a mesh from an .obj file into the mesh library with a given name.
  /// `path` specifies a path to the .obj file.
  /// `name` is the name given to the mesh in the mesh library.
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

    let mesh_handle = self.upload_mesh_to_gpu(mesh);
    self.mesh_library.insert(String::from(name), mesh_handle);

    Ok(MdrMesh {
      name: String::from(name),
    })
  }

  /// Returns an `MdrMesh` specified by `name` from the mesh library. If no match is found for the
  /// key, it returns `MdrResourceError::MeshNotFound`.
  pub fn retrieve_mesh(&self, name: &str) -> Result<MdrMesh, MdrResourceError> {
    if !self.mesh_library.contains_key(name) {
      return Err(MdrResourceError::MeshNotFound);
    }

    Ok(MdrMesh {
      name: String::from(name),
    })
  }

  /// Removes the mesh specified by `name` from the mesh library and drops it, freeing it
  /// from GPU memory. Doing this will effectively invalidate any existing `MdrMesh` objects.
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

  // ////////////////
  // Texture handling
  // ////////////////

  pub fn load_texture(
    &mut self,
    texture_create_info: MdrTextureCreateInfo,
    name: &str,
  ) -> Result<MdrTexture, MdrResourceError> {
    // Check that the texture name isn't already in use
    if self.texture_library.contains_key(name) {
      error!("Texture library already contains name: {}", name);
      return Err(MdrResourceError::DuplicateTextureName);
    }

    // Load image data from disk
    let image = match ImageReader::open(texture_create_info.source) {
      Ok(reader) => reader.decode().unwrap(),
      Err(_) => return Err(MdrResourceError::ImageLoadError),
    };

    // Upload to GPU and catalogue texture in library
    let texture_handle = self.upload_image_to_gpu(image, texture_create_info);
    self
      .texture_library
      .insert(String::from(name), texture_handle);

    Ok(MdrTexture {
      name: String::from(name),
    })
  }

  /// Returns an `MdrTexture` specified by `name` from the texture library. If no match is found for the
  /// key, it returns `MdrResourceError::TextureNotFound`.
  pub fn retrieve_texture(&self, name: &str) -> Result<MdrTexture, MdrResourceError> {
    if !self.texture_library.contains_key(name) {
      return Err(MdrResourceError::TextureNotFound);
    }

    Ok(MdrTexture {
      name: String::from(name),
    })
  }

  /// Removes the texture specified by `name` from the texture library and drops it, freeing it
  /// from GPU memory. Doing this will effectively invalidate any existing `MdrTexture` objects.
  pub fn unload_texture(&mut self, name: &str) {
    if !self.texture_library.contains_key(name) {
      warn!(
        "Cannot unload texture {} because it is not in the library",
        name
      );
      return;
    }

    self.texture_library.remove(&String::from(name));
  }

  // /////////////////
  // Material handling
  // /////////////////

  pub fn create_material(
    &mut self,
    material_create_info: MdrMaterialCreateInfo,
    name: &str,
  ) -> Result<MdrMaterial, MdrResourceError> {
    // Check that the mesh name isn't already in use
    if self.material_library.contains_key(name) {
      error!("Material library already contains name: {}", name);
      return Err(MdrResourceError::DuplicateMaterialName);
    }

    // Generate material uniform buffer contents from create info
    let material = MdrMaterialUniformData {
      diffuse_color: material_create_info.diffuse_color.into(),
      alpha: material_create_info.alpha,

      specular_color: material_create_info.specular_color.into(),
      shininess: material_create_info.shininess,
    };

    // Push material to GPU and store in library
    let material_handle = self.upload_material_to_gpu(material);
    self
      .material_library
      .insert(String::from(name), material_handle);

    Ok(MdrMaterial {
      name: String::from(name),
    })
  }

  /// Returns an `MdrMaterial` specified by `name` from the material library. If no match is found for the
  /// key, it returns `MdrResourceError::MaterialNotFound`.
  pub fn retrieve_material(&self, name: &str) -> Result<MdrMaterial, MdrResourceError> {
    if !self.material_library.contains_key(name) {
      return Err(MdrResourceError::MaterialNotFound);
    }

    Ok(MdrMaterial {
      name: String::from(name),
    })
  }

  /// Removes the material specified by `name` from the material library and drops it, freeing it
  /// from GPU memory. Doing this will effectively invalidate any existing `MdrMaterial` objects.
  pub fn unload_material(&mut self, name: &str) {
    if !self.material_library.contains_key(name) {
      warn!(
        "Cannot unload material {} because it is not in the library",
        name
      );
      return;
    }

    self.material_library.remove(&String::from(name));
  }

  // //////////////////
  // Internal functions
  // //////////////////

  /// Gets a reference to the `MdrGpuMeshHandle` that corresponds to the input `MdrMesh`.
  /// This is called when building the render command buffer to bind the underlying buffers.
  pub(crate) fn get_mesh_handle(&self, mesh: &MdrMesh) -> &MdrGpuMeshHandle {
    match self.mesh_library.get_key_value(&mesh.name) {
      Some((_, handle)) => handle,
      None => {
        panic!("Could not find mesh {} in mesh library", mesh.name);
      }
    }
  }

  /// Gets a reference to the `MdrGpuTextureHandle` that corresponds to the input `MdrTexture`.
  /// This is called when building the render command buffer to bind the underlying buffers.
  pub(crate) fn get_texture_handle(&self, texture: &MdrTexture) -> &MdrGpuTextureHandle {
    match self.texture_library.get_key_value(&texture.name) {
      Some((_, handle)) => handle,
      None => {
        panic!("Could not find texture {} in texture library", texture.name);
      }
    }
  }

  /// Gets a reference to the `MdrGpuMaterialHandle` that corresponds to the input `MdrMaterial`.
  /// This is called when building the render command buffer to bind the underlying buffers.
  pub(crate) fn get_material_handle(&self, mat: &MdrMaterial) -> &MdrGpuMaterialHandle {
    match self.material_library.get_key_value(&mat.name) {
      Some((_, handle)) => handle,
      None => {
        panic!("Could not find material {} in mat library", mat.name);
      }
    }
  }

  /// Uploads input `MdrMeshdata` to the GPU and returns an `MdrGpuMeshHandle` containing the
  /// vertex buffer, index buffer, and index count for the input data.
  fn upload_mesh_to_gpu(&mut self, mesh: MdrMeshData) -> MdrGpuMeshHandle {
    let index_count = mesh.indices.len() as u32;
    MdrGpuMeshHandle {
      vertex_chunk: self.vertex_buffer_pool.chunk(mesh.vertices).unwrap(),
      index_chunk: self.index_buffer_pool.chunk(mesh.indices).unwrap(),
      index_count,
    }
  }

  /// Uploads an input `image::DynamicImage` to the GPU  with settings defined by the `texture_create_info`.
  /// Returns an `MdrGpuTextureHandle` containing the resulting image view and sampler.
  fn upload_image_to_gpu(
    &mut self,
    image: DynamicImage,
    texture_create_info: MdrTextureCreateInfo,
  ) -> MdrGpuTextureHandle {
    // Get image parameters
    let dimensions = ImageDimensions::Dim2d {
      width: image.width(),
      height: image.height(),
      array_layers: 1,
    };

    // Handle intended color use types provided by user
    let (immutable_image, upload_future) = match texture_create_info.color_type {
      // SRGBA images are in standard (gamma-corrected) color space.
      // They are used for images that will be shown to the user
      MdrColorType::SRGBA => ImmutableImage::from_iter(
        image.to_rgba8().into_raw(),
        dimensions,
        MipmapsCount::One,
        Format::R8G8B8A8_SRGB,
        self.queue.clone(),
      )
      .unwrap(),

      // SRGB images are in standard color space, too, but with just the R, G, and B channels.
      // They are also used for images that will be shown to the user
      MdrColorType::SRGB => ImmutableImage::from_iter(
        image.to_rgb8().into_raw(),
        dimensions,
        MipmapsCount::One,
        Format::R8G8B8_SRGB,
        self.queue.clone(),
      )
      .unwrap(),

      // NonColorData images are in linear color space, and their values are read as data, not rgb.
      // They are used for images that inform shading algorithms (normal maps, roughness maps, etc.)
      // TODO Make sure assuming NonColor = RGB doesn't break something elsewhere
      MdrColorType::NonColorData => ImmutableImage::from_iter(
        image.to_rgb8().into_raw(),
        dimensions,
        MipmapsCount::One,
        Format::R8G8B8_UNORM,
        self.queue.clone(),
      )
      .unwrap(),
    };

    let image_view = ImageView::new_default(immutable_image).unwrap();
    let sampler = self.get_sampler(texture_create_info.sampler_mode);
    self.join_texture_future(upload_future);

    MdrGpuTextureHandle {
      image_view,
      sampler,
    }
  }

  /// Uploads an input `MdrMaterialUniformData` to the GPU .
  /// Returns an `MdrGpuMaterialHandle` containing the resulting buffer.
  fn upload_material_to_gpu(&mut self, material: MdrMaterialUniformData) -> MdrGpuMaterialHandle {
    MdrGpuMaterialHandle {
      material_chunk: self.material_buffer_pool.chunk([material]).unwrap(),
    }
  }

  /// Gets a sampler with the input `MdrSamplerMode` by either grabbing a reference off the
  /// sampler palette or, if none is available, creating a new one.
  fn get_sampler(&mut self, sampler_mode: MdrSamplerMode) -> Arc<Sampler> {
    // If we've already got that sampler, return it
    if let Some((_, sampler)) = self.sampler_palette.get_key_value(&sampler_mode) {
      return sampler.clone();
    }

    // If not, we need to create one
    let sampler = Sampler::new(
      self.logical_device.clone(),
      SamplerCreateInfo {
        mag_filter: Filter::Linear,
        min_filter: Filter::Linear,
        address_mode: match sampler_mode {
          MdrSamplerMode::Repeat => [SamplerAddressMode::Repeat; 3],
          MdrSamplerMode::ClampToEdge => [SamplerAddressMode::ClampToEdge; 3],
        },
        ..Default::default()
      },
    )
    .unwrap();

    // Map the new sampler and return it
    self.sampler_palette.insert(sampler_mode, sampler).unwrap()
  }

  fn join_texture_future(
    &mut self,
    texture_future: CommandBufferExecFuture<NowFuture, PrimaryAutoCommandBuffer>,
  ) {
    let new_future = match self.texture_load_futures.take() {
      Some(future) => future.join(texture_future).boxed(),
      None => texture_future.boxed(),
    };

    self.texture_load_futures = Some(new_future);
  }
}

#[derive(Debug)]
/// Error emitted by `MdrResourceManager`.
pub enum MdrResourceError {
  /// Emitted when the resource manager fails to load an .obj file.
  ObjLoadError,
  /// Emitted when the resource manager fails to load an image file.
  ImageLoadError,

  /// Emitted when the resource manager cannot find a mesh with a given name in its
  /// mesh library.
  MeshNotFound,
  /// Emitted when the resource manager attempts to add a mesh with a name that is
  /// already present in the mesh library.
  DuplicateMeshName,

  /// Emitted when the resource manager cannot find a material with a given name in its
  /// material library.
  MaterialNotFound,
  /// Emitted when the resource manager attempts to add a material with a name that is
  /// already present in the material library.
  DuplicateMaterialName,

  /// Emitted when the resource manager cannot find a texture with a given name in its
  /// texture library.
  TextureNotFound,
  /// Emitted when the resource manager attempts to add a texture with a name that is
  /// already present in the texture library.
  DuplicateTextureName,
}
