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
pub mod world;

pub mod common;

pub mod types;

/// アプリケーション構造体
pub struct App {
  window: Option<Arc<Window>>,
  gfx: Option<gfx::GfxBundle>,
  user_input: control::UserControlInput,
  player: player::Player,
  player_camera: gfx::world::camera3d::Camera3DInstance,
  player_camera_cfg:
    gfx::world::camera3d::Camera3DConfig,
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

    let mut gfx =
      pollster::block_on(gfx::GfxBundle::new(window))
        .expect(
          "Graphics Bundle Module initialize failure",
        );
    gfx.world_init(
      &self.player_camera_cfg,
      &self.player_camera,
    );
    gfx.world(|ctx, w| {
      w.chunk.insert(
        common::BlockPos::new(0, 0, 0),
        || {
          gfx::world::chunk::ChunkObject::new(
            ctx,
            common::BlockPos::new(0, 0, 0).into(),
            &w.chunk_layout,
            std::array::from_fn(|_| {
              (0..1088).map(|i| {
                gfx::world::tile::types::BakedInstance {
                  stride: i,
                  tex_pos: [0., 0.],
                  tex_scale: [0., 0.],
                }
              })
            }),
          )
        },
      );

      w.chunk.insert(
        common::BlockPos::new(16, 0, 0),
        || {
          gfx::world::chunk::ChunkObject::new(
            ctx,
            common::BlockPos::new(16, 0, 0).into(),
            &w.chunk_layout,
            std::array::from_fn(|_| {
              (0..1088).map(|i| {
                gfx::world::tile::types::BakedInstance {
                  stride: i,
                  tex_pos: [0., 0.],
                  tex_scale: [0., 0.],
                }
              })
            }),
          )
        },
      );
    });
    self.gfx = Some(gfx);
  }

  fn window_event(
    &mut self,
    event_loop: &ActiveEventLoop,
    _window_id: WindowId,
    event: WindowEvent,
  ) {
    let Some(gfx) = self.gfx.as_mut() else {
      return;
    };
    match event {
      // 再描画処理
      WindowEvent::RedrawRequested => {
        // プレイヤーのビューの更新
        self
          .player
          .update(&self.user_input);
        self.user_input.update();
        self
          .player
          .update_camera(&mut self.player_camera);
        gfx.world(|ctx, w| {
          w.update(
            ctx,
            &self.player_camera_cfg,
            &self.player_camera,
          )
        });

        // GFXバンドル構造体を呼び出し、描画する。
        match gfx.rendering() {
          Ok(_) => {}
          Err(wgpu::SurfaceError::Lost) => {
            gfx.reconfigure();
          }
          Err(wgpu::SurfaceError::OutOfMemory) => {
            event_loop.exit()
          }
          Err(e) => eprintln!("Error occured: {e}"),
        }
      }

      // ウィンドウのリサイズ処理
      WindowEvent::Resized(_) => {
        gfx.resize();
      }

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
    gfx: None,
    user_input: control::UserControlInput::new(),
    player: player::Player::new(),
    player_camera:
      gfx::world::camera3d::Camera3DInstance {
        position: [0., 0., 0.].into(),
        velocity: [0., 0., 0.].into(),
        rotation:
          nalgebra::UnitQuaternion::from_axis_angle(
            &nalgebra::UnitVector3::new_normalize(
              nalgebra::Vector3::z(),
            ),
            0.,
          ),
      },
    player_camera_cfg:
      gfx::world::camera3d::Camera3DConfig {
        fovy: 45. * std::f64::consts::PI / 180.,
        near: 0.5,
        far: 10000.,
      },
  };
  event_loop
    .run_app(&mut app)
    .expect("Error occured");
}
