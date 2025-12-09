struct CameraUniform {
  view_proj: mat4x4<f32>,
}
@group(0) @binding(0) var<uniform> camera: CameraUniform;

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
) -> VertexOutput {
  var out: VertexOutput;
  out.position = camera.view_proj * model.position;
  out.color = model.color;
  return out;
}

@fragment
fn fs_main(
  in: VertexOutput,
) -> @location(0) vec4<f32> {
  return in.color;
}