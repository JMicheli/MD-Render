mod logger;

use mdr_engine::{MdrEngine, MdrEngineOptions};

fn main() {
  logger::init().expect("Failed to initialize logger");

  let opts = MdrEngineOptions { debug: true };
  let engine = MdrEngine::new(opts);
}
