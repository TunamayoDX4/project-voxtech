use std::{f64, ops::Range};

use nalgebra::{Point2, Point3, Vector3};

/// レイキャストの結果
#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct RayCastResult {
  /// 接点のベクトル上の位置
  pub t: f64,

  /// 接点の平面上の座標
  pub uv: Point2<f64>,
}

const RANGE: Range<f64> =
  -f64::EPSILON..1.0 + f64::EPSILON;

#[inline]
pub fn ray_casting_rhombus(
  ray_points: &[Point3<f64>; 2],
  rect_points: &[Point3<f64>; 3],
) -> Option<RayCastResult> {
  let result = ray_casting_inner(
    ray_points, //
    rect_points,
  )?;

  if RANGE.contains(&result.t)
    && RANGE.contains(&result.uv.x)
    && RANGE.contains(&result.uv.y)
  {
    Some(result)
  } else {
    None
  }
}

#[inline]
pub fn ray_casting_triangle(
  ray_points: &[Point3<f64>; 2],
  rect_points: &[Point3<f64>; 3],
) -> Option<RayCastResult> {
  let result = ray_casting_inner(
    ray_points, //
    rect_points,
  )?;

  if RANGE.contains(&result.t)
    && RANGE
      .contains(&(result.uv.x + result.uv.y))
  {
    Some(result)
  } else {
    None
  }
}

#[inline]
fn ray_casting_inner(
  ray_points: &[Point3<f64>; 2],
  rect_points: &[Point3<f64>; 3],
) -> Option<RayCastResult> {
  let edge = [
    rect_points[1] - rect_points[0],
    rect_points[2] - rect_points[0],
  ];
  let d = ray_points[1] - ray_points[0];
  let q = d.cross(&edge[1]);
  let denom = q.dot(&edge[0]);
  if denom.abs() < f64::EPSILON {
    return None;
  }
  let e0 = ray_points[0] - rect_points[0];
  let r = e0.cross(&edge[0]);
  let tuv = (1. / denom)
    * Vector3::from([
      r.dot(&edge[1]),
      q.dot(&e0),
      r.dot(&d),
    ]);

  Some(RayCastResult {
    t: tuv.x,
    uv: Point2::from([tuv.y, tuv.z]),
  })
}

mod test {
  use super::*;

  #[test]
  fn ray_casting_test_rhombus_0() {
    let rect_points = [
      Point3::from([-1., -1., 0.]), //
      Point3::from([-1., 1., 0.]),  //
      Point3::from([1., -1., 0.]),
    ];
    let ray_points = [
      Point3::from([0., 0., -1.]), //
      Point3::from([0., 0., 1.]),
    ];
    let res = ray_casting_rhombus(
      &ray_points, //
      &rect_points,
    );
    assert!(res.is_some());
    let res = res.unwrap();
    assert!((res.t - 0.5).abs() < f64::EPSILON);
    assert!(
      (res.uv[0] - 0.5).abs() < f64::EPSILON
    );
    assert!(
      (res.uv[1] - 0.5).abs() < f64::EPSILON
    );
  }

  #[test]
  fn ray_casting_test_rhombus_1() {
    let rect_points = [
      Point3::from([-1., -1., 1.]), //
      Point3::from([-1., 1., 1.]),  //
      Point3::from([1., -1., 1.]),
    ];
    let ray_points = [
      Point3::from([0., 0., 0.]), //
      Point3::from([0., 0., 2.]),
    ];
    let res = ray_casting_rhombus(
      &ray_points, //
      &rect_points,
    );
    assert!(res.is_some());
    let res = res.unwrap();
    assert!((res.t - 0.5).abs() < f64::EPSILON);
    assert!(
      (res.uv[0] - 0.5).abs() < f64::EPSILON
    );
    assert!(
      (res.uv[1] - 0.5).abs() < f64::EPSILON
    );
  }

  #[test]
  fn ray_casting_test_triangle_0() {
    let tri_points = [
      Point3::from([-1., -1., 0.]), //
      Point3::from([-1., 1., 0.]),  //
      Point3::from([1., -1., 0.]),
    ];
    let ray_points = [
      Point3::from([0., 0., -1.]), //
      Point3::from([0., 0., 1.]),
    ];
    let res = ray_casting_triangle(
      &ray_points, //
      &tri_points,
    );
    assert!(res.is_some());
    let res = res.unwrap();
    assert!((res.t - 0.5).abs() < f64::EPSILON);
    assert!(
      (res.uv[0] - 0.5).abs() < f64::EPSILON
    );
    assert!(
      (res.uv[1] - 0.5).abs() < f64::EPSILON
    );
  }

  #[test]
  fn ray_casting_test_triangle_1() {
    let tri_points = [
      Point3::from([-1., -1., 0.]), //
      Point3::from([-1., 1., 0.]),  //
      Point3::from([1., -1., 0.]),
    ];
    let ray_points = [
      Point3::from([-0.1, -0.1, -1.]), //
      Point3::from([-0.1, -0.1, 1.]),
    ];
    let res = ray_casting_triangle(
      &ray_points, //
      &tri_points,
    );
    assert!(res.is_some());
    let res = res.unwrap();
    assert!((res.t - 0.5).abs() < f64::EPSILON);
    assert!(
      (res.uv[0] - 0.45).abs() < f64::EPSILON
    );
    assert!(
      (res.uv[1] - 0.45).abs() < f64::EPSILON
    );
  }

  #[test]
  fn ray_casting_test_triangle_2() {
    let tri_points = [
      Point3::from([-1., -1., 0.]), //
      Point3::from([-1., 1., 0.]),  //
      Point3::from([1., -1., 0.]),
    ];
    let ray_points = [
      Point3::from([0.1, 0.1, -1.]), //
      Point3::from([0.1, 0.1, 1.]),
    ];
    let res = ray_casting_triangle(
      &ray_points, //
      &tri_points,
    );
    assert!(res.is_none());
  }
}
