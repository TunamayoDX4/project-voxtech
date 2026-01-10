use std::sync::Arc;
use winit::{
  application::ApplicationHandler,
  event::WindowEvent,
  window::{Window, WindowAttributes},
};

use super::gfx;

pub struct AppWindow {
  window: Arc<Window>,
  gfx: gfx::GfxHandler,
}

pub struct App {
  window: Option<AppWindow>,
}
impl Default for App {
  fn default() -> Self {
    Self { window: None }
  }
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

      let gfx = tracing::info_span!("gfx init").in_scope(
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
      }
    });
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
        }
      }
      WindowEvent::Resized(_) => {
        if let Some(w) = self.window.as_mut() {
          w.gfx.resized();
        }
      }
      WindowEvent::CloseRequested => {
        tracing::info!("Receive close request.");
        if let Some(w) = self.window.take() {
          w.gfx.stop().unwrap().unwrap();
        };
        event_loop.exit()
      }
      _ => {}
    }
  }
}
