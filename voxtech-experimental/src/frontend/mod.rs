pub mod gfx;
pub mod window;

pub fn run() -> crate::common::StdResult<()> {
  tracing::info!("Initializing frondend initializing");
  let ev_loop = winit::event_loop::EventLoop::new()?;
  let mut app = window::App::default();
  ev_loop.run_app(&mut app)?;

  Ok(())
}
