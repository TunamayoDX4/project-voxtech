use std::sync::atomic::{AtomicBool, Ordering};

use crossbeam::{
  channel::{Receiver, RecvError, Sender, bounded},
  sync::{Parker, Unparker},
};

static NOW_RENDERING: AtomicBool =
  AtomicBool::new(false);

#[derive(Debug, Clone, Copy)]
pub enum RenderCommand {
  /// 再描画
  Redraw,

  /// リサイズ
  Resize,

  /// 終了指示
  Exit,
}

#[derive(Debug)]
pub enum RenderingError {
  /// メモリ不足
  OutOfMemory,
}
impl std::fmt::Display for RenderingError {
  fn fmt(
    &self,
    f: &mut std::fmt::Formatter<'_>,
  ) -> std::fmt::Result {
    <Self as std::fmt::Debug>::fmt(&self, f)
  }
}
impl std::error::Error for RenderingError {}

#[derive(Debug)]
pub enum RenderingSuccess {
  /// 終了指示を受けた
  StopRequested,

  /// 何もしない
  Nop,
}

/// コマンド送信モジュール
pub(super) struct RenderCommandSend {
  tx: Sender<RenderCommand>,
  rendering_now_parker: Parker,
}
impl RenderCommandSend {
  pub fn new() -> (Self, RenderCommandRecv) {
    let (tx, rx) = bounded(1);
    let rendering_now_parker = Parker::new();
    let rendering_now_unparker = rendering_now_parker
      .unparker()
      .clone();
    let sender = RenderCommandSend {
      tx,
      rendering_now_parker,
    };
    let receiver = RenderCommandRecv {
      rx,
      rendering_now_unparker,
    };
    (sender, receiver)
  }

  pub fn command_send(&self, command: RenderCommand) {
    if NOW_RENDERING.load(Ordering::Acquire) {
      tracing::trace!(
        "Rendering process is not complete"
      );
      tracing::debug!("Wait render process complete");
      self.rendering_now_parker.park();
    }
    NOW_RENDERING.store(true, Ordering::Release);
    match self.tx.send(command) {
      Ok(_) => {}
      Err(_) => {
        tracing::warn!("Renderer thread finished.")
      }
    }
  }
}
impl Drop for RenderCommandSend {
  fn drop(&mut self) {
    match self
      .tx
      .send(RenderCommand::Exit)
    {
      Ok(_) => {}
      Err(_) => {
        tracing::warn!("Renderer thread finished.")
      }
    }
  }
}

/// 描画リクエストの受信モジュール
pub(super) struct RenderCommandRecv {
  rx: Receiver<RenderCommand>,
  rendering_now_unparker: Unparker,
}
impl RenderCommandRecv {
  pub fn wait_and_rendering(
    &self,
    f: impl FnOnce(
      RenderCommand,
    ) -> Result<
      RenderingSuccess,
      RenderingError,
    >,
  ) -> Result<RenderingSuccess, RenderingError> {
    let res = match self.rx.recv() {
      Ok(comm) => f(comm),
      Err(RecvError) => {
        Ok(RenderingSuccess::StopRequested)
      }
    };
    NOW_RENDERING.store(false, Ordering::Release);
    self
      .rendering_now_unparker
      .unpark();
    tracing::debug!("Rendering process finished.");
    res
  }
}
