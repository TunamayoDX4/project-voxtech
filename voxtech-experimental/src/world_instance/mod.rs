//! ワールドのゲーム上・処理上の実体…インスタンスを実装するモジュール

use crate::{
  common::{
    l0_cell::Cell,
    l1_chunk::{Chunk, ChunkInfo},
    l2_sector::Sector,
    l3_region::Region,
    BlockPos, Dir, World,
  },
  gfx::{
    world::{
      chunk::ChunkObject, tile::types::BakedInstance,
    },
    GfxBundle,
  },
};

pub mod player;

pub struct WorldInstance {
  world: World,
}
impl WorldInstance {
  pub fn new() -> Self {
    let mut world = World::new();
    for x in -1..1 {
      for y in -1..1 {
        let pos = BlockPos::new(x, y, -1);
        let pos = pos.level_up(4);
        world
          .dim
          .spawn_region(pos, |_| {
            let sector =
              Box::new(std::array::from_fn(|i| {
                if 47 < i {
                  let chunk_info =
                    std::array::from_fn(|i| {
                      if (47 < i) {
                        ChunkInfo {
                          dirty_opq_tile: true,
                        }
                      } else {
                        Default::default()
                      }
                    });
                  let chunk = Box::new(
                    std::array::from_fn(|i| {
                      if (47 < i)
                        && ((i / 4) % 2 ^ i % 2) == 0
                      {
                        let chunk = Box::new(
                          std::array::from_fn(|i| {
                            if ((i / 16) % 2
                              ^ (i / 4) % 2
                              ^ i % 2)
                              == 0
                            {
                              Cell(std::array::from_fn(
                                |i| i as _,
                              ))
                            } else {
                              Cell([0; 64])
                            }
                          }),
                        );
                        Chunk { cell: Some(chunk) }
                      } else {
                        Chunk { cell: None }
                      }
                    }),
                  );
                  Sector {
                    chunk_info,
                    chunk: Some(chunk),
                    chunk_halo: None,
                  }
                } else {
                  Sector {
                    chunk_info: [Default::default(); _],
                    chunk: None,
                    chunk_halo: None,
                  }
                }
              }));
            Region {
              sector: Some(sector),
              sector_halo: None,
            }
          });
      }
    }
    Self { world }
  }

  pub fn rendering(&self, gfx: &mut GfxBundle) {
    gfx.world_modify(|ctx, wr| {
      for (pos, region) in self.world.dim.iter() {
        let Some(iter) = region.iter_sector(pos) else {
          continue;
        };
        for (_ipos, bpos, sector) in iter {
          let Some(iter) = sector.iter_chunk(&bpos)
          else {
            continue;
          };
          for (ipos, bpos, chunk) in iter {
            // 更新不要(ブロックが書き換えられていない場合)であればスキップ
            if !sector.chunk_info[ipos.0 as usize]
              .dirty_opq_tile
            {
              continue;
            }

            // セルがそもそもない場合にもスキップ
            let Some(cells) = chunk.cell.as_ref()
            else {
              continue;
            };
            if let Some((key, obj)) =
              wr.chunk.get_mut_by_pos(&bpos)
            {
              // 更新処理
              for i in 0..Dir::COUNT {
                let dir = Dir::from(i);
                obj.write_instance(
                  (0..4096)
                    .map(|i| {
                      let cell =
                        cells[i / 64].0[i % 64];
                      if cell != 0 {
                        let rgba = [
                          ((cell >> 0) & 3) as f32 / 3.,
                          ((cell >> 2) & 3) as f32 / 3.,
                          ((cell >> 4) & 3) as f32 / 3.,
                          1.,
                        ];
                        Some(BakedInstance {
                          stride: i as u32,
                          color: rgba,
                        })
                      } else {
                        None
                      }
                    })
                    .filter_map(|i| i),
                  dir,
                );
              }
              /*obj.write_instance(instance, dir);*/
            } else {
              // 新規登録処理
              wr.chunk.insert(bpos, || {
                ChunkObject::new(
                  ctx,
                  bpos.into(),
                  &wr.chunk_layout,
                  std::array::from_fn(|_| {
                    (0..4096)
                      .map(|i| {
                        let cell =
                          cells[i / 64].0[i % 64];
                        if cell != 0 {
                          let rgba = [
                            ((cell >> 0) & 3) as f32
                              / 3.,
                            ((cell >> 2) & 3) as f32
                              / 3.,
                            ((cell >> 4) & 3) as f32
                              / 3.,
                            1.,
                          ];
                          Some(BakedInstance {
                            stride: i as u32,
                            color: rgba,
                          })
                        } else {
                          None
                        }
                      })
                      .filter_map(|i| i)
                  }),
                )
              });
            }
          }
        }
      }
    });
  }
}
