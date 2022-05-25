use std::sync::Arc;

use vulkano::device::Device;
use vulkano::format::Format;
use vulkano::image::view::ImageView;
use vulkano::image::{ImageUsage, SwapchainImage};
use vulkano::render_pass::{Framebuffer, FramebufferCreateInfo, RenderPass};
use vulkano::swapchain::{Surface, SurfaceCapabilities, Swapchain, SwapchainCreateInfo};

use winit::dpi::PhysicalSize;
use winit::window::Window;

use crate::mdr_device::MdrDevice;
use crate::mdr_window::MdrWindow;

pub struct MdrSwapchain {
  device: Arc<MdrDevice>,
  surface: Arc<Surface<Window>>,
  swapchain: Arc<Swapchain<Window>>,

  images: Vec<Arc<SwapchainImage<Window>>>,
  image_format: Option<Format>,
  surface_capabilities: SurfaceCapabilities,
}

impl MdrSwapchain {
  pub fn new(device: Arc<MdrDevice>, surface: Arc<Surface<Window>>) -> Self {
    let logical_device = device.logical_device();
    let physical_device = logical_device.physical_device();

    // Retrieve surface capabilities with respect to the physical device
    let surface_capabilities = physical_device
      .surface_capabilities(&surface, Default::default())
      .expect("Failed to retrieve surface capabilities.");
    // Get other settings
    let dimensions = surface.window().inner_size();
    let image_format = Some(
      physical_device
        .surface_formats(&surface, Default::default())
        .unwrap()[0]
        .0,
    );

    let (swapchain, images) = Self::generate_swapchain(
      &logical_device,
      &surface,
      dimensions,
      image_format,
      &surface_capabilities,
    );

    Self {
      device,
      surface,
      swapchain,
      images,
      image_format,
      surface_capabilities,
    }
  }

  pub fn regenerate(&mut self) {
    let device = self.device.logical_device();

    // Makes the assumption that image_format and surface_capabilities remain static once set
    (self.swapchain, self.images) = Self::generate_swapchain(
      &device,
      &self.surface,
      self.surface.window().inner_size(),
      self.image_format,
      &self.surface_capabilities,
    );
  }

  pub fn create_render_pass(&self) -> Arc<RenderPass> {
    let image_format = self
      .image_format
      .expect("No swapchain image format available, was the swapchain initialized?");

    return vulkano::single_pass_renderpass!(
      self.device.logical_device(),
      attachments: {
        color: {
          load: Clear,
          store: Store,
          format: image_format,
          samples: 1,
        }
      },
      pass: {
        color: [color],
        depth_stencil: {}
      }
    )
    .unwrap();
  }

  pub fn create_frame_buffers(&self, render_pass: Arc<RenderPass>) -> Vec<Arc<Framebuffer>> {
    return self
      .images
      .iter()
      .map(|image| {
        let view = ImageView::new_default(image.clone()).unwrap();
        Framebuffer::new(
          render_pass.clone(),
          FramebufferCreateInfo {
            attachments: vec![view],
            ..Default::default()
          },
        )
        .unwrap()
      })
      .collect::<Vec<_>>();
  }

  pub fn image_format(&self) -> Format {
    return self
      .image_format
      .expect("Accesed image format before swpachain initialization");
  }

  fn generate_swapchain(
    logical_device: &Arc<Device>,
    surface: &Arc<Surface<Window>>,
    dimensions: PhysicalSize<u32>,
    image_format: Option<Format>,
    capabilities: &SurfaceCapabilities,
  ) -> (Arc<Swapchain<Window>>, Vec<Arc<SwapchainImage<Window>>>) {
    return Swapchain::new(
      logical_device.clone(),
      surface.clone(),
      SwapchainCreateInfo {
        min_image_count: capabilities.min_image_count + 1,
        image_format: image_format,
        image_extent: dimensions.into(),
        image_usage: ImageUsage::color_attachment(),
        composite_alpha: capabilities
          .supported_composite_alpha
          .iter()
          .next()
          .unwrap(),
        ..Default::default()
      },
    )
    .unwrap();
  }
}
