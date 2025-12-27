//! ワールドのゲーム上・処理上の実体…インスタンスを実装するモジュール

use crate::{
  common::{
    l0_cell::Cell, l1_chunk::Chunk, l2_sector::Sector,
    l3_region::Region, BlockPos, World,
  },
  gfx::{
    world::{
      chunk::ChunkObject, tile::types::BakedInstance,
    },
    GfxBundle,
  },
};

pub struct WorldInstance {
  world: World,
}
impl WorldInstance {
  pub fn new() -> Self {
    let mut world = World::new();
    for x in -1..=1 {
      for y in -1..=1 {
        let pos = BlockPos::new(x, y, -1);
        println!("RAW: {pos:?}");
        let pos = pos.level_up(4);
        println!("REGION: {pos:?}");
        world
          .dim
          .spawn_region(pos, |_| {
            let sector =
              Box::new(std::array::from_fn(|i| {
                if 47 < i {
                  let chunk = Box::new(
                    std::array::from_fn(|i| {
                      if 47 < i {
                        let chunk = Box::new(
                          std::array::from_fn(|_| {
                            Cell([1; 64])
                          }),
                        );
                        Chunk { cell: Some(chunk) }
                      } else {
                        Chunk { cell: None }
                      }
                    }),
                  );
                  Sector {
                    chunk: Some(chunk),
                    chunk_halo: None,
                  }
                } else {
                  Sector {
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
          for (_ipos, bpos, chunk) in iter {
            let Some(cells) = chunk.cell.as_ref()
            else {
              continue;
            };
            if let Some((key, obj)) =
              wr.chunk.get_mut_by_pos(&bpos)
            {
              /*obj.write_instance(instance, dir);*/
            } else {
              println!("{bpos:?}");
              wr.chunk.insert(bpos, || {
                ChunkObject::new(
                  ctx,
                  bpos.into(),
                  &wr.chunk_layout,
                  std::array::from_fn(|_| {
                    (0..4096)
                      .map(|i| {
                        if cells[i / 64].0[i % 64] == 1
                        {
                          Some(BakedInstance {
                            stride: i as u32,
                            tex_pos: [0., 0.],
                            tex_scale: [0., 0.],
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
