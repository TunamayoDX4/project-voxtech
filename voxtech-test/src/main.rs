pub mod app;
pub mod back;
pub mod front;

pub mod config;
use config::APP_CONFIG;

pub mod util;

pub mod gfx;

pub mod game_logic;
pub mod game_logic_old;

fn main() {
  let ev_loop = winit::event_loop::EventLoop::new()
    .expect("event loop initialize failure");
  let mut app = app::AppCtx::new();
  ev_loop
    .run_app(&mut app)
    .expect("application running failure");
}
