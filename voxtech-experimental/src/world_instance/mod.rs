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
  PRwLock,
};

pub mod player;

pub struct WorldInstance {
  pub world: World,
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
                      if ((i / 16) % 2
                        ^ (i / 4) % 2
                        ^ i % 2)
                        == 0
                      {
                        ChunkInfo {
                          dirty_opq_tile: true,
                        }
                      } else {
                        Default::default()
                      }
                    });
                  let chunk = Box::new(
                    std::array::from_fn(|i| {
                      if ((i / 16) % 2
                        ^ (i / 4) % 2
                        ^ i % 2)
                        == 0
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
                    chunk_info: PRwLock::new(
                      chunk_info,
                    ),
                    chunk: Some(chunk),
                    chunk_halo: None,
                  }
                } else {
                  Sector {
                    chunk_info: PRwLock::new(
                      [Default::default(); _],
                    ),
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
            let mut cinfo = sector
              .chunk_info
              .upgradable_read();
            // 更新不要(ブロックが書き換えられていない場合)であればスキップ
            if !cinfo[ipos.0 as usize].dirty_opq_tile {
              continue;
            }
            // 更新フラグをリセットする
            cinfo.with_upgraded(|cinfo| {
              cinfo[ipos.0 as usize].dirty_opq_tile =
                false
            });

            // セルがそもそもない場合にもスキップ
            let Some(cells) = chunk.cell.as_ref()
            else {
              continue;
            };
            if let Some((key, obj)) =
              wr.chunk.get_mut_by_pos(&bpos)
            {
              // 更新処理
              for dir in Dir::iter() {
                obj.write_instance(
                  ctx,
                  (0..64).flat_map(move |i| {
                    let cell = cells[i];
                    let strided_cell = match dir
                      .invert()
                    {
                      Dir::WST => cell.stride_west(),
                      Dir::EST => cell.stride_east(),
                      Dir::STH => cell.stride_south(),
                      Dir::NTH => cell.stride_north(),
                      Dir::BTM => cell.stride_bottom(),
                      Dir::TOP => cell.stride_top(),
                      Dir::UNDEF => unreachable!(),
                    };
                    (0..64)
                      .filter(move |p| {
                        strided_cell.0[*p] == 0
                          && cell.0[*p] != 0
                      })
                      .map(move |j| {
                        let block = cell.0[j];
                        let rgba = [
                          ((block >> 0) & 3) as f32
                            / 3.,
                          ((block >> 2) & 3) as f32
                            / 3.,
                          ((block >> 4) & 3) as f32
                            / 3.,
                          1.,
                        ];
                        BakedInstance {
                          stride: (i * 64 + j) as u32,
                          color: rgba,
                        }
                      })
                  }),
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
                  std::array::from_fn(|dir_i| {
                    let dir = Dir::from(dir_i as u8);
                    (0..64).flat_map(move |i| {
                      let cell = cells[i];
                      let strided_cell = match dir
                        .invert()
                      {
                        Dir::WST => cell.stride_west(),
                        Dir::EST => cell.stride_east(),
                        Dir::STH => cell.stride_south(),
                        Dir::NTH => cell.stride_north(),
                        Dir::BTM => {
                          cell.stride_bottom()
                        }
                        Dir::TOP => cell.stride_top(),
                        Dir::UNDEF => unreachable!(),
                      };
                      (0..64)
                        .filter(move |p| {
                          strided_cell.0[*p] == 0
                            && cell.0[*p] != 0
                        })
                        .map(move |j| {
                          let block = cell.0[j];
                          let rgba = [
                            ((block >> 0) & 3) as f32
                              / 3.,
                            ((block >> 2) & 3) as f32
                              / 3.,
                            ((block >> 4) & 3) as f32
                              / 3.,
                            1.,
                          ];
                          BakedInstance {
                            stride: (i * 64 + j) as u32,
                            color: rgba,
                          }
                        })
                    })
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
