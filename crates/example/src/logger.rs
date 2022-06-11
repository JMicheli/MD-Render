use log::debug;
pub use log::SetLoggerError;

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

pub fn init() -> Result<(), SetLoggerError> {
  let result = log::set_logger(&LOGGER).map(|()| log::set_max_level(log::LevelFilter::Debug));
  debug!("Initialized logger");

  result
}
