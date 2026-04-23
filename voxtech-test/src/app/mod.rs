use std::sync::Arc;

use winit::{
  application::ApplicationHandler,
  dpi::PhysicalSize,
  event::{ElementState, KeyEvent, WindowEvent},
  keyboard::{KeyCode, PhysicalKey},
  window::{CursorGrabMode, Window},
};

use crate::{back::BackEnd, front::FrontEnd};

pub struct AppCtx {
  window_handler: Option<Arc<Window>>,
  frontend: FrontEnd,
  backend: BackEnd,
}
impl AppCtx {
  #[allow(clippy::new_without_default)]
  pub fn new() -> Self {
    let window_handler = None;
    let frontend = FrontEnd::new();
    let backend = BackEnd::new();
    Self {
      window_handler,
      frontend,
      backend,
    }
  }
}
impl ApplicationHandler for AppCtx {
  fn resumed(
    &mut self,
    event_loop: &winit::event_loop::ActiveEventLoop,
  ) {
    let window_attributes =
      winit::window::WindowAttributes::default()
        .with_active(true)
        .with_resizable(
          crate::APP_CONFIG
            .window
            .allow_resize,
        )
        .with_inner_size(PhysicalSize::new(
          crate::APP_CONFIG.window.size.0,
          crate::APP_CONFIG.window.size.1,
        ))
        .with_enabled_buttons(
          winit::window::WindowButtons::MINIMIZE
            | winit::window::WindowButtons::CLOSE,
        );
    let window = event_loop
      .create_window(window_attributes)
      .expect("failed create window");
    let window = Arc::new(window);
    window
      .set_cursor_grab(CursorGrabMode::Confined)
      .expect("set cursor grab config failure");
    window.set_cursor_visible(false);

    // フロントエンドに対するウィンドウ初期化処理通知
    self
      .frontend
      .window_create(Arc::clone(&window))
      .expect(
        "failed window create process in frontend.",
      );
    self.window_handler = Some(window);
  }

  fn window_event(
    &mut self,
    event_loop: &winit::event_loop::ActiveEventLoop,
    window_id: winit::window::WindowId,
    event: WindowEvent,
  ) {
    let window = self
      .window_handler
      .as_ref()
      .filter(|w| w.id() == window_id);
    if let WindowEvent::CloseRequested = event {
      event_loop.exit();
    } else if let Some(window) = window {
      match event {
        WindowEvent::RedrawRequested => {
          // バックエンドの更新処理

          self
            .backend
            .frontend_update(&mut self.frontend);

          if let Err(e) = self.frontend.rendering() {
            eprintln!(
              "uncoverable error occured. in rendering."
            );
            eprintln!("{e}");
            event_loop.exit();
          } else {
            window.request_redraw();
            self.backend.update();
          }
        }

        // リサイズ処理
        WindowEvent::Resized(new_size) => self
          .frontend
          .window_resize(new_size)
          .expect("Window resize appling failure."),

        // Escapeが入力された場合はプログラムを終了する
        WindowEvent::KeyboardInput {
          event:
            KeyEvent {
              physical_key:
                PhysicalKey::Code(KeyCode::Escape),
              state: ElementState::Pressed,
              ..
            },
          ..
        } => event_loop.exit(),

        // 左Ctrlキーが入力されている場合はカーソルのグラブを外す
        WindowEvent::KeyboardInput {
          event:
            KeyEvent {
              physical_key:
                PhysicalKey::Code(KeyCode::ControlLeft),
              state,
              ..
            },
          ..
        } => {
          if state == ElementState::Pressed {
            window
              .set_cursor_grab(CursorGrabMode::None)
              .expect("set cursor grab config failure");
            window.set_cursor_visible(true);
          } else {
            window
              .set_cursor_grab(CursorGrabMode::Confined)
              .expect("set cursor grab config failure");
            window.set_cursor_visible(false);
          }
        }

        // それ以外のキー入力をバックエンドに送る
        WindowEvent::KeyboardInput {
          event, ..
        } => self
          .backend
          .keyboard_input(event),

        // マウスボタン入力をバックエンドに送る
        WindowEvent::MouseInput {
          state,
          button,
          ..
        } => self
          .backend
          .mouse_button_input(button, state),

        // CloseRequestedは前のコードで回収されているはずなので、
        // 論理的にはこのコードは実行されない。
        WindowEvent::CloseRequested => unreachable!(),
        _ => {}
      }
    }
  }

  fn device_event(
    &mut self,
    _event_loop: &winit::event_loop::ActiveEventLoop,
    _device_id: winit::event::DeviceId,
    event: winit::event::DeviceEvent,
  ) {
    match event {
      winit::event::DeviceEvent::MouseMotion {
        delta,
      } => self
        .backend
        .mouse_motion_input(delta),
      _ => {}
    }
  }
}
