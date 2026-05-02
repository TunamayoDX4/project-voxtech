use nalgebra::{
  matrix, Point2, Point3, Vector3,
};

use crate::util::ray_check::{
  ray_casting_rhombus,
  RayCastResult as RayCastModuleRCResult,
};

pub const SKIN: f64 = 1.0e-9;

#[derive(Debug, Clone, Copy)]
pub struct RayCheckResultType {
  rcr: RayCastModuleRCResult,
  normal: Vector3<f64>,
  size: [Vector3<f64>; 2],
}

impl super::AbstractBlock {
  #[inline]
  pub fn ray_block_normalize(
    block_pos: &Point3<f64>,
    ray_points: &[Point3<f64>; 2],
    scale: &Vector3<f64>,
  ) -> [Point3<f64>; 2] {
    let offset = (-block_pos
      - Point3::origin())
      - Vector3::from([0.5, 0.5, 0.5]);
    let scale = scale.map(|v| 1. / (1. + v));
    let scale_mat = matrix![
      scale.x, 0., 0.;
      0., scale.y, 0.;
      0., 0., scale.z;
    ];
    std::array::from_fn(|i| {
      let v = ray_points[i] - Point3::origin();
      let v = scale_mat * (v + offset);
      Point3::origin() + v
    })
  }

  /// 向き合っているタイルに対するレイの判定
  pub fn ray_check_opposing_normalize_wrap(
    block_pos: &Point3<f64>,
    ray_points: &[Point3<f64>; 2],
    scale: &Vector3<f64>,
  ) -> Option<(RayCastResult, Dir)> {
    Self::ray_check_opposing(
      &Self::ray_block_normalize(
        block_pos, ray_points, scale,
      ),
    )
    .map(|(rcr, dir)| {
      (RayCastResult::new(rcr, ray_points), dir)
    })
  }

  /// 向き合っているタイルに対するレイの判定
  pub fn ray_check_opposing(
    ray_points: &[Point3<f64>; 2],
  ) -> Option<(RayCheckResultType, Dir)> {
    let ray_dir = ray_points[1] - ray_points[0];
    Dir::iter()
      // レイと法線が向き合っている場合にのみ処理をする。
      // つまり、タイルの面方向に対して外側からのレイに対してのみ当たり判定を行う。
      .filter(|dir| ray_dir.dot(&dir.normal()) < f64::EPSILON)
      .map(|dir| (dir.face(), dir))
      .filter_map(|(face, dir)| {
        face.ray_check(ray_points).map(|res| (res, dir))
      })
      // 最も手前で当たっている面を取り出す
      .fold(
        None::<(
          RayCastModuleRCResult,
          Vector3<f64>,
          [Vector3<f64>; 2],
          Dir,
        )>,
        |best_hit, ((hit, normal, face_scale), dir)| {
          match best_hit {
            Some((best_hit_, _, _, _))
              if best_hit_.t < hit.t =>
            {
              best_hit
            }
            _ => Some((hit, normal, face_scale, dir)),
          }
        },
      )
      .map(|(rcr, normal, size, dir)| {
        (RayCheckResultType { rcr, normal, size }, dir)
      })
  }
}

#[repr(C)]
#[derive(Debug, Clone)]
pub struct RayCastResult {
  pub t: f64,
  pub uv: Point2<f64>,
  pub uv_gap: [Vector3<f64>; 2],
  pub pos: Point3<f64>,
  pub normal: Vector3<f64>,
}
impl RayCastResult {
  #[inline]
  pub fn new(
    rcr: RayCheckResultType,
    ray_points: &[Point3<f64>],
  ) -> Self {
    let rc_t = rcr.rcr.t - SKIN;
    let ray_vec =
      (ray_points[1] - ray_points[0]) * rc_t;
    let uv =
      rcr.rcr.uv.map(|uv| (uv - 0.5) * 2.0);
    let uv_gap = std::array::from_fn(|i| {
      rcr.size[i] * (1. - uv[i])
    });
    Self {
      t: rc_t,
      uv: rcr.rcr.uv,
      uv_gap,
      pos: ray_points[0] + ray_vec,
      normal: rcr.normal,
    }
  }
}

