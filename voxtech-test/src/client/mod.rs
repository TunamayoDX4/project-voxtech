use std::sync::Arc;
use winit::{
  application::ApplicationHandler,
  event::{
    DeviceEvent, ElementState, KeyEvent,
    WindowEvent,
  },
  keyboard::{KeyCode, PhysicalKey},
  window::{CursorGrabMode, Window},
};

pub mod config;
pub mod gfx;

pub mod world;

#[derive(Default)]
pub struct Client {
  input_locking: bool,
  window: Option<Arc<Window>>,
  gfx: Option<gfx::GfxCtx>,
  world: Option<world::World>,
}
impl ApplicationHandler for Client {
  fn resumed(
    &mut self,
    event_loop: &winit::event_loop::ActiveEventLoop,
  ) {
    let window_attr =
      winit::window::WindowAttributes::default(
      )
      .with_title("voxtech")
      .with_active(true)
      .with_inner_size(
        winit::dpi::PhysicalSize::new(
          config::COMMON_CLIENT_CFG
            .window
            .scale
            .0,
          config::COMMON_CLIENT_CFG
            .window
            .scale
            .1,
        ),
      );
    let window = event_loop
      .create_window(window_attr)
      .map(Arc::new)
      .expect(
        "window initialize process failure",
      );
    // カーソルを画面内にグラブするように設定する
    window
      .set_cursor_grab(
        winit::window::CursorGrabMode::Confined,
      )
      .expect(
        "cursor grabbing mode setting failure",
      );
    // カーソルを非表示とする
    window.set_cursor_visible(false);
    self.window = Some(window.clone());

    self.gfx = Some(
      pollster::block_on(gfx::GfxCtx::new(
        window,
      ))
      .expect("gfx module initialize failure"),
    );

    // ワールドを初期化する
    self.world = Some(world::World::default());
  }

  fn window_event(
    &mut self,
    event_loop: &winit::event_loop::ActiveEventLoop,
    window_id: winit::window::WindowId,
    event: WindowEvent,
  ) {
    // 既にwindowが作られていて、かつ
    // window idがメインウィンドウの物である場合にのみ処理を行う
    let Some(window) = self
      .window
      .as_ref()
      .filter(|w| w.id() == window_id)
    else {
      return;
    };

    match event {
      WindowEvent::CloseRequested => {
        event_loop.exit()
      }
      WindowEvent::RedrawRequested
        if let Some(gfx) =
          self.gfx.as_mut() =>
      {
        if let Some(world) = self.world.as_mut()
        {
          world.cycle_update();
          gfx.update(world);
        }

        gfx
          .rendering()
          .expect("rendering process failure");
        window.request_redraw();
      }
      WindowEvent::Resized(new_size)
        if let Some(gfx) =
          self.gfx.as_mut() =>
      {
        gfx.resize(new_size);
      }
      WindowEvent::KeyboardInput {
        event:
          KeyEvent {
            physical_key:
              PhysicalKey::Code(
                KeyCode::ControlLeft,
              )
              | PhysicalKey::Code(
                KeyCode::ControlRight,
              ),
            repeat: false,
            state,
            ..
          },
        ..
      } => {
        if state == ElementState::Pressed {
          self.input_locking = true;
          window
            .set_cursor_grab(
              CursorGrabMode::None,
            )
            .expect(
              "cursor grabbing release failure",
            );
          window.set_cursor_visible(true);
        } else {
          self.input_locking = false;
          window
            .set_cursor_grab(
              CursorGrabMode::Confined,
            )
            .expect(
              "cursor grabbing setting failure",
            );
          window.set_cursor_visible(false);
        }
      }
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
      // ユーザ入力に対する処理
      WindowEvent::KeyboardInput {
        event,
        ..
      } if !self.input_locking
        && let Some(world) =
          self.world.as_mut() =>
      {
        world.key_input(&event);
      }
      WindowEvent::MouseInput {
        state,
        button,
        ..
      } if let Some(world) =
        self.world.as_mut() =>
      {
        world.mouse_button_input(state, button)
      }

      _ => {}
    }
  }

  fn device_event(
    &mut self,
    _event_loop: &winit::event_loop::ActiveEventLoop,
    _device_id: winit::event::DeviceId,
    event: DeviceEvent,
  ) {
    match event {
      DeviceEvent::MouseMotion { delta }
        if !self.input_locking
          && let Some(world) =
            self.world.as_mut() =>
      {
        world.mouse_motion_input(delta);
      }
      _ => {}
    }
  }
}
