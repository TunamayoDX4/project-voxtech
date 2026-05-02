use std::ops::Range;

use super::cell::*;
use crate::client::gfx::{
  renderer::tile::tile_instance::{
    Instance, InstanceBufferArray,
    InstanceBufferKey,
  },
  wgpu_ctx::WGPUCtx,
};

#[repr(C)]
#[derive(
  Debug, Clone, Copy, PartialEq, Eq, Hash,
)]
pub struct ChunkPos(pub(super) [i64; 3]);
impl ChunkPos {
  #[inline]
  pub fn new(value: [i64; 3]) -> Self {
    value.into()
  }
  #[inline]
  pub fn get(&self) -> &[i64; 3] {
    &self.0
  }
}
impl From<CellPos> for ChunkPos {
  #[inline]
  fn from(value: CellPos) -> Self {
    value.0.into()
  }
}
impl From<[i64; 3]> for ChunkPos {
  #[inline]
  fn from(value: [i64; 3]) -> Self {
    Self([
      value[0] & !15,
      value[1] & !15,
      value[2] & !15,
    ])
  }
}
impl From<ChunkPos> for [i64; 3] {
  #[inline]
  fn from(value: ChunkPos) -> Self {
    value.0
  }
}

#[repr(C, align(64))]
#[derive(Debug, Clone)]
pub struct ChunkLoID(pub [CellLoID; 64]);

#[repr(C, align(64))]
#[derive(Debug, Clone)]
pub struct ChunkHiID(pub [CellHiID; 64]);

#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct ChunkInfo {
  pub update_flag: bool,
  pub block_count: u16,
}
impl ChunkInfo {
  pub fn new(lo_id: &ChunkLoID) -> Self {
    Self {
      block_count: lo_id
        .0
        .iter()
        .flat_map(|b| b.0.iter())
        .filter(|b| !b.is_air())
        .count() as _,
      update_flag: true,
    }
  }
}

#[derive(Debug, Clone, Default)]
pub struct ChunkRenderData {
  key: Option<InstanceBufferKey>,
  instances: Vec<Instance>,
}
impl ChunkRenderData {
  pub fn update_instances(
    &mut self,
    pos: &ChunkPos,
    blocks: &ChunkBlocks,
  ) {
    self.instances.clear();
    for i in (0..64u8).map(|p| {
      [(p << 2) & 12, p & 12, (p >> 2) & 12, p]
    }) {
      for j in (0..64u8).map(|p| {
        [
          (p & 3) | i[0],
          ((p >> 2) & 3) | i[1],
          ((p >> 4) & 3) | i[2],
          p,
        ]
      }) {
        let hi_id =
          blocks.hi_id.clone().map(|hi_id| {
            hi_id.as_ref().0[i[3] as usize].0
              [j[3] as usize]
          });
        let lo_id = blocks.lo_id.as_ref().0
          [i[3] as usize]
          .0[j[3] as usize];
        if lo_id.is_air() {
          continue;
        }
        self.instances.push(Instance {
          position: [
            ((pos.get()[0]) + j[0] as i64)
              as f32,
            ((pos.get()[1]) + j[1] as i64)
              as f32,
            ((pos.get()[2]) + j[2] as i64)
              as f32,
            0.,
          ],
          color: [
            (lo_id.0 & 3) as f32 / 3.
              + (hi_id.map_or(0., |id| {
                (id.0 & 3) as f32
              }))
                / 16.,
            ((lo_id.0 >> 2) & 3) as f32 / 3.
              + (hi_id.map_or(0., |id| {
                ((id.0 >> 2) & 3) as f32
              }))
                / 16.,
            ((lo_id.0 >> 4) & 3) as f32 / 3.
              + (hi_id.map_or(0., |id| {
                ((id.0 >> 4) & 3) as f32
              }))
                / 16.,
            1.,
          ],
        })
      }
    }
  }

  pub fn write_instance(
    &mut self,
    wgpu_ctx: &WGPUCtx,
    instance_array: &mut InstanceBufferArray,
  ) {
    if !self.instances.is_empty() {
      if let Some(key) = self.key {
        instance_array.update(
          &key,
          wgpu_ctx,
          &self.instances,
        );
      } else {
        self.key = Some(
          instance_array
            .insert(wgpu_ctx, &self.instances),
        );
      }
    } else {
      if let Some(key) = self.key {
        instance_array.remove(&key);
      }
      self.key = None;
    }
  }
}

