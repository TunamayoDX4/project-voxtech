struct CameraUniform {
  view_proj: mat4x4<f32>,
}
@group(0) @binding(0) var<uniform> camera: CameraUniform;

struct InstanceInput {
  @location(8) stride: u32, 
  @location(9) offset: vec2<f32>, 
  @location(10) color: vec2<f32>,
}

struct VertexInput {
  @location(0) position: vec4<f32>, 
  @location(1) tex_coord: vec2<f32>,
  @location(2) color: vec4<f32>,
}

struct VertexOutput {
  @builtin(position) position: vec4<f32>,
  @location(1) color: vec4<f32>,
};

@vertex
fn vs_main(
  model: VertexInput,
  instance: InstanceInput,
) -> VertexOutput {
  var out: VertexOutput;
  var stride = vec4<f32>(
    f32((instance.stride >> 4) & 0xC | (instance.stride >> 0) & 0x3), 
    f32((instance.stride >> 6) & 0xC | (instance.stride >> 2) & 0x3),
    f32((instance.stride >> 8) & 0xC | (instance.stride >> 4) & 0x3),
    0.0,
  );
  out.position = camera.view_proj * (model.position + stride);
  out.color = model.color;
  return out;
}

@fragment
fn fs_main(
  in: VertexOutput,
) -> @location(0) vec4<f32> {
  return in.color;
}