use std::sync::Arc;

use vulkano::{
  device::{
    physical::{PhysicalDevice, PhysicalDeviceType, QueueFamily},
    Device, DeviceCreateInfo, DeviceExtensions, Queue, QueueCreateInfo,
  },
  instance::Instance,
  swapchain::Surface,
};
use winit::window::Window;

use super::mdr_window::MdrWindow;

pub struct MdrDevice {
  pub vk_logical_device: Arc<Device>,
  pub vk_queue: Arc<Queue>,
}

impl MdrDevice {
  pub fn new(instance: &Arc<Instance>, window: &MdrWindow) -> Arc<Self> {
    // Get surface from window
    let surface = &window.surface;
    // Pick physical device
    let (physical_device, queue_family) = Self::pick_physical_device(instance, surface.clone());
    // Create logical device
    let device_extensions = DeviceExtensions {
      khr_swapchain: true,
      ..DeviceExtensions::none()
    };
    let (vk_logical_device, mut queues) = Device::new(
      physical_device,
      DeviceCreateInfo {
        queue_create_infos: vec![QueueCreateInfo::family(queue_family)],
        enabled_extensions: physical_device
          .required_extensions()
          .union(&device_extensions),
        ..Default::default()
      },
    )
    .expect("Failed to create logical device");

    // Get queue
    let vk_queue = queues.next().unwrap();

    return Arc::new(Self {
      vk_logical_device,
      vk_queue,
    });
  }

  pub fn queue_family(&self) -> QueueFamily {
    return self.vk_queue.family();
  }

  fn pick_physical_device(
    instance: &Arc<Instance>,
    surface: Arc<Surface<Window>>,
  ) -> (PhysicalDevice, QueueFamily) {
    let device_extensions = DeviceExtensions {
      khr_swapchain: true,
      ..DeviceExtensions::none()
    };

    let (physical_device, queue_family) = PhysicalDevice::enumerate(&instance)
      .filter(|&p| p.supported_extensions().is_superset_of(&device_extensions))
      .filter_map(|p| {
        p.queue_families()
          .find(|&q| q.supports_graphics() && q.supports_surface(&surface).unwrap_or(false))
          .map(|q| (p, q))
      })
      .min_by_key(|(p, _)| match p.properties().device_type {
        PhysicalDeviceType::DiscreteGpu => 0,
        PhysicalDeviceType::IntegratedGpu => 1,
        PhysicalDeviceType::VirtualGpu => 2,
        PhysicalDeviceType::Cpu => 3,
        PhysicalDeviceType::Other => 4,
      })
      .expect("No physical device available");

    return (physical_device, queue_family);
  }
}
