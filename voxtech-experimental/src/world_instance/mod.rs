//! ワールドのゲーム上・処理上の実体…インスタンスを実装するモジュール

use parking_lot::lock_api::RwLock;

use crate::{
  common::{
    l0_cell::{Cell, CellHalo},
    l1_chunk::{Chunk, ChunkInfo},
    l2_sector::Sector,
    l3_region::{self, Region},
    BlockPos, Dir, World,
  },
  gfx::{
    wgpu_ctx::WGPUCtx,
    world::{
      chunk::ChunkObject,
      tile::{
        types::BakedInstance, OpaqueTileInstances,
      },
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
                if 31 < i {
                  let chunk_info =
                    std::array::from_fn(|_| {
                      ChunkInfo {
                        visibility: [true; 6],
                        dirty_opq_tile: [true; 6],
                        rdr_storage_key: None,
                      }
                    });
                  let chunk = Box::new(
                    std::array::from_fn(|_| {
                      let chunk = Box::new(
                        std::array::from_fn(|_| {
                          Cell(std::array::from_fn(
                            |i| {
                              if i == 0 {
                                0
                              } else {
                                i as _
                              }
                            },
                          ))
                        }),
                      );
                      Chunk { cell: Some(chunk) }
                    }),
                  );
                  Sector {
                    chunk_info: PRwLock::new(
                      chunk_info,
                    ),
                    chunk: PRwLock::new(Some(chunk)),
                    chunk_halo: PRwLock::new(None),
                  }
                } else {
                  Sector {
                    chunk_info: PRwLock::new(
                      [Default::default(); 64],
                    ),
                    chunk: PRwLock::new(None),
                    chunk_halo: PRwLock::new(None),
                  }
                }
              }));
            Region {
              sector: RwLock::new(Some(sector)),
              sector_halo: RwLock::new(None),
            }
          });
      }
    }
    Self { world }
  }

  pub fn visibility_update(
    &self,
    camera_pos: &nalgebra::Point3<f64>,
    gfx: &mut GfxBundle,
  ) {
    for (pos, region) in self.world.dim.iter() {
      let rpos = nalgebra::Vector3::new(
        *pos.x() as f64,
        *pos.y() as f64,
        *pos.z() as f64,
      );
      let relat_cpos = camera_pos - rpos;
      let visibility =
        Region::chk_visible_face(&relat_cpos);
      region.iter_sector(
        pos,
        |sector_ipos, bpos, sector| {
          let sector_visibility =
            Sector::chk_visible_face(
              sector_ipos,
              &relat_cpos,
            );
          let visibility: [bool; Dir::COUNT as usize] =
            std::array::from_fn(|i| {
              sector_visibility[i] && visibility[i]
            });
          let mut lock = sector
            .chunk_info
            .upgradable_read();
          sector.iter_chunk(
            &bpos,
            |ipos, bpos, chunk| {
              let chunk_visibility =
                Chunk::chk_visible_face(
                  sector_ipos,
                  ipos,
                  &relat_cpos,
                );
              let visibility: [bool;
                Dir::COUNT as usize] =
                std::array::from_fn(|i| {
                  chunk_visibility[i] && visibility[i]
                });
              if lock[ipos.0 as usize].visibility
                != visibility
              {
                lock.with_upgraded(|lock| {
                  lock[ipos.0 as usize].visibility =
                    visibility;
                })
              }
              true
            },
          );
          true
        },
      );
    }
  }

  pub fn rendering(&self, gfx: &mut GfxBundle) {
    gfx.world_modify(|ctx, wr| {
      for (pos, region) in self.world.dim.iter() {
        region.update_neigh_west(None);
        region.update_neigh_east(None);
        region.update_neigh_south(None);
        region.update_neigh_north(None);
        region.update_neigh_bottom(None);
        region.update_neigh_top(None);
        region.iter_sector(
          pos,
          |ipos, bpos, sector| {
            let mut cinfo = sector
              .chunk_info
              .upgradable_read();
            // sector.update_neigh_west(None);
            // sector.update_neigh_east(None);
            // sector.update_neigh_south(None);
            // sector.update_neigh_north(None);
            // sector.update_neigh_bottom(None);
            // sector.update_neigh_top(None);
            sector.iter_chunk(
              &bpos,
              |ipos, bpos, chunk| {
                // 更新不要(ブロックが書き換えられていない場合)であればスキップ
                if !cinfo[ipos.0 as usize]
                  .dirty_opq_tile
                  .iter()
                  .fold(false, |r, dirty| r || *dirty)
                {
                  return true;
                }

                let halo = sector.chunk_halo.read();

                // セルがそもそもない場合にもスキップ
                let Some(cells) = chunk.cell.as_ref()
                else {
                  return true;
                };
                if let Some((key, obj)) =
                  wr.chunk.get_mut_by_pos(&bpos)
                {
                  // 更新処理
                  for dir in Dir::iter() {
                    if cinfo[ipos.0 as usize].visibility
                      [dir as usize]
                      && cinfo[ipos.0 as usize]
                        .dirty_opq_tile
                        [dir as usize]
                    {
                      cinfo.with_upgraded(|cinfo| {
                        cinfo[ipos.0 as usize]
                          .dirty_opq_tile
                          [dir as usize] = false
                      });
                      let halo = halo
                        .as_ref()
                        .map(|h| {
                          &h[dir.invert() as usize]
                            .as_ref()
                            [ipos.0 as usize]
                        })
                        .map(|h| h.cell.as_ref())
                        .flatten();
                      generate_chunk_mesh(
                        dir,
                        cells,
                        halo,
                        ctx,
                        &mut obj.opaque_tile
                          [dir as usize],
                      );
                    }
                  }
                  /*obj.write_instance(instance, dir);*/
                } else {
                  // 新規登録処理
                  if let Some(key) =
                    wr.chunk.insert(bpos, || {
                      ChunkObject::new_a(
                        ctx,
                        bpos.into(),
                        &wr.chunk_layout,
                        std::array::from_fn(|dir_i| {
                          let dir =
                            Dir::from(dir_i as u8);
                          if cinfo[ipos.0 as usize]
                            .visibility
                            [dir as usize]
                          {
                            let mut buffer = None;
                            let halo = halo
                              .as_ref()
                              .map(|h| {
                                &h[dir.invert()
                                  as usize]
                                  .as_ref()
                                  [ipos.0 as usize]
                              })
                              .map(|h| h.cell.as_ref())
                              .flatten();
                            generate_chunk_mesh(
                              dir,
                              cells,
                              halo,
                              ctx,
                              &mut buffer,
                            );
                            buffer
                          } else {
                            None
                          }
                        }),
                      )
                    })
                  {
                    cinfo.with_upgraded(|info| {
                      info[ipos.0 as usize]
                        .rdr_storage_key = Some(key)
                    })
                  }
                }
                true
              },
            );
            true
          },
        );
      }
    });
  }
}

