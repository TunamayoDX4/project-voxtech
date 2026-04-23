enable wgpu_mesh_shader;

// カメラ
struct CameraUniform {
  view_proj: mat4x4<f32>,
}
@group(0) @binding(0) var<uniform> camera: CameraUniform;

// タスクペイロード構造体
struct TaskPayload {
  cell_position: array<vec3<u32>, 64>,
  visible: u32, 
}

// 四角形の座標(固定)
const positions = array<vec4<u32>, 8> (
  vec4<u32>(0u, 0u, 0u, 1u), 
  vec4<u32>(1u, 0u, 0u, 1u), 
  vec4<u32>(1u, 0u, 1u, 1u), 
  vec4<u32>(0u, 0u, 1u, 1u), 
  vec4<u32>(0u, 1u, 0u, 1u), 
  vec4<u32>(1u, 1u, 0u, 1u), 
  vec4<u32>(1u, 1u, 1u, 1u), 
  vec4<u32>(0u, 1u, 1u, 1u), 
);

// 各面の頂点インデックス(固定)
const face_indices = array<array<u32, 4>, 6>(
  array<u32, 4>(4u, 0u, 3u, 7u), // Wst(-X face)
  array<u32, 4>(1u, 5u, 6u, 2u), // Est(+X face)
  array<u32, 4>(0u, 1u, 2u, 3u), // Sth(-Y face)
  array<u32, 4>(5u, 4u, 7u, 6u), // Nth(+Y face)
  array<u32, 4>(4u, 5u, 1u, 0u), // Btm(-Z face)
  array<u32, 4>(3u, 2u, 6u, 7u), // Top(+Z face)
);

// 四角形のオフセット位置(固定)
const position_offset = vec4<u32>(
  1u, 1u, 1u, 0u
);

// 四角形の頂点インデックス(固定)
const indices = array<vec3<u32>, 2>(
  vec3<u32>(0u, 1u, 2u),
  vec3<u32>(0u, 2u, 3u), 
);

// 四角形の頂点カラー(固定)
const colors = array<vec4<f32>, 4>(
  vec4<f32>(1.0, 0.0, 0.0, 1.0),
  vec4<f32>(0.0, 1.0, 0.0, 1.0),
  vec4<f32>(0.0, 0.0, 1.0, 1.0),
  vec4<f32>(0.0, 0.0, 0.0, 1.0),
);

// 頂点構造体
struct VertexOut {
  @builtin(position) pos: vec4<f32>, 
  @location(0) color: vec4<f32>,
}

// プリミティブ出力構造体
struct PrimitiveOut {
  @builtin(triangle_indices) indices: vec3<u32>,
  @builtin(cull_primitive) cull: bool, 
  @per_primitive @location(1) color_mask: vec4<f32>,
}

// プリミティブ入力構造体
struct PrimitiveIn {
  @per_primitive @location(1) color_mask: vec4<f32>,
}

// メッシュ出力構造体
struct MeshOut {
  @builtin(vertex_count) vertex_count: u32, 
  @builtin(primitive_count) primitive_count: u32,
  @builtin(vertices) vertices: array<VertexOut, 256>,
  @builtin(primitives) primitives: array<PrimitiveOut, 128>,
}

// タスクペイロード変数
var<task_payload> task_payload: TaskPayload;

@task
@payload(task_payload)
@workgroup_size(4, 4, 4)
fn task_main(
  @builtin(local_invocation_index) loc_idx: u32,
  @builtin(global_invocation_id) glob_id: vec3<u32>,
) -> @builtin(mesh_task_size) vec3<u32> {

  let glob_id2 = (glob_id / 4u) * 64u;

  task_payload.cell_position[loc_idx % 64u] = vec3<u32>(loc_idx % 4u, loc_idx / 4u % 4u, loc_idx / 16u % 4u) * 16u;
  task_payload.cell_position[loc_idx % 64u] += glob_id2;
  task_payload.visible = 1u;

  workgroupBarrier();

  return vec3<u32>(64u, 64u, 6u); // 64Cell, 64Chunk, 6Faces
}

// メッシュ出力変数
var<workgroup> mesh_out: MeshOut;

@mesh(mesh_out)
@payload(task_payload)
@workgroup_size(4, 4, 4)
fn mesh_main(
  @builtin(local_invocation_index) loc_idx: u32, 
  @builtin(global_invocation_id) glob_id: vec3<u32>, 
) {
  if (loc_idx == 63u) {
    mesh_out.vertex_count = 256u;
    mesh_out.primitive_count = 128u;
  }

  // 四角形を構成する頂点を出力
  for (var i: u32 = 0u; i < 4u; i = i + 1u) {
    let index = face_indices[(glob_id.z >> 2u) % 6u];
    let vert = 
      vec4<f32>(positions[index[i]] + vec4<u32>(
        loc_idx & 3u * position_offset.x,
        (loc_idx >> 2u) & 3u * position_offset.y,
        (loc_idx >> 4u) & 3u * position_offset.z,
        0u
      ) + vec4<u32>(
        (glob_id.x >> 2u) & 3u,
        (glob_id.x >> 4u) & 3u,
        (glob_id.x >> 6u) & 3u,
        0u
      ) * 4u + vec4<u32>(
        task_payload.cell_position[glob_id.y >> 2u], 0u
      ));
    mesh_out.vertices[i + loc_idx * 4u].pos = 
      camera.view_proj * vert;
    mesh_out.vertices[i + loc_idx * 4u].color = colors[i];
  }

  // 四角形を構成するプリミティブを出力
  for (var j: u32 = 0u; j < 2u; j = j + 1u) {
    mesh_out.primitives[j + loc_idx * 2u].indices = indices[j] + vec3<u32>(loc_idx * 4u);
    mesh_out.primitives[j + loc_idx * 2u].cull = task_payload.visible == 0u;
    mesh_out.primitives[j + loc_idx * 2u].color_mask = vec4<f32>(
      1.0, 1.0, 1.0, 1.0
    );
  }
}

@fragment
fn frag_main(
  vertex: VertexOut, 
  primitive: PrimitiveIn,
) -> @location(0) vec4<f32> {
  return vertex.color * primitive.color_mask;
}