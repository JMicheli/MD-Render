use log::{debug, info, warn};
use std::sync::Arc;
use winit::{event_loop::EventLoop, window::Window};

use vulkano::{
  device::{
    physical::{PhysicalDevice, PhysicalDeviceType, QueueFamily},
    Device, DeviceCreateInfo, DeviceExtensions, Queue, QueueCreateInfo,
  },
  instance::{Instance, InstanceCreateInfo, InstanceExtensions},
  swapchain::Surface,
};

use crate::graphics_context::window::{MdrWindow, MdrWindowOptions};

/// A Vulkan graphics context, contains Vulkano members.
pub struct MdrGraphicsContext {
  instance: Arc<Instance>,
  window: Arc<MdrWindow>,
  logical_device: Arc<Device>,
}

impl MdrGraphicsContext {
  /// Create a new MD Renderer Graphics context with optional debug.
  pub fn new(event_loop: &EventLoop<()>, debug_enabled: bool) -> Self {
    debug!("Creating graphics context");

    // Create instance containing Vulkan function pointers
    let instance = Self::create_instance(debug_enabled);
    debug!("Created vulkan instance");

    // Create window
    let window_options = MdrWindowOptions {
      width: 800,
      height: 600,
      resizable: true,
      title: "MD Renderer",
    };
    let window = MdrWindow::new(&instance, &event_loop, window_options);
    debug!("Created window");

    // Select physical device and queue
    let (physical_device, queue_family) =
      Self::pick_physical_device(&instance, window.surface.clone());
    info!(
      "Using device: {} (type: {:?})",
      physical_device.properties().device_name,
      physical_device.properties().device_type,
    );

    // Create logical device
    let device_extensions = DeviceExtensions {
      khr_swapchain: true,
      ..DeviceExtensions::none()
    };
    let (logical_device, mut queues) =
      Self::create_logical_device(physical_device, device_extensions, queue_family);
    debug!("Created logical device");

    Self {
      instance,
      window,
      logical_device,
    }
  }

  /// Create a Vulkan instance with optional debug extensions.
  fn create_instance(debug_enabled: bool) -> Arc<Instance> {
    let required_extensions = {
      let mut extensions = vulkano_win::required_extensions();

      // If debugging is enabled, add the debug utility extension
      if debug_enabled {
        warn!("Debug enabled");
        let debug_extensions = InstanceExtensions {
          ext_debug_utils: true,
          ..InstanceExtensions::none()
        };
        extensions = extensions.union(&debug_extensions);
      }

      extensions
    };

    // Enable layers
    let enabled_layers = {
      let mut output_layers: Vec<String> = vec![];

      // Ignore layers if not in debug mode
      if debug_enabled {
        // Print out available layers
        let mut available_layers = vulkano::instance::layers_list().unwrap();
        debug!("Available debugging layers:");
        while let Some(layer) = available_layers.next() {
          debug!("\t{}", layer.name());
        }

        // Os-specific layers
        #[cfg(not(target_os = "macos"))]
        output_layers.push("VK_LAYER_KHRONOS_validation".to_owned());
        #[cfg(target_os = "macos")]
        output_layers.push("VK_LAYER_KHRONOS_validation".to_owned());
      }

      output_layers
    };

    match Instance::new(InstanceCreateInfo {
      enabled_extensions: required_extensions,
      enabled_layers,
      ..Default::default()
    }) {
      Ok(instance) => instance,
      Err(e) => {
        panic!("Failed to create instance: {}", e);
      }
    }
  }

  /// Select a physical device to use. Returns the device and associated queue family.
  fn pick_physical_device(
    instance: &Arc<Instance>,
    surface: Arc<Surface<Window>>,
  ) -> (PhysicalDevice, QueueFamily) {
    let device_extensions = DeviceExtensions {
      khr_swapchain: true,
      ..DeviceExtensions::none()
    };

    let device_creation_results = PhysicalDevice::enumerate(&instance)
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
      });

    match device_creation_results {
      Some(value) => value,
      None => {
        panic!("Failed to find physical device and queue family.");
      }
    }
  }

  /// Create a Vulkan logical device and queue.
  fn create_logical_device(
    physical_device: PhysicalDevice,
    device_extensions: DeviceExtensions,
    queue_family: QueueFamily,
  ) -> (Arc<Device>, Arc<Queue>) {
    let device_creation_results = Device::new(
      physical_device,
      DeviceCreateInfo {
        enabled_extensions: physical_device
          .required_extensions()
          .union(&device_extensions),
        queue_create_infos: vec![QueueCreateInfo::family(queue_family)],
        ..Default::default()
      },
    );

    match device_creation_results {
      Ok(mut value) => {
        let device = value.0;
        let queues = value.1.next().unwrap();
        (device, queues)
      }
      Err(e) => {
        panic!("Failed to create logical device: {}", e);
      }
    }
  }

  fn create_swapchain() {}

  fn create_render_pass() {}

  fn self_create_fn_2() {}

  fn self_create_fn_3() {}
}
