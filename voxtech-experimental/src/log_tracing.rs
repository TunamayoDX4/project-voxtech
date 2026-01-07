use tracing_subscriber::prelude::*;

pub struct LogWorkerGuard {
  _file_appender:
    tracing_appender::non_blocking::WorkerGuard,
}

pub fn init_tracing_subscriber() -> LogWorkerGuard {
  let time = std::time::SystemTime::now()
    .duration_since(std::time::UNIX_EPOCH)
    .unwrap();
  let file_appender = tracing_appender::rolling::never(
    "./log",
    format!(
      "voxtech-applog-{}.log",
      time.as_secs()
    ),
  );
  let (nb, file_appender) =
    tracing_appender::non_blocking(file_appender);
  let logfile_layer = tracing_subscriber::fmt::layer()
    .with_writer(nb)
    .with_ansi(false)
    .with_line_number(true)
    .with_thread_ids(true)
    .with_thread_names(true)
    .with_filter(if cfg!(debug_assertions) {
      tracing_subscriber::filter::LevelFilter::DEBUG
    } else {
      tracing_subscriber::filter::LevelFilter::INFO
    });
  let stdout_layer = tracing_subscriber::fmt::layer()
    .compact()
    .with_thread_ids(true)
    .with_thread_names(true)
    .with_filter(
      tracing_subscriber::filter::LevelFilter::INFO,
    );
  tracing_subscriber::registry()
    .with(logfile_layer)
    .with(stdout_layer)
    .init();

  LogWorkerGuard {
    _file_appender: file_appender,
  }
}
