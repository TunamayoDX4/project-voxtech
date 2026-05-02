use std::collections::VecDeque;

use bytemuck::{Pod, Zeroable};
use wgpu::{util::DeviceExt, Buffer};

#[derive(Debug, Clone, Copy)]
pub struct InstanceBufferKey {
  index: u32,
  generation: u32,
}

pub struct InstanceBufferEntryRef<'a> {
  length: u32,
  buffer: &'a Buffer,
}
impl InstanceBufferEntryRef<'_> {
  pub fn length(&self) -> u32 {
    self.length
  }
  pub fn buffer(&self) -> &Buffer {
    self.buffer
  }
}

pub struct InstanceBufferEntry {
  length: u32,
  buffer: Buffer,
}

#[derive(Default)]
pub struct InstanceBufferArray {
  empty_slot: VecDeque<u32>,
  generation: Vec<u32>,
  buffer_length: Vec<u32>,
  buffer: Vec<Option<Buffer>>,
}
impl InstanceBufferArray {
  pub fn insert(
    &mut self,
    wgpu_ctx: &super::WGPUCtx,
    instance: &[Instance],
  ) -> InstanceBufferKey {
    let buffer =
      wgpu_ctx.device.create_buffer_init(
        &wgpu::util::BufferInitDescriptor {
          label: None,
          contents: bytemuck::cast_slice(
            instance,
          ),
          usage: wgpu::BufferUsages::VERTEX,
        },
      );
    let buffer_length = instance.len() as u32;
    if let Some(index) =
      self.empty_slot.pop_front()
    {
      let generation = self
        .generation
        .get_mut(index as usize)
        .expect("logic error");
      *generation = generation.wrapping_add(1);
      self.buffer[index as usize] =
        Some(buffer);
      self.buffer_length[index as usize] =
        buffer_length;
      InstanceBufferKey {
        index,
        generation: *generation,
      }
    } else {
      let index = self.generation.len() as u32;
      self.generation.push(0);
      self.buffer.push(Some(buffer));
      self.buffer_length.push(buffer_length);
      InstanceBufferKey {
        index,
        generation: 0,
      }
    }
  }

  pub fn remove(
    &mut self,
    key: &InstanceBufferKey,
  ) -> Option<InstanceBufferEntry> {
    if *self
      .generation
      .get(key.index as usize)?
      == key.generation
    {
      self.empty_slot.push_back(key.index);
      let buffer = self.buffer
        [key.index as usize]
        .take()
        .expect("logic error");
      let length =
        self.buffer_length[key.index as usize];

      Some(InstanceBufferEntry {
        length,
        buffer,
      })
    } else {
      None
    }
  }

  pub fn update(
    &mut self,
    key: &InstanceBufferKey,
    wgpu_ctx: &super::WGPUCtx,
    instance: &[Instance],
  ) -> bool {
    println!(
      "{key:?}, GENERATION: {:?}",
      self.generation.get(key.index as usize)
    );
    if self
      .generation
      .get(key.index as usize)
      .filter(|generation| {
        **generation == key.generation
      })
      .is_none()
    {
      return false;
    };
    let index = key.index;
    self.buffer_length[index as usize] =
      instance.len() as u32;
    self.buffer[index as usize] =
      Some(wgpu_ctx.device.create_buffer_init(
        &wgpu::util::BufferInitDescriptor {
          label: None,
          contents: bytemuck::cast_slice(
            instance,
          ),
          usage: wgpu::BufferUsages::VERTEX,
        },
      ));
    true
  }

  pub fn get<'a>(
    &'a self,
    key: &InstanceBufferKey,
  ) -> Option<InstanceBufferEntryRef<'a>> {
    let index = self
      .generation
      .get(key.index as usize)
      .filter(|generation| {
        **generation == key.generation
      })?;
    let length =
      self.buffer_length[*index as usize];
    let buffer = self.buffer[*index as usize]
      .as_ref()
      .unwrap();

    Some(InstanceBufferEntryRef {
      length,
      buffer,
    })
  }

  pub fn iter<'a>(
    &'a self,
  ) -> impl Iterator<Item = InstanceBufferEntryRef<'a>>
  {
    (0..self.buffer.len())
      .filter_map(|index| {
        self.buffer[index]
          .as_ref()
          .map(|buffer| (index, buffer))
      })
      .map(|(index, buffer)| {
        InstanceBufferEntryRef {
          length: self.buffer_length[index],
          buffer,
        }
      })
  }
}

#[repr(C)]
#[derive(Debug, Clone, Copy, Pod, Zeroable)]
pub struct Instance {
  pub position: [f32; 4],
  pub color: [f32; 4],
}
impl Instance {
  pub const ATTRIB: [wgpu::VertexAttribute; 2] = wgpu::vertex_attr_array![
    5 => Float32x4,
    6 => Float32x4,
  ];

  pub fn desc<'a>(
  ) -> wgpu::VertexBufferLayout<'a> {
    wgpu::VertexBufferLayout {
      array_stride: std::mem::size_of::<Self>()
        as _,
      step_mode: wgpu::VertexStepMode::Instance,
      attributes: &Self::ATTRIB,
    }
  }
}
