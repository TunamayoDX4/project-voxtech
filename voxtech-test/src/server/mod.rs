use std::thread::JoinHandle;

pub mod config;

pub struct ServerHandler {
  jh: JoinHandle<Result<(), &'static str>>,
}
impl ServerHandler {
  pub fn new() -> crate::util::StdResult<Self> {
    let server = Server::default();
    let jh =
      std::thread::spawn(move || server.run());

    Ok(Self { jh })
  }
}

pub struct Server {
  start_time: std::time::Instant,
}
impl Default for Server {
  fn default() -> Self {
    Self {
      start_time: std::time::Instant::now(),
    }
  }
}
impl Server {
  pub fn run(self) -> Result<(), &'static str> {
    for _ in 0..3000 {}
    Ok(())
  }
}
