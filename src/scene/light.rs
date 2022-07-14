use crate::{config::MAX_LIGHTS, resources::MdrColor, scene::transform::MdrTranslation};

#[derive(Clone, Copy)]
pub struct MdrLight {
  pub color: MdrColor,
  pub brightness: f32,

  pub translation: MdrTranslation,
}

impl MdrLight {
  pub fn new(r: f32, g: f32, b: f32, brightness: f32) -> Self {
    Self {
      color: MdrColor { r, g, b },
      brightness,

      translation: MdrTranslation::identity(),
    }
  }

  pub fn white(brightness: f32) -> Self {
    Self::new(1.0, 1.0, 1.0, brightness)
  }

  pub const fn unused() -> Self {
    Self {
      color: MdrColor {
        r: 0.0,
        g: 0.0,
        b: 0.0,
      },
      brightness: 0.0,
      translation: MdrTranslation::identity(),
    }
  }
}

pub struct MdrLightSet {
  lights: Vec<MdrLight>,
  light_count: usize,
}

impl MdrLightSet {
  pub fn new() -> Self {
    Self {
      lights: Vec::<MdrLight>::with_capacity(MAX_LIGHTS),
      light_count: 0,
    }
  }

  pub fn add_light(&mut self, light: MdrLight) {
    if self.light_count == MAX_LIGHTS {
      panic!(
        "You added more than {} lights and now everything broke, be careful.",
        MAX_LIGHTS
      )
    }

    self.lights.push(light);
    self.light_count += 1;
  }

  pub fn remove_light(&mut self, light_index: usize) {
    assert!(light_index < MAX_LIGHTS);

    self.lights.remove(light_index);
    self.light_count -= 1;
  }

  pub fn get_light(&self, light_index: usize) -> Option<&MdrLight> {
    self.lights.get(light_index)
  }

  pub fn get_light_mut(&mut self, light_index: usize) -> Option<&mut MdrLight> {
    self.lights.get_mut(light_index)
  }

  pub fn get_light_array(&self) -> [MdrLight; MAX_LIGHTS] {
    let mut light_array = [MdrLight::unused(); MAX_LIGHTS];
    for i in 0..self.light_count {
      light_array[i] = self.lights[i];
    }

    light_array
  }

  pub fn get_count(&self) -> u32 {
    self.light_count as u32
  }
}
