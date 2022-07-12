use std::path::Path;

// Build debug configuration
#[cfg(debug_assertions)]
pub const MDR_LOG_LEVEL: &str = "debug";
#[cfg(not(debug_assertions))]
pub const MDR_LOG_LEVEL: &str = "info";
#[cfg(debug_assertions)]
pub const DEBUG_ENABLED: bool = true;
#[cfg(not(debug_assertions))]
pub const DEBUG_ENABLED: bool = false;

// Asset handling
#[cfg(debug_assertions)]
const ASSET_PREFIX: &str = "examples/basic/assets/";
#[cfg(not(debug_assertions))]
const ASSET_PREFIX: &str = "assets/";

pub fn asset(asset_path: &str) -> String {
  let asset_path_prefix = Path::new(ASSET_PREFIX);
  asset_path_prefix
    .join(Path::new(asset_path))
    .to_str()
    .unwrap()
    .to_string()
}
