/*
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

pub mod frontend;

pub mod world_instance;

pub mod control;
pub mod player;

pub mod common;

pub mod types;

/// アプリケーション構造体
pub struct App {
  window: Option<Arc<Window>>,
  gfx: Option<gfx::GfxBundle>,
  world: Option<world_instance::WorldInstance>,
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

    self.world =
      Some(world_instance::WorldInstance::new());

    let mut gfx =
      pollster::block_on(gfx::GfxBundle::new(window))
        .expect(
          "Graphics Bundle Module initialize failure",
        );
    gfx.world_init(
      &self.player_camera_cfg,
      &self.player_camera,
    );
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

        // ワールドの描画・更新
        if let Some(world) = self.world.as_mut() {
          world.visibility_update(
            &self.player_camera.position,
            gfx,
          );
          world.rendering(gfx);
        }

        gfx.world_modify(|ctx, w| {
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
        if let Some(world) = self.world.as_mut() {
          if !event.state.is_pressed() {
            return;
          }
          match event.physical_key {
            winit::keyboard::PhysicalKey::Code(kc) => {
              match kc {
                winit::keyboard::KeyCode::ArrowLeft => {}
                winit::keyboard::KeyCode::ArrowRight => {}
                winit::keyboard::KeyCode::ArrowUp => {}
                winit::keyboard::KeyCode::ArrowDown => {}
                winit::keyboard::KeyCode::KeyT => {}
                winit::keyboard::KeyCode::KeyB => {}
                _ => {
                  return;
                }
              }
              let Some(region) =
                world.world.dim.get_mut(
                  &crate::common::BlockPos::new(
                    0, 0, -256,
                  ),
                )
              else {
                return;
              };
              let sector = &mut region.0.sector.write();
              let sector =
                &sector.as_mut().unwrap()[48];
              let mut chunk = sector.chunk.write();
              let p = 48;
              let cells = chunk.as_mut().unwrap()[p]
                .cell
                .as_mut()
                .unwrap();
              let mut info = sector.chunk_info.write();
              cells[p] = match kc {
                winit::keyboard::KeyCode::ArrowLeft => {
                  cells[p].rotate_west()
                }
                winit::keyboard::KeyCode::ArrowRight => {
                  cells[p].rotate_east()
                }
                winit::keyboard::KeyCode::ArrowUp => {
                  cells[p].rotate_top()
                }
                winit::keyboard::KeyCode::ArrowDown => {
                  cells[p].rotate_bottom()
                }
                winit::keyboard::KeyCode::KeyT => {
                  cells[p].rotate_north()
                }
                winit::keyboard::KeyCode::KeyB => {
                  cells[p].rotate_south()
                }
                _ => {
                  return;
                }
              };
              (0..6).for_each(|i| {
                info[p].dirty_opq_tile[i] = true
              });
            }
            _ => {}
          }
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
    world: None,
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
        near: 0.1,
        far: 10000.,
      },
  };
  event_loop
    .run_app(&mut app)
    .expect("Error occured");
}

*/

mod common;
use common::*;
use tracing_subscriber::prelude::*;
mod frontend;

fn main() -> StdResult<()> {
  let _guard = init_tracing_subscriber();
  frontend::run()
}

struct LogWorkerGuard {
  _file_appender:
    tracing_appender::non_blocking::WorkerGuard,
}

fn init_tracing_subscriber() -> LogWorkerGuard {
  let time = std::time::SystemTime::now()
    .duration_since(std::time::UNIX_EPOCH)
    .unwrap();
  let file_appender = tracing_appender::rolling::never(
    "./log",
    format!(
      "voxtech-applog-{}.log",
      time.as_secs()
    ),
  );
  let (nb, file_appender) =
    tracing_appender::non_blocking(file_appender);
  let logfile_layer = tracing_subscriber::fmt::layer()
    .with_writer(nb)
    .with_ansi(false)
    .with_line_number(true)
    .with_thread_ids(true)
    .with_thread_names(true)
    .with_filter(if cfg!(debug_assertions) {
      tracing_subscriber::filter::LevelFilter::DEBUG
    } else {
      tracing_subscriber::filter::LevelFilter::INFO
    });
  let stdout_layer = tracing_subscriber::fmt::layer()
    .compact()
    .with_thread_ids(true)
    .with_thread_names(true)
    .with_filter(
      tracing_subscriber::filter::LevelFilter::INFO,
    );
  tracing_subscriber::registry()
    .with(logfile_layer)
    .with(stdout_layer)
    .init();

  LogWorkerGuard {
    _file_appender: file_appender,
  }
}
