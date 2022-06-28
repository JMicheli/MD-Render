use std::env;

pub use log::SetLoggerError;
use log::{debug, LevelFilter};

static LOGGER: MdrLogger = MdrLogger;

pub struct MdrLogger;

impl log::Log for MdrLogger {
  fn enabled(&self, metadata: &log::Metadata) -> bool {
    metadata.level() <= log::Level::Debug
  }

  fn log(&self, record: &log::Record) {
    if self.enabled(record.metadata()) {
      println!("{} - {}", record.level(), record.args());
    }
  }

  fn flush(&self) {}
}

pub fn init(max_level: LevelFilter) -> Result<(), SetLoggerError> {
  let result = log::set_logger(&LOGGER).map(|()| log::set_max_level(max_level));
  debug!("Initialized logger");

  result
}

pub fn init_from_env() -> Result<(), SetLoggerError> {
  // Get log level from cargo profile (release or debug)
  let mdr_log_level = get_env_var("MDR_LOG_LEVEL", "_");
  let max_level = match mdr_log_level.as_str() {
    "error" => LevelFilter::Error,
    "warn" => LevelFilter::Warn,
    "info" => LevelFilter::Info,
    "debug" => LevelFilter::Debug,
    "trace" => LevelFilter::Trace,
    _ => LevelFilter::Info,
  };

  init(max_level)
}

fn get_env_var(name: &str, default: &str) -> String {
  env::var(name).unwrap_or(default.to_string()).to_lowercase()
}
