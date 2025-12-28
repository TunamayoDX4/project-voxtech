//! プレイヤーに関する処理を実装するためのモジュール

use nalgebra::{Point3, Vector3};

pub mod control;

/// プレイヤーの回転に関わる情報
#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct PlayerRotation {
	pub yaw: f64, 
	pub pitch: f64, 
	pub roll: f64, 
}
impl Default for PlayerRotation {
	fn default() -> Self {
		Self { 
			yaw: 0., 
			pitch: 0., 
			roll: 0. 
		}
	}
}

/// プレイヤーのオブジェクト
#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct Player {
	pub position: Point3<f64>, 
	pub velocity: Vector3<f64>, 
	pub rotation: PlayerRotation, 
}
impl Default for Player {
	fn default() -> Self {
		Self { 
			position: [0., 0., 0.].into(), 
			velocity: [0., 0., 0.].into(), 
			rotation: Default::default() 
		}
	}
}
impl Player {
	pub fn new(
		position: impl Into<Point3<f64>>, 
		velocity: impl Into<Vector3<f64>>, 
		rotation: PlayerRotation, 
	) -> Self { Self {
		position: position.into(), 
		velocity: velocity.into(), 
		rotation: rotation, 
	}}
}
