//! チャンク描画関連

use std::{collections::VecDeque, num::NonZero};

use hashbrown::{hash_map::Entry, HashMap};

use crate::common::{BlockPos, Dir};

use super::{tile::OpaqueTileInstances, WGPUCtx};

pub mod uniform;

#[repr(C, align(8))]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct ChunkStorageKey {
  /// チャンクストレージ上でのインデックスを格納するキー
  key: u32,

  /// キーが同じチャンクの世代
  generation: NonZero<u32>,
}

pub struct ChunkStorage {
  map: HashMap<BlockPos, u32>,
  pos: Vec<Option<BlockPos>>,
  generation: Vec<NonZero<u32>>,
  mem: Vec<Option<ChunkObject>>,
  remque: VecDeque<u32>,
}
impl ChunkStorage {
  pub fn new() -> Self {
    Self {
      map: HashMap::new(),
      pos: Vec::new(),
      generation: Vec::new(),
      mem: Vec::new(),
      remque: VecDeque::new(),
    }
  }

  pub fn insert(
    &mut self,
    pos: BlockPos,
    obj: impl FnOnce() -> ChunkObject,
  ) -> Option<ChunkStorageKey> {
    let Entry::Vacant(ve) = self.map.entry(pos) else {
      // エントリーがすでにある場合はインサート失敗
      return None;
    };

    // インサート処理
    let key = match self.remque.pop_front() {
      Some(idx) => {
        let generation = NonZero::new(
          self.generation[idx as usize]
            .get()
            .wrapping_add(1),
        )
        .unwrap_or(NonZero::new(1).unwrap());
        self.pos[idx as usize] = Some(pos);
        self.generation[idx as usize] = generation;
        self.mem[idx as usize] = Some(obj());

        ChunkStorageKey {
          key: idx,
          generation,
        }
      }
      None => {
        let idx = self.mem.len() as u32;
        let generation = NonZero::new(1).unwrap();
        self.pos.push(Some(pos));
        self.generation.push(generation);
        self.mem.push(Some(obj()));

        ChunkStorageKey {
          key: idx,
          generation,
        }
      }
    };

    ve.insert(key.key);
    Some(key)
  }

  pub fn remove(
    &mut self,
    key: ChunkStorageKey,
  ) -> Option<(BlockPos, ChunkObject)> {
    if self
      .generation
      .get(key.key as usize)
      .copied()?
      != key.generation
    {
      return None;
    }

    let pos = self
      .pos
      .get_mut(key.key as usize)?;
    if pos.is_none() {
      return None;
    };
    self.map.remove(&pos.unwrap())?;
    let pos = pos.take().unwrap();
    let obj = self.mem[key.key as usize]
      .take()
      .unwrap();

    Some((pos, obj))
  }

  #[inline]
  pub fn get_by_key(
    &self,
    key: ChunkStorageKey,
  ) -> Option<&ChunkObject> {
    (self
      .generation
      .get(key.key as usize)
      .copied()?
      == key.generation)
      .then(|| self.mem[key.key as usize].as_ref())
      .flatten()
  }

  #[inline]
  pub fn get_mut_by_key(
    &mut self,
    key: ChunkStorageKey,
  ) -> Option<&mut ChunkObject> {
    (self
      .generation
      .get(key.key as usize)
      .copied()?
      == key.generation)
      .then(|| self.mem[key.key as usize].as_mut())
      .flatten()
  }

  #[inline]
  pub fn get_pos_by_key(
    &self,
    key: ChunkStorageKey,
  ) -> Option<&BlockPos> {
    (self
      .generation
      .get(key.key as usize)
      .copied()?
      == key.generation)
      .then(|| self.pos[key.key as usize].as_ref())
      .flatten()
  }

  #[inline]
  pub fn get_by_pos(
    &self,
    pos: &BlockPos,
  ) -> Option<(ChunkStorageKey, &ChunkObject)> {
    let key = self.map.get(pos).copied()?;
    let generation = self.generation[key as usize];
    let mem = self.mem[key as usize]
      .as_ref()
      .unwrap();
    Some((
      ChunkStorageKey { key, generation },
      mem,
    ))
  }