#[derive(Debug, Clone, Copy)]
pub struct BlockTile(pub [Point3<f64>; 3]);
impl BlockTile {
  #[inline]
  pub fn ray_check_normalize_wrap(
    &self,
    block_pos: &Point3<f64>,
    ray_points: &[Point3<f64>; 2],
    scale: &Vector3<f64>,
  ) -> Option<RayCastResult> {
    self
      .ray_check(
        &super::AbstractBlock::ray_block_normalize(
          block_pos, ray_points, scale,
        ),
      )
      .map(|(rcr, normal, size)| {
        let rcr = RayCheckResultType {
          rcr,
          normal,
          size,
        };
        RayCastResult::new(rcr, ray_points)
      })
  }

  #[inline]
  pub fn ray_check(
    &self,
    ray_points: &[Point3<f64>; 2],
  ) -> Option<(
    RayCastModuleRCResult,
    Vector3<f64>,
    [Vector3<f64>; 2],
  )> {
    let normal = (self.0[1] - self.0[0])
      .cross(&(self.0[2] - self.0[0]))
      .normalize();

    ray_casting_rhombus(ray_points, &self.0)
      .map(|rc| {
        (
          rc,
          normal,
          [
            self.0[1] - self.0[0],
            self.0[2] - self.0[0],
          ],
        )
      })
  }
}

#[repr(u8)]
#[derive(Debug, Clone, Copy)]
pub enum Dir {
  EAST = 0,
  WEST = 1,
  SOUTH = 2,
  NORTH = 3,
  BOTTOM = 4,
  TOP = 5,
}
impl Dir {
  #[inline]
  pub fn face(self) -> &'static BlockTile {
    &TILES[self as usize]
  }

  #[inline]
  pub fn inverse(&self) -> Self {
    match self {
      Dir::EAST => Dir::EAST,
      Dir::WEST => Dir::WEST,
      Dir::SOUTH => Dir::NORTH,
      Dir::NORTH => Dir::SOUTH,
      Dir::BOTTOM => Dir::TOP,
      Dir::TOP => Dir::BOTTOM,
    }
  }

  #[inline]
  pub fn normal(&self) -> Vector3<f64> {
    match self {
      Dir::EAST => [-1., 0., 0.].into(),
      Dir::WEST => [1., 0., 0.].into(),
      Dir::SOUTH => [0., -1., 0.].into(),
      Dir::NORTH => [0., 1., 0.].into(),
      Dir::BOTTOM => [0., 0., -1.].into(),
      Dir::TOP => [0., 0., 1.].into(),
    }
  }

  #[inline]
  pub fn iter(
  ) -> impl DoubleEndedIterator<Item = Dir> {
    (0..6u8).map(|i| unsafe {
      std::mem::transmute(i)
    })
  }
}

/// タイルの配列
pub const TILES: [BlockTile; 6] =
  [EAST, WEST, SOUTH, NORTH, BOTTOM, TOP];

pub const EAST: BlockTile = BlockTile([
  Point3::new(-0.5, -0.5, -0.5),
  Point3::new(-0.5, -0.5, 0.5),
  Point3::new(-0.5, 0.5, -0.5),
]);

pub const WEST: BlockTile = BlockTile([
  Point3::new(0.5, -0.5, -0.5),
  Point3::new(0.5, 0.5, -0.5),
  Point3::new(0.5, -0.5, 0.5),
]);

pub const SOUTH: BlockTile = BlockTile([
  Point3::new(-0.5, -0.5, -0.5),
  Point3::new(-0.5, -0.5, 0.5),
  Point3::new(0.5, -0.5, -0.5),
]);

pub const NORTH: BlockTile = BlockTile([
  Point3::new(-0.5, 0.5, -0.5),
  Point3::new(0.5, 0.5, -0.5),
  Point3::new(-0.5, 0.5, 0.5),
]);

pub const BOTTOM: BlockTile = BlockTile([
  Point3::new(-0.5, -0.5, -0.5),
  Point3::new(-0.5, 0.5, -0.5),
  Point3::new(0.5, -0.5, -0.5),
]);

pub const TOP: BlockTile = BlockTile([
  Point3::new(-0.5, -0.5, 0.5),
  Point3::new(0.5, -0.5, 0.5),
  Point3::new(-0.5, 0.5, 0.5),
]);

pub mod test {
  use super::*;
  use nalgebra::Point3;

  pub const TEST_RAY_TO_BOTTOM: [Point3<f64>;
    2] = [
    Point3::new(0.5, 0.5, 5.0),
    Point3::new(0.5, 0.5, -5.0),
  ];
  pub const TEST_RAY_TO_TOP: [Point3<f64>; 2] = [
    Point3::new(0.5, 0.5, -5.0),
    Point3::new(0.5, 0.5, 5.0),
  ];
  pub const EPSILON: f64 = 0.000001;

