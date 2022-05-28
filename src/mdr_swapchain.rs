use std::sync::Arc;

use vulkano::device::Device;
use vulkano::format::Format;
use vulkano::image::{ImageUsage, SwapchainImage};
use vulkano::swapchain::{
  self, AcquireError, SurfaceCapabilities, Swapchain, SwapchainAcquireFuture, SwapchainCreateInfo,
};

use winit::dpi::PhysicalSize;
use winit::window::Window;

use crate::mdr_device::MdrDevice;
use crate::mdr_window::MdrWindow;

pub struct MdrSwapchain {
  vk_image_format: Option<Format>,
  vk_logical_device: Arc<Device>,
  pub vk_swapchain: Arc<Swapchain<Window>>,
  pub vk_images: Vec<Arc<SwapchainImage<Window>>>,
}

impl MdrSwapchain {
  pub fn new(device: &Arc<MdrDevice>, window: &MdrWindow) -> Self {
    let vk_surface = &window.surface;
    let vk_logical_device = &device.vk_logical_device;
    let vk_physical_device = vk_logical_device.physical_device();

    // Retrieve surface capabilities with respect to the physical device
    let surface_capabilities = vk_physical_device
      .surface_capabilities(vk_surface, Default::default())
      .expect("Failed to retrieve surface capabilities.");
    // Get other settings
    let dimensions = vk_surface.window().inner_size();
    let vk_image_format = Some(
      vk_physical_device
        .surface_formats(vk_surface, Default::default())
        .unwrap()[0]
        .0,
    );

    let (vk_swapchain, vk_images) = Swapchain::new(
      vk_logical_device.clone(),
      vk_surface.clone(),
      SwapchainCreateInfo {
        min_image_count: surface_capabilities.min_image_count + 1,
        image_format: vk_image_format,
        image_extent: dimensions.into(),
        image_usage: ImageUsage::color_attachment(),
        composite_alpha: surface_capabilities
          .supported_composite_alpha
          .iter()
          .next()
          .unwrap(),
        ..Default::default()
      },
    )
    .expect("Failed to generate swapchain");

    // Clone to keep shared reference
    let vk_logical_device = vk_logical_device.clone();
    Self {
      vk_logical_device,
      vk_swapchain,
      vk_image_format,
      vk_images,
    }
  }

  pub fn recreate(&mut self) {
    let vk_surface = self.vk_swapchain.surface();
    let dimensions = vk_surface.window().inner_size();

    // Recreate swapchain
    let mut recreate_info = self.vk_swapchain.create_info();
    recreate_info.image_extent = dimensions.into();
    (self.vk_swapchain, self.vk_images) = self
      .vk_swapchain
      .recreate(recreate_info)
      .expect("Failed to recreate swapchain");
  }

  pub fn image_format(&self) -> Format {
    return self.vk_image_format.unwrap();
  }

  pub fn acquire_next_image(
    &self,
  ) -> Result<(usize, bool, SwapchainAcquireFuture<Window>), AcquireError> {
    return swapchain::acquire_next_image(self.vk_swapchain.clone(), None);
  }
}