fn generate_chunk_mesh(
  dir: Dir,
  cells: &[Cell; 64],
  halo: Option<&[CellHalo; 16]>,
  ctx: &WGPUCtx,
  tile_instances: &mut Option<OpaqueTileInstances>,
) {
  let strided: [Cell; 64] = std::array::from_fn(|i| {
    let cell = cells[i];
    match dir.invert() {
      Dir::WST => {
        if (i & 3) < 3 {
          cell.stride_west_neigh(
            &CellHalo::make_halo_west(&cells[i + 1]),
          )
        } else if let Some(halo) = halo {
          cell.stride_west_neigh(
            &halo[(i >> 2) & 3 | (i >> 4) & 12],
          )
        } else {
          cell.stride_west()
        }
      }
      Dir::EST => {
        if 0 < (i & 3) {
          cell.stride_east_neigh(
            &CellHalo::make_halo_east(&cells[i - 1]),
          )
        } else if let Some(halo) = halo {
          cell.stride_east_neigh(
            &halo[(i >> 2) & 3 | (i >> 4) & 12],
          )
        } else {
          cell.stride_east()
        }
      }
      Dir::STH => {
        if (i & 12) < 12 {
          cell.stride_south_neigh(
            &CellHalo::make_halo_south(&cells[i + 4]),
          )
        } else if let Some(halo) = halo {
          cell.stride_south_neigh(
            &halo[i & 3 | (i >> 4) & 12],
          )
        } else {
          cell.stride_south()
        }
      }
      Dir::NTH => {
        if 0 < (i & 12) {
          cell.stride_north_neigh(
            &CellHalo::make_halo_north(&cells[i - 4]),
          )
        } else if let Some(halo) = halo {
          cell.stride_north_neigh(
            &halo[i & 3 | (i >> 4) & 12],
          )
        } else {
          cell.stride_north()
        }
      }
      Dir::BTM => {
        if (i & 48) < 48 {
          cell.stride_bottom_neigh(
            &CellHalo::make_halo_bottom(&cells[i + 16]),
          )
        } else if let Some(halo) = halo {
          cell.stride_bottom_neigh(
            &halo[i & 3 | (i >> 2) & 12],
          )
        } else {
          cell.stride_bottom()
        }
      }
      Dir::TOP => {
        if 0 < (i & 48) {
          cell.stride_top_neigh(
            &CellHalo::make_halo_top(&cells[i - 16]),
          )
        } else if let Some(halo) = halo {
          cell.stride_top_neigh(
            &halo[i & 3 | (i >> 2) & 12],
          )
        } else {
          cell.stride_top()
        }
      }
      Dir::UNDEF => unreachable!(),
    }
  });
  let non_air_bits: [u64; 64] =
    std::array::from_fn(|i| cells[i].non_air_bits());
  if non_air_bits
    .iter()
    .map(|b| (*b != 0) as u64)
    .sum::<u64>()
    != 0
  {
    let instances =
      tile_instances.get_or_insert_with(|| {
        OpaqueTileInstances::new_empty(
          non_air_bits
            .iter()
            .map(|b| b.count_ones() as usize)
            .sum(),
        )
      });
    instances.write_with(|v| {
      for i in (0..64).filter(|i| non_air_bits[*i] != 0)
      {
        (0..64)
          .filter(move |j| {
            strided[i].0[*j] == 0 && cells[i].0[*j] != 0
          })
          .map(move |j| {
            let block = cells[i].0[j];
            let rgba = [
              ((block >> 0) & 3) as f32 / 3.,
              ((block >> 2) & 3) as f32 / 3.,
              ((block >> 4) & 3) as f32 / 3.,
              1.,
            ];
            BakedInstance {
              stride: (i * 64 + j) as u32,
              color: rgba,
            }
          })
          .for_each(|inst| v.push(inst));
      }
    });
    instances.update(ctx);
  }
}
