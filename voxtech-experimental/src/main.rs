use std::sync::Arc;

use winit::{
  application::ApplicationHandler,
  event::{DeviceEvent, DeviceId, WindowEvent},
  event_loop::{
    ActiveEventLoop, ControlFlow, EventLoop,
  },
  window::{Window, WindowAttributes, WindowId},
};

pub mod aliases;
pub use aliases::*;
pub mod gfx;

pub mod control;
pub mod player;

pub mod types;

/// アプリケーション構造体
pub struct App {
  window: Option<Arc<Window>>,
  wgpu_ctx: Option<gfx::WGPUContext>,
  camera: Option<gfx::camera::CameraInstance>,
  world_renderer:
    Option<gfx::world_renderer::WorldRenderer>,
  block_renderer: Option<
    Vec<gfx::world_renderer::block_rdr::BlockRenderInstance,
  >>,
  user_input: control::UserControlInput,
  player: player::Player,
}
impl ApplicationHandler for App {
  fn resumed(&mut self, event_loop: &ActiveEventLoop) {
    // ウィンドウオブジェクトの初期化
    let window = event_loop
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
      .expect("Winit Window initialize failure");
    let window = Arc::new(window);
    match window.set_cursor_grab(
      winit::window::CursorGrabMode::Confined,
    ) {
      Ok(()) => window.set_cursor_visible(false),
      Err(e) => {
        eprintln!("cursor grabmode change error: {e}")
      }
    }
    self.window = Some(Arc::clone(&window));

    // カメラの初期化
    let camera = gfx::camera::CameraInstance {
      position: [0., 0., -5.].into(),
      velocity: [0., 0., 0.].into(),
      rotation:
        nalgebra::UnitQuaternion::from_axis_angle(
          &nalgebra::UnitVector3::new_normalize(
            nalgebra::Vector3::x(),
          ),
          0.,
        ),
    };

    // WGPUコンテキストの初期化
    let wgpu_ctx =
      pollster::block_on(gfx::WGPUContext::new(window))
        .expect("WGPU Context initialize failure");
    let world_renderer =
      gfx::world_renderer::WorldRenderer::new(
        &wgpu_ctx, &camera,
      )
      .expect("World renderer initialize failure");
    let block_renderer = [
      gfx::world_renderer::block_rdr::BlockRenderInstance::new(&wgpu_ctx)
    ].into();
    self.wgpu_ctx = Some(wgpu_ctx);
    self.world_renderer = Some(world_renderer);
    self.block_renderer = Some(block_renderer);
    self.camera = Some(camera);
  }

  fn window_event(
    &mut self,
    event_loop: &ActiveEventLoop,
    _window_id: WindowId,
    event: WindowEvent,
  ) {
    let Some(wgpu_ctx) = self.wgpu_ctx.as_mut() else {
      return;
    };
    match event {
      // 再描画処理
      WindowEvent::RedrawRequested => {
        if let Some(world_renderer) =
          self.world_renderer.as_mut()
        {
          if let Some(camera) = self.camera.as_mut() {
            self
              .player
              .update(&self.user_input);
            self.user_input.update();
            self
              .player
              .update_camera(camera);
            world_renderer
              .update_camera(&wgpu_ctx, &camera);
          }
          let Some(block_rdr) =
            self.block_renderer.as_ref()
          else {
            return;
          };
          match wgpu_ctx
            .rendering(world_renderer, &block_rdr)
          {
            Ok(_) => {}
            Err(wgpu::SurfaceError::Lost) => {
              wgpu_ctx.reconfigure()
            }
            Err(wgpu::SurfaceError::OutOfMemory) => {
              event_loop.exit()
            }
            Err(e) => eprintln!("Error occured: {e}"),
          }
        }
      }

      // ウィンドウのリサイズ処理
      WindowEvent::Resized(_) => wgpu_ctx.resize(),

      // ウィンドウを閉じる要求が来た時の処理
      WindowEvent::CloseRequested => event_loop.exit(),

      // キーボード入力処理
      WindowEvent::KeyboardInput { event, .. } => {
        if let Some(window) = self.window.as_ref() {
          self
            .user_input
            .key_input(&event, window);
        }
      }
      _ => {}
    }
  }

  fn device_event(
    &mut self,
    _event_loop: &ActiveEventLoop,
    _device_id: DeviceId,
    event: DeviceEvent,
  ) {
    match event {
      // マウス入力処理
      DeviceEvent::MouseMotion { delta } => {
        self
          .user_input
          .mouse_input([delta.0, delta.1]);
      }
      _ => {}
    }
  }
}

fn main() {
  let event_loop = EventLoop::new()
    .expect("Winit eventloop initialize failure");
  event_loop.set_control_flow(ControlFlow::Poll);
  let mut app = App {
    window: None,
    wgpu_ctx: None,
    world_renderer: None,
    block_renderer: None,
    camera: None,
    user_input: control::UserControlInput::new(),
    player: player::Player::new(),
  };
  event_loop
    .run_app(&mut app)
    .expect("Error occured");
}