  #[test]
  fn top_tile_from_top_raycast_test() {
    let check_result = TOP
      .ray_check_normalize_wrap(
        &[0., 0., 0.].into(),
        &TEST_RAY_TO_BOTTOM,
        &[0., 0., 0.].into(),
      );
    println!("{:?}", check_result);
    assert!(check_result.is_some());
    let check_result = check_result.unwrap();
    assert!(
      (check_result.t - 4.0 / 10.0).abs()
        < EPSILON
    );
  }

  #[test]
  fn bottom_tile_from_top_raycast_test() {
    let check_result = BOTTOM
      .ray_check_normalize_wrap(
        &[0., 0., 0.].into(),
        &TEST_RAY_TO_BOTTOM,
        &[0., 0., 0.].into(),
      );
    println!("{:?}", check_result);
    assert!(check_result.is_some());
    let check_result = check_result.unwrap();
    assert!(
      (check_result.t - 5.0 / 10.0).abs()
        < EPSILON
    );
  }

  #[test]
  fn bottom_tile_from_bottom_raycast_test() {
    let check_result = BOTTOM
      .ray_check_normalize_wrap(
        &[0., 0., 0.].into(),
        &TEST_RAY_TO_TOP,
        &[0., 0., 0.].into(),
      );
    println!(
      "\x1b[034mBOTTOM_TILE_FROM_BOTTOM:\x1b[037m {:?}",
      check_result
    );
    assert!(check_result.is_some());
    let check_result = check_result.unwrap();
    assert!(
      (check_result.t - 5.0 / 10.0).abs()
        < EPSILON
    );
  }

  #[test]
  fn top_tile_from_top_sized_05_raycast_test() {
    let check_result = TOP
      .ray_check_normalize_wrap(
        &[0., 0., 0.].into(),
        &TEST_RAY_TO_BOTTOM,
        &[0.5, 0.5, 0.5].into(),
      );
    println!(
      "TOP_TILE_FROM_TOP_SIZED_05\t{:?}",
      check_result
    );
    assert!(check_result.is_some());
    let check_result = check_result.unwrap();
    assert!(
      (check_result.t - 3.75 / 10.0).abs()
        < EPSILON
    );
  }

  #[test]
  fn top_tile_from_top_sized_1_raycast_test() {
    let check_result = TOP
      .ray_check_normalize_wrap(
        &[0., 0., 0.].into(),
        &TEST_RAY_TO_BOTTOM,
        &[1., 1., 1.].into(),
      );
    println!("{:?}", check_result);
    assert!(check_result.is_some());
    let check_result = check_result.unwrap();
    assert!(
      (check_result.t - 3.5 / 10.0).abs()
        < EPSILON
    );
  }

  #[test]
  fn top_tile_from_top_sized_2_raycast_test() {
    let check_result = TOP
      .ray_check_normalize_wrap(
        &[0., 0., 0.].into(),
        &TEST_RAY_TO_BOTTOM,
        &[4., 4., 4.].into(),
      );
    println!("{:?}", check_result);
    assert!(check_result.is_some());
    let check_result = check_result.unwrap();
    assert!(
      (check_result.t - 2.0 / 10.0).abs()
        < EPSILON
    );
  }

  #[test]
  fn bottom_tile_from_bottom_sized_2_raycast_test(
  ) {
    let check_result = BOTTOM
      .ray_check_normalize_wrap(
        &[0., 0., 0.].into(),
        &TEST_RAY_TO_TOP,
        &[4., 4., 4.].into(),
      );
    println!(
      "\x1b[034mBOTTOM_TILE_FROM_BOTTOM_SIZED:\x1b[037m {:?}",
      check_result
    );
    assert!(check_result.is_some());
    let check_result = check_result.unwrap();
    assert!(
      (check_result.t - 5.0 / 10.0).abs()
        < EPSILON
    );
  }

  #[test]
  fn block_top_from_top_sized_2_raycast_test() {
    let check_result =
      super::super::AbstractBlock::ray_check_opposing_normalize_wrap(
        &[0., 0., 0.].into(),
        &TEST_RAY_TO_BOTTOM,
        &[4., 4., 4.].into(),
      );
    println!("{:?}", check_result);
    assert!(check_result.is_some());
    let check_result = check_result.unwrap();
    assert!(
      (check_result.0.t - 2.0 / 10.0).abs()
        < EPSILON
    );
  }
}
