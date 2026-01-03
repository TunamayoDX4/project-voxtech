//! ワールドのゲーム上・処理上の実体…インスタンスを実装するモジュール

use parking_lot::lock_api::RwLock;

use crate::{
  common::{
    l0_cell::Cell,
    l1_chunk::{Chunk, ChunkInfo},
    l2_sector::Sector,
    l3_region::{self, Region},
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
                          visibility: [true; 6],
                          dirty_opq_tile: [true; 6],
                          rdr_storage_key: None,
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
                          std::array::from_fn(|_| {
                            Cell(std::array::from_fn(
                              |i| i as _,
                            ))
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

  pub fn visibility_update(
    &self,
    camera_pos: &nalgebra::Point3<f64>,
  ) {
    for (pos, region) in self.world.dim.iter() {
      let Some(iter_sector) = region.iter_sector(pos)
      else {
        continue;
      };
      let rpos = nalgebra::Vector3::new(
        *pos.x() as f64,
        *pos.y() as f64,
        *pos.z() as f64,
      );
      let relat_cpos = camera_pos - rpos;
      let visibility =
        Region::chk_visible_face(&relat_cpos);
      for (ipos, bpos, sector) in iter_sector {
        let Some(iter_chunk) = sector.iter_chunk(&bpos)
        else {
          continue;
        };
        let sector_visibility =
          Sector::chk_visible_face(ipos, &relat_cpos);
        let visibility: [bool; Dir::COUNT as usize] =
          std::array::from_fn(|i| {
            sector_visibility[i] && visibility[i]
          });
        let mut lock = sector.chunk_info.write();
        for (ipos, _bpos, _chunk) in iter_chunk {
          lock[ipos.0 as usize].visibility = visibility;
        }
      }
    }
  }

  pub fn rendering(&self, gfx: &mut GfxBundle) {
    gfx.world_modify(|ctx, wr| {
      for (pos, region) in self.world.dim.iter() {
        let Some(iter) = region.iter_sector(pos) else {
          continue;
        };
        for (ipos, bpos, sector) in iter {
          let Some(iter) = sector.iter_chunk(&bpos)
          else {
            continue;
          };
          for (ipos, bpos, chunk) in iter {
            let mut cinfo = sector
              .chunk_info
              .upgradable_read();
            // 更新不要(ブロックが書き換えられていない場合)であればスキップ
            if !cinfo[ipos.0 as usize]
              .dirty_opq_tile
              .iter()
              .fold(false, |r, dirty| r || *dirty)
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
                  obj.write_instance(
                    ctx,
                    generate_chunk_mesh(dir, cells),
                    dir,
                  );
                }
              }
              /*obj.write_instance(instance, dir);*/
            } else {
              // 新規登録処理
              if let Some(key) =
                wr.chunk.insert(bpos, || {
                  ChunkObject::new(
                    ctx,
                    bpos.into(),
                    &wr.chunk_layout,
                    std::array::from_fn(|dir_i| {
                      let dir = Dir::from(dir_i as u8);
                      if cinfo[ipos.0 as usize]
                        .visibility
                        [dir as usize]
                      {
                        cinfo.with_upgraded(|cinfo| {
                          cinfo[ipos.0 as usize]
                            .dirty_opq_tile
                            [dir as usize] = false
                        });
                        Some(
                          generate_chunk_mesh(
                            dir, cells,
                          )
                          .collect(),
                        )
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
          }
        }
      }
    });
  }
}

fn generate_chunk_mesh(
  dir: Dir,
  cells: &[Cell; 64],
) -> impl Iterator<Item = BakedInstance> {
  (0..64).flat_map(move |i| {
    let cell = cells[i];
    let strided_cell = match dir.invert() {
      Dir::WST => {
        if (i & 3) < 3 {
          cell.stride_west_neigh(&cells[i + 1])
        } else {
          cell.stride_west()
        }
      }
      Dir::EST => {
        if 0 < (i & 3) {
          cell.stride_east_neigh(&cells[i - 1])
        } else {
          cell.stride_east()
        }
      }
      Dir::STH => {
        if (i & 12) < 12 {
          cell.stride_south_neigh(&cells[i + 4])
        } else {
          cell.stride_south()
        }
      }
      Dir::NTH => {
        if 0 < (i & 12) {
          cell.stride_north_neigh(&cells[i - 4])
        } else {
          cell.stride_north()
        }
      }
      Dir::BTM => {
        if (i & 48) < 48 {
          cell.stride_bottom_neigh(&cells[i + 16])
        } else {
          cell.stride_bottom()
        }
      }
      Dir::TOP => {
        if 0 < (i & 48) {
          cell.stride_top_neigh(&cells[i - 16])
        } else {
          cell.stride_top()
        }
      }
      Dir::UNDEF => unreachable!(),
    };
    (0..64)
      .filter(move |p| {
        strided_cell.0[*p] == 0 && cell.0[*p] != 0
      })
      .map(move |j| {
        let block = cell.0[j];
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
  })
}
