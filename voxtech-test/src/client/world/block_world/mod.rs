use crate::client::{
  gfx::wgpu_ctx::WGPUCtx,
  world::block_world::block::BlockLoID,
};

use super::super::gfx::renderer::tile::tile_instance::{
  InstanceBufferArray
};
use hashbrown::HashMap;
use nalgebra::{Point3, Vector3};

pub mod block;
pub mod cell;
pub mod chunk;

pub struct AreaQueryResult<
  'a,
  I: Iterator<
    Item = (chunk::ChunkPos, &'a chunk::Chunk),
  >,
> {
  pub iterator: I,
}

pub struct BlockWorld {
  chunks:
    HashMap<chunk::ChunkPos, chunk::Chunk>,
}
impl BlockWorld {
  pub fn new(_: ()) -> Self {
    let mut chunks = HashMap::new();
    for x in (-4..4).map(|i| i * 16) {
      for y in (-4..4).map(|i| i * 16) {
        for z in (-2..0).map(|i| i * 16) {
          chunks.insert(
            [x, y, z].into(),
            chunk::Chunk::new(
              chunk::ChunkBlocks::new(
                chunk::ChunkLoID(
                  std::array::from_fn(|i| {
                    if (i & 1)
                      ^ (i >> 2 & 1)
                      ^ (i >> 4 & 1)
                      == 0
                    {
                      cell::CellLoID(
                        std::array::from_fn(
                          |i| {
                            block::BlockLoID(
                              i as u8,
                            )
                          },
                        ),
                      )
                    } else if (2 <= (i & 3))
                      && (2 <= (i >> 2 & 3))
                      && (2 <= (i >> 4 & 3))
                    {
                      cell::CellLoID(
                        std::array::from_fn(
                          |i| {
                            if (i >> 1 & 1)
                              ^ (i >> 3 & 1)
                              ^ (i >> 5 & 1)
                              == 0
                            {
                              block::BlockLoID(
                                if (i >> 2 & 1) == 0 {
                                  1
                                } else if (i >> 4 & 1) == 0 {
                                  25
                                } else {
                                  30
                                },
                              )
                            } else {
                              block::BlockLoID::air()
                            }
                          },
                        ),
                      )
                    } else {
                      cell::CellLoID::air_cell()
                    }
                  }),
                ),
                Some(chunk::ChunkHiID(
                  std::array::from_fn(|j| {
                    cell::CellHiID(
                      std::array::from_fn(
                        |_| {
                          block::BlockHiID(
                            j as u8,
                          )
                        },
                      ),
                    )
                  }),
                )),
              ),
            ),
          );
        }
      }
    }
    Self { chunks }
  }

  pub fn render_update(
    &mut self,
    wgpu_ctx: &WGPUCtx,
    instance_array: &mut InstanceBufferArray,
  ) {
    for (chunk_pos, chunk) in
      self.chunks.iter_mut()
    {
      chunk.render_update(
        chunk_pos,
        wgpu_ctx,
        instance_array,
      );
    }
  }

  pub fn entity_motion_choose_nearest(
    &self,
    entity_pos: &Point3<f64>,
    entity_vel: &Vector3<f64>,
    entity_scale: &Vector3<f64>,
    entity_center_offset: Option<&Vector3<f64>>,
  ) -> Option<(
    [i64; 3],
    block::ray_check::RayCastResult,
    block::ray_check::Dir,
  )> {
    let mut best_hit: Option<(
      [i64; 3],
      block::ray_check::RayCastResult,
      block::ray_check::Dir,
    )> = None;
    let range = Self::calcurate_entity_range(
      entity_pos,
      entity_vel,
      entity_scale,
      entity_center_offset,
    );
    let offseted_center = entity_pos
      + entity_center_offset
        .unwrap_or(&Vector3::zeros());
    for (pos, cell_index, cell_entry) in
      self.block_index_query(&range)
    {
      // ブロックが空気である場合には処理不要。スキップ
      if cell_entry.lo_id.0[cell_index].is_air()
      {
        continue;
      }

      // ブロック座標・エンティティ座標を元に正規化した座標を計算する
      let normalized_point = block::AbstractBlock::ray_block_normalize(
        &Point3::from([pos[0] as _, pos[1] as _, pos[2] as _]), 
        &[offseted_center, offseted_center + entity_vel], 
        entity_scale
      );

      if let Some((rcr, dir)) =
        block::AbstractBlock::ray_check_opposing(
          &normalized_point,
        )
      {
        let hit =
          block::ray_check::RayCastResult::new(
            rcr,
            &normalized_point,
          );
        match best_hit.as_mut() {
          Some(best_hit) => {
            if hit.t < best_hit.1.t {
              *best_hit = (pos, hit, dir);
            }
          }
          None => {
            best_hit = Some((pos, hit, dir))
          }
        };
      }
    }

    best_hit
  }

  pub fn choose_raycheck_nearest(
    &self,
    point: Point3<f64>,
    ray: Vector3<f64>,
  ) -> Option<(
    [i64; 3],
    block::ray_check::RayCastResult,
    block::ray_check::Dir,
  )> {
    let mut best_hit: Option<(
      [i64; 3],
      block::ray_check::RayCastResult,
      block::ray_check::Dir,
    )> = None;
    let range = Self::range_to_discrete([
      point,
      point + ray,
    ]);

    for (pos, cell_index, cell_entry) in
      self.block_index_query(&range)
    {
      // ブロックが空気である場合には処理不要。スキップ
      if cell_entry.lo_id.0[cell_index].is_air()
      {
        continue;
      }

      // ブロック座標・エンティティ座標を元に正規化した座標を計算する
      let normalized_point = block::AbstractBlock::ray_block_normalize(
        &Point3::from([pos[0] as _, pos[1] as _, pos[2] as _]), 
        &[point, point + ray], 
        &Vector3::zeros(),
      );

      if let Some((rcr, dir)) =
        block::AbstractBlock::ray_check_opposing(
          &normalized_point,
        )
      {
        let hit =
          block::ray_check::RayCastResult::new(
            rcr,
            &normalized_point,
          );
        match best_hit.as_mut() {
          Some(best_hit) => {
            if hit.t < best_hit.1.t {
              *best_hit = (pos, hit, dir);
            }
          }
          None => {
            best_hit = Some((pos, hit, dir))
          }
        };
      }
    }

    best_hit
  }

  pub fn calcurate_entity_range(
    entity_pos: &Point3<f64>,
    entity_vel: &Vector3<f64>,
    entity_scale: &Vector3<f64>,
    entity_center_offset: Option<&Vector3<f64>>,
  ) -> [[i64; 2]; 3] {
    let half_scale = entity_scale * 0.5;
    let center = entity_pos
      + entity_center_offset
        .unwrap_or(&Vector3::zeros());
    let p = [center, center + entity_vel];
    let p: [Point3<f64>; 2] =
      [
        Point3::from(std::array::from_fn(
          |i| p[0][i].min(p[1][i]),
        )) - half_scale,
        Point3::from(std::array::from_fn(
          |i| p[0][i].max(p[1][i]),
        )) + half_scale,
      ];

    Self::range_to_discrete(p)
  }

  pub fn range_to_discrete(
    point: [Point3<f64>; 2],
  ) -> [[i64; 2]; 3] {
    let min: [i64; 3] =
      std::array::from_fn(|i| {
        point[0][i].min(point[1][i]).floor()
          as i64
      });
    let max: [i64; 3] =
      std::array::from_fn(|i| {
        point[0][i].max(point[1][i]).floor()
          as i64
      });

    std::array::from_fn(|i| [min[i], max[i]])
  }

  pub fn area_query<'a>(
    &'a self,
    range: &[[i64; 2]; 3],
  ) -> AreaQueryResult<
    'a,
    impl Iterator<
      Item = (
        chunk::ChunkPos,
        &'a chunk::Chunk,
      ),
    >,
  > {
    let iterator = (range[2][0] >> 4
      ..=range[2][1] >> 4)
      .flat_map(move |z| {
        (range[1][0] >> 4..=range[1][1] >> 4)
          .map(move |y| (y, z))
      })
      .flat_map(move |(y, z)| {
        (range[0][0] >> 4..=range[0][1] >> 4)
          .map(move |x| (x, y, z))
      })
      .map(|(x, y, z)| {
        chunk::ChunkPos::new([
          x << 4,
          y << 4,
          z << 4,
        ])
      })
      .filter_map(|p| {
        self.chunks.get(&p).map(|c| (p, c))
      })
      .filter(|(_, c)| {
        c.info().block_count != 0
      });

    AreaQueryResult { iterator }
  }

  pub fn block_index_query<'a>(
    &'a self,
    range: &[[i64; 2]; 3],
  ) -> impl Iterator<
    Item = (
      [i64; 3],
      usize,
      chunk::CellQueryEntry<'a>,
    ),
  > {
    self
      .area_query(range)
      .iterator
      .map(move |(cp, c)| (range, cp, c))
      .flat_map(|(range, chunk_pos, chunk)| {
        chunk
          .area_query(chunk_pos, range)
          .iterator
      })
      .flat_map(|data| {
        cell::AbstractCell::area_query(
          data.range,
        )
        .iterator
        .map(move |d| {
          (
            std::array::from_fn::<i64, 3, _>(
              |i| data.pos.0[i] + d.0[i] as i64,
            ),
            d.1 as _,
            data,
          )
        })
      })
  }

  pub fn remove_block(
    &mut self,
    pos: [i64; 3],
  ) -> Option<BlockLoID> {
    let chunk = self
      .chunks
      .get_mut(&chunk::ChunkPos::new(pos))?;
    chunk.info.update_flag = true;
    chunk.info.block_count -= 1;
    println!("{:?}", chunk.info);
    let cell_pos = (((pos[0] & 12) >> 2)
      | (pos[1] & 12)
      | ((pos[2] & 12) << 2))
      as usize;
    let cell =
      &mut chunk.blocks.lo_id.as_mut().0
        [cell_pos];
    let block_pos = ((pos[0] & 3)
      | (pos[1] & 3) << 2
      | (pos[2] & 3) << 4)
      as usize;
    Some(std::mem::replace(
      &mut cell.0[block_pos],
      BlockLoID::air(),
    ))
  }

  pub fn put_block(&mut self, pos: [i64; 3]) {
    let chunk = self
      .chunks
      .entry(chunk::ChunkPos::new(pos))
      .or_insert_with(|| chunk::Chunk::new(
        chunk::ChunkBlocks {
          lo_id: Box::new(chunk::ChunkLoID(
            [cell::CellLoID::air_cell(); 64],
          )),
          hi_id: None,
        },
      ));
    chunk.info.update_flag = true;
    chunk.info.block_count += 1;
    println!("{:?}", chunk.info);
    let cell_pos = (((pos[0] & 12) >> 2)
      | (pos[1] & 12)
      | ((pos[2] & 12) << 2))
      as usize;
    let cell =
      &mut chunk.blocks.lo_id.as_mut().0
        [cell_pos];
    let block_pos = ((pos[0] & 3)
      | (pos[1] & 3) << 2
      | (pos[2] & 3) << 4)
      as usize;
    cell.0[block_pos] = BlockLoID(0b00010101);
  }
}
