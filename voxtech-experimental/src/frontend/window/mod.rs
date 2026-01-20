use std::sync::Arc;
use winit::{
  application::ApplicationHandler,
  event::WindowEvent,
  window::{Window, WindowAttributes},
};

pub mod player;

use super::gfx;

pub struct AppWindow {
  window: Arc<Window>,
  gfx: gfx::GfxHandler,
  rdr_handler: gfx::renderer::WorldRendererHandler,
  player: player::PlayerState,
}

#[derive(Default)]
pub struct App {
  window: Option<AppWindow>,
}
impl ApplicationHandler for App {
  fn resumed(
    &mut self,
    event_loop: &winit::event_loop::ActiveEventLoop,
  ) {
    let app_window = tracing::info_span!(
      "voxtech app init"
    ).in_scope(|| {
      tracing::info!("starting app initialize");
      let window = tracing::info_span!("window init").in_scope(
        || {
          tracing::info!("starting window init");
          match event_loop
            .create_window(
              WindowAttributes::default()
                .with_active(true)
                .with_inner_size(
                  winit::dpi::PhysicalSize::new(1280, 720),
                )
                .with_enabled_buttons(
                  winit::window::WindowButtons::CLOSE
                    | winit::window::WindowButtons::MINIMIZE,
                ),
            )
          {
            Ok(window) => {
              tracing::info!("window init succeed");
              Arc::new(window)
            },
            Err(e) => {
              tracing::error_span!(
                "show detail: winit window init fail"
              ).in_scope(|| {
                tracing::error!("winit window init failure.");
                tracing::error!("winit error: {e}");
                eprintln!("winit initialize error.");
                eprintln!("error: {e}");
              });
              panic!("winit initialize error");
            }
          }
        }
      );

      let (gfx, rdr_handler) = tracing::info_span!("gfx init").in_scope(
        || {
          tracing::info!("starting gfx initialize");
          match pollster::block_on(
            gfx::GfxHandler::new(Arc::clone(&window))
          ) {
            Ok(gfx) => {
              gfx
            }
            Err(e) => {
              tracing::error_span!(
                "show detail: gfx init fail"
              ).in_scope(|| {
                tracing::error!("gfx init failure.");
                tracing::error!("gfx error: {e}");
                eprintln!("gfx initialize error.");
                eprintln!("error: {e}");
              });
              panic!("gfx initialize error");
            }
          }
        }
      );
      tracing::info!("voxtech app init succeed");

      AppWindow {
        window,
        gfx,
        rdr_handler,
        player: player::PlayerState::default(),
      }
    });
    /*
    app_window
      .window
      .set_cursor_grab(
        winit::window::CursorGrabMode::Confined,
      )
      .expect("mouse cursor mode setting failure");
    app_window
      .window
      .set_cursor_visible(false);
    */
    self.window = Some(app_window);
  }

  fn window_event(
    &mut self,
    event_loop: &winit::event_loop::ActiveEventLoop,
    _window_id: winit::window::WindowId,
    event: winit::event::WindowEvent,
  ) {
    match event {
      WindowEvent::RedrawRequested => {
        if let Some(w) = self.window.as_mut() {
          w.gfx.rendering();
          w.window.request_redraw();
          w.player.update();
          if let Err(e) = w.rdr_handler.channel.send((
            [
              w.player.phys.yaw,
              w.player.phys.pitch,
            ],
            w.player.phys.pos,
          )) {
            tracing::warn!(
              "Physics update sending error: {e}"
            );
          }
        }
      }
      WindowEvent::Resized(new_size) => {
        if let Some(w) = self.window.as_mut() {
          w.gfx.resize(new_size);
        }
      }
      WindowEvent::CloseRequested => {
        tracing::info!("Receive close request.");
        if let Some(w) = self.window.take() {
          w.gfx.stop().unwrap().unwrap();
        };
        event_loop.exit()
      }
      WindowEvent::KeyboardInput {
        device_id: _,
        event,
        is_synthetic: _,
      } => {
        if let winit::keyboard::PhysicalKey::Code(
          winit::keyboard::KeyCode::Escape,
        ) = event.physical_key
        {
          self
            .window
            .take()
            .unwrap()
            .gfx
            .stop()
            .unwrap()
            .unwrap();
          event_loop.exit();
        } else if let Some(w) = self.window.as_mut() {
          w.player.input(event);
        }
      }
      _ => {}
    }
  }

  fn device_event(
    &mut self,
    event_loop: &winit::event_loop::ActiveEventLoop,
    device_id: winit::event::DeviceId,
    event: winit::event::DeviceEvent,
  ) {
    match event {
      winit::event::DeviceEvent::MouseMotion {
        delta,
      } => {
        if let Some(w) = self.window.as_mut() {
          w.player.phys.mouse_input(delta);
        }
      }
      winit::event::DeviceEvent::Key(_) => {}
      _ => {}
    }
  }
}
