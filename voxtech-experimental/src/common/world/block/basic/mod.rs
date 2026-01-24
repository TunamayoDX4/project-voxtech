//! Basic Block

use bytemuck::{Pod, Zeroable};

pub trait BasicBlockElement:
  Copy + Zeroable + Pod
{
}

#[repr(C)]
#[derive(
  Debug, Clone, Copy, PartialEq, Eq, Zeroable, Pod,
)]
pub struct LoID(u8);
impl BasicBlockElement for LoID {}

#[repr(C)]
#[derive(
  Debug, Clone, Copy, PartialEq, Eq, Zeroable, Pod,
)]
pub struct HiID(u8);
impl BasicBlockElement for HiID {}

#[repr(C)]
#[derive(
  Debug, Clone, Copy, PartialEq, Eq, Zeroable, Pod,
)]
pub struct SysTag(u8);
impl BasicBlockElement for SysTag {}

#[repr(C)]
#[derive(
  Debug, Clone, Copy, PartialEq, Eq, Zeroable, Pod,
)]
pub struct UserTag(u8);
impl BasicBlockElement for UserTag {}
