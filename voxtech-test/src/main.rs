pub mod client;
pub mod server;
pub mod util;

fn main() -> util::StdResult<()> {
  let server = server::ServerHandler::new()?;
  winit::event_loop::EventLoop::new()?
    .run_app(&mut client::Client::default())?;

  Ok(())
}
