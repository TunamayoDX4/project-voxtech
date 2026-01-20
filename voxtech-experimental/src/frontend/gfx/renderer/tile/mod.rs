use std::sync::Arc;

use parking_lot::RwLock;
use wgpu::util::DeviceExt;

use super::WGPUCtx;

pub struct OpaqueTileRendererHandler {}

pub struct OpaqueTileRenderer {
  render_pipe_layout: wgpu::PipelineLayout,
  render_pipe: wgpu::RenderPipeline,
}
