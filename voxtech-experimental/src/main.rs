//! # Voxtech Experimental
//! VoxTechの試験的実装
//!
//! ## 目的
//! 3DサンドボックスゲームのVoxTechの実装に向けた検証・調査

mod common;
use common::*;
mod frontend;
mod log_tracing;

fn main() -> StdResult<()> {
  let _guard = log_tracing::init_tracing_subscriber();
  frontend::run()
}