pub struct ChunkBlocks {
  pub lo_id: Box<ChunkLoID>,
  pub hi_id: Option<Box<ChunkHiID>>,
}
impl ChunkBlocks {
  pub fn new(
    lo_id: ChunkLoID,
    hi_id: Option<ChunkHiID>,
  ) -> Self {
    Self {
      lo_id: Box::new(lo_id),
      hi_id: hi_id.map(Box::new),
    }
  }
}

pub struct AreaQueryResult<
  'a,
  I: Iterator<Item = CellQueryEntry<'a>>,
> {
  pub range: [[i64; 2]; 3],
  pub iterator: I,
}

#[repr(align(64))]
#[derive(Debug, Clone, Copy)]
pub struct CellQueryEntry<'a> {
  pub pos: CellPos,
  pub range: [[u8; 2]; 3],
  pub lo_id: &'a CellLoID,
  pub hi_id: Option<&'a CellHiID>,
}

pub struct Chunk {
  rdr_data: ChunkRenderData,
  pub info: ChunkInfo,
  pub blocks: ChunkBlocks,
}
impl Chunk {
  pub fn new(blocks: ChunkBlocks) -> Self {
    let info = ChunkInfo::new(&blocks.lo_id);
    Self {
      blocks,
      info,
      rdr_data: ChunkRenderData::default(),
    }
  }

  pub fn info(&self) -> &ChunkInfo {
    &self.info
  }

  pub fn render_update(
    &mut self,
    pos: &ChunkPos,
    wgpu_ctx: &WGPUCtx,
    instance_array: &mut InstanceBufferArray,
  ) {
    if self.info.update_flag {
      self.info.update_flag = false;
      self
        .rdr_data
        .update_instances(pos, &self.blocks);
      self.rdr_data.write_instance(
        wgpu_ctx,
        instance_array,
      );
    }
  }

  pub fn area_query<'a>(
    &'a self,
    chunk_pos: ChunkPos,
    range: &[[i64; 2]; 3],
  ) -> AreaQueryResult<
    'a,
    impl Iterator<Item = CellQueryEntry<'a>>,
  > {
    let range: [[i64; 2]; 3] =
      std::array::from_fn(|i| {
        [
          chunk_pos.0[i].max(range[i][0]) & 15,
          (chunk_pos.0[i] + 15)
            .min(range[i][1])
            & 15,
        ]
      });
    let iterator = ((range[2][0] >> 2)
      ..=(range[2][1] >> 2))
      .flat_map(move |z| {
        ((range[1][0] >> 2)
          ..=(range[1][1] >> 2))
          .map(move |y| [y, z])
      })
      .flat_map(move |[y, z]| {
        ((range[0][0] >> 2)
          ..=(range[0][1] >> 2))
          .map(move |x| [x, y, z])
      })
      .map(|pos| {
        (
          (pos[0] + pos[1] * 4 + pos[2] * 16)
            as usize,
          pos,
        )
      })
      .filter(|(idx, _)| {
        !self.blocks.lo_id.as_ref().0[*idx]
          .is_all_air()
      })
      .map(move |(idx, p)| {
        (
          idx,
          CellPos::new(std::array::from_fn(
            |i| p[i] * 4 + chunk_pos.0[i],
          )),
        )
      })
      .map(|(idx, p)| {
        (
          idx,
          p,
          std::array::from_fn::<i64, 3, _>(
            |i| {
              4 * ((idx >> (i * 2)) & 3) as i64
            },
          ),
        )
      })
      .map(move |(idx, p, local_p)| {
        let range: [[u8; 2]; 3] =
          std::array::from_fn(|i| {
            [
              ((local_p[i]).max(range[i][0])
                & 3) as u8,
              ((local_p[i] + 3)
                .min(range[i][1])
                & 3) as u8,
            ]
          });
        CellQueryEntry {
          pos: p,
          range,
          lo_id: &self.blocks.lo_id.as_ref().0
            [idx],
          hi_id: self
            .blocks
            .hi_id
            .as_ref()
            .map(|hid| &hid.as_ref().0[idx]),
        }
      });

    AreaQueryResult { range, iterator }
  }

  pub fn aabb_check(
    &self,
    chunk_pos: ChunkPos,
    range: &[[i64; 2]; 3],
  ) -> bool {
    let query =
      self.area_query(chunk_pos, range);
    for entry in query.iterator {
      if entry.lo_id.aabb_check(entry.range) {
        return true;
      }
    }
    false
  }
}
