enable wgpu_mesh_shader;

/* ------------ 型定義 ------------ */

struct CameraUniform {
  mvp: mat4x4<f32>,
}

struct TaskPayload {
  cell_position: array<vec3<u32>, 64>,
  visible: u32, 
}

struct VertexOut {
  @builtin(position) pos: vec4<f32>, 
  @location(0) color: vec4<f32>,
}

struct PrimitiveOut {}

struct PrimitiveIn {}

struct MeshOut {}

/* ------------ 定数定義 ------------ */

const VERTICES = array<vec4<u32>, 8> (
  vec4<u32>(0u, 0u, 0u, 1u), 
  vec4<u32>(1u, 0u, 0u, 1u), 
  vec4<u32>(1u, 0u, 1u, 1u), 
  vec4<u32>(0u, 0u, 1u, 1u), 
  vec4<u32>(0u, 1u, 0u, 1u), 
  vec4<u32>(1u, 1u, 0u, 1u), 
  vec4<u32>(1u, 1u, 1u, 1u), 
  vec4<u32>(0u, 1u, 1u, 1u), 
);

const TRI_INDICES = array<vec3<u32>, 2>(
  vec3<u32>(0u, 1u, 2u),
  vec3<u32>(0u, 2u, 3u), 
);

const FACE_INDICES = array<array<u32, 4>, 6> (
  array<u32, 4>(4u, 0u, 3u, 7u), // Wst(-X face)
  array<u32, 4>(1u, 5u, 6u, 2u), // Est(+X face)
  array<u32, 4>(0u, 1u, 2u, 3u), // Sth(-Y face)
  array<u32, 4>(5u, 4u, 7u, 6u), // Nth(+Y face)
  array<u32, 4>(4u, 5u, 1u, 0u), // Btm(-Z face)
  array<u32, 4>(3u, 2u, 6u, 7u), // Top(+Z face)
);

const POSITION_OFFSET = vec4<u32>(
  1u, 1u, 1u, 0u
);

/* ------------ バインディング定義 ------------ */

@group(0) @binding(0) var<uniform> camera: CameraUniform;
var<task_payload> task_payload: TaskPayload;
var<workgroup> mesh_out: MeshOut;

/* ------------ タスクシェーダ ------------ */


/* ------------ メッシュシェーダ ------------ */


/* ------------ フラグメントシェーダ ------------ */