  #[inline]
  pub fn get_mut_by_pos(
    &mut self,
    pos: &BlockPos,
  ) -> Option<(
    ChunkStorageKey,
    &mut ChunkObject,
  )> {
    let key = self.map.get(pos).copied()?;
    let generation = self.generation[key as usize];
    let mem = self.mem[key as usize]
      .as_mut()
      .unwrap();
    Some((
      ChunkStorageKey { key, generation },
      mem,
    ))
  }

  #[inline]
  pub fn rendering(
    &self,
    rpass: &mut wgpu::RenderPass,
    dir: Dir,
  ) {
    for chunk in self
      .mem
      .iter()
      .filter_map(|c| c.as_ref())
      .filter(|c| c.opaque_tile[dir as usize].is_some())
    {
      chunk.uniform.rendering(rpass);
      if let Some(instance_len) = chunk.opaque_tile
        [dir as usize]
        .as_ref()
        .unwrap()
        .rendering(rpass)
      {
        rpass.draw_indexed(
          0..super::tile::types::TILE_INDICES.len()
            as _,
          0,
          0..instance_len.get(),
        );
      }
    }
  }
}

pub struct ChunkObject {
  uniform: uniform::ChunkUniformInstance,
  pub opaque_tile:
    [Option<OpaqueTileInstances>; Dir::COUNT as usize],
}
impl ChunkObject {
  pub fn new(
    ctx: &WGPUCtx,
    uniform: uniform::ChunkUniform,
    layout: &uniform::ChunkUniformLayout,
    mut instance: [Option<Vec<super::tile::types::BakedInstance>>;
      Dir::COUNT as usize],
  ) -> Self {
    let uniform = uniform::ChunkUniformInstance::new(
      ctx, layout, uniform,
    );
    let opaque_tile = std::array::from_fn(|i| {
      instance[i]
        .take()
        .map(|instance| {
          OpaqueTileInstances::new(ctx, instance)
        })
    });
    Self {
      uniform,
      opaque_tile,
    }
  }

  pub fn new_a(
    ctx: &WGPUCtx,
    uniform: uniform::ChunkUniform,
    layout: &uniform::ChunkUniformLayout,
    instance: [Option<OpaqueTileInstances>;
      Dir::COUNT as usize],
  ) -> Self {
    let uniform = uniform::ChunkUniformInstance::new(
      ctx, layout, uniform,
    );
    Self {
      uniform,
      opaque_tile: instance,
    }
  }

  pub fn write_instance_with<R>(
    &mut self,
    ctx: &WGPUCtx,
    f: impl FnOnce(
      &mut Vec<super::tile::types::BakedInstance>,
    ) -> R,
    dir: Dir,
  ) -> R {
    match &mut self.opaque_tile[dir as usize] {
      Some(opqt) => {
        let r = opqt.write_with(f);
        opqt.update(ctx);
        r
      }
      a => {
        let mut buffer = Vec::new();
        let r = f(&mut buffer);
        a.insert(OpaqueTileInstances::new(
          ctx, buffer,
        ));
        r
      }
    }
  }

  pub fn write_instance(
    &mut self,
    ctx: &WGPUCtx,
    instance: impl Iterator<
      Item = super::tile::types::BakedInstance,
    >,
    dir: Dir,
  ) {
    match &mut self.opaque_tile[dir as usize] {
      Some(opqt) => {
        opqt.write(instance);
        opqt.update(ctx);
      }
      a => {
        let opqt = OpaqueTileInstances::new(
          ctx,
          instance.collect(),
        );
        *a = Some(opqt)
      }
    }
  }

  pub fn remove_instance(
    &mut self,
    dir: Dir,
  ) -> Option<OpaqueTileInstances> {
    self.opaque_tile[dir as usize].take()
  }
}
