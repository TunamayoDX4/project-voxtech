struct CameraUniform {
  view_proj: mat4x4<f32>,
}
@group(0) @binding(0) var<uniform> camera: CameraUniform;

struct VertexInput {
  @location(0) position: vec4<f32>,
}

struct VertexOutput {
  @builtin(position) position: vec4<f32>,
  @location(0) color: vec4<f32>,
}

struct InstanceInput {
  @location(5) position: vec4<f32>,
  @location(6) color: vec4<f32>,
}

@vertex
fn vs_main(
  model: VertexInput,
  instance: InstanceInput,
) -> VertexOutput {
  var output: VertexOutput;
  output.position = camera.view_proj * (
    model.position + instance.position
  );
  output.color = instance.color;
  return output;
}

@fragment
fn fs_main(in: VertexOutput) -> @location(0) vec4<f32> {
  return in.color;
}
