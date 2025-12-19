//! チャンク描画関連

use std::collections::VecDeque;

use hashbrown::HashMap;

use crate::common::{BlockPos, Dir};

use super::{tile::OpaqueTileInstances, WGPUCtx};

pub mod uniform;

pub struct ChunkStorage {
  map: HashMap<BlockPos, ChunkObject>,
  mem: Vec<Option<ChunkObject>>,
  remque: VecDeque<u32>,
}
impl ChunkStorage {
  pub fn new() -> Self {
    Self {
      map: HashMap::new(),
      mem: Vec::new(),
      remque: VecDeque::new(),
    }
  }
}

pub struct ChunkObject {
  uniform: uniform::ChunkUniformInstance,
  opaque_tile:
    [OpaqueTileInstances; Dir::COUNT as usize],
}
impl ChunkObject {
  pub fn new(
    ctx: &WGPUCtx,
    uniform: uniform::ChunkUniform,
    layout: &uniform::ChunkUniformLayout,
    mut instance: [impl Iterator<Item = super::tile::types::BakedInstance>;
      Dir::COUNT as usize],
  ) -> Self {
    let uniform = uniform::ChunkUniformInstance::new(
      ctx, layout, uniform,
    );
    let opaque_tile = std::array::from_fn(|i| {
      OpaqueTileInstances::new(
        ctx,
        std::mem::replace(&mut instance[i], unsafe {
          std::mem::MaybeUninit::uninit().assume_init()
        }),
      )
    });
    Self {
      uniform,
      opaque_tile,
    }
  }

  pub fn write_instance(
    &mut self,
    instance: impl Iterator<
      Item = super::tile::types::BakedInstance,
    >,
    dir: Dir,
  ) {
    self.opaque_tile[dir as usize].write(instance);
  }
}
