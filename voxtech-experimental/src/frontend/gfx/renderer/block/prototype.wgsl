enable wgpu_mesh_shader;

// カメラ
struct CameraUniform {
  view_proj: mat4x4<f32>,
}
@group(0) @binding(0) var<uniform> camera: CameraUniform;

// タスクペイロード構造体
struct TaskPayload {
  color_mask: vec4<f32>,
  visible: u32, 
}

// 四角形の座標(固定)
const positions = array<vec4<f32>, 4> (
  vec4<f32>(0.0, 0.0, 0.0, 1.0),
  vec4<f32>(1.0, 0.0, 0.0, 1.0),
  vec4<f32>(1.0, 0.0, 1.0, 1.0),
  vec4<f32>(0.0, 0.0, 1.0, 1.0),
);

// 四角形のオフセット位置(固定)
const position_offset = vec4<f32>(
  1.0, 0.0, 1.0, 0.0
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
  @builtin(vertices) vertices: array<VertexOut, 64>,
  @builtin(primitives) primitives: array<PrimitiveOut, 32>,
}

// タスクペイロード変数
var<task_payload> task_payload: TaskPayload;
var<workgroup> workgroup_data: f32;

@task
@payload(task_payload)
@workgroup_size(1, 1, 1)
fn task_main() -> @builtin(mesh_task_size) vec3<u32> {
  // ワークグループデータの初期化
  workgroup_data = 1.0;

  task_payload.color_mask = vec4<f32>(1.0, 1.0, 1.0, 1.0);
  task_payload.visible = 1u;

  return vec3<u32>(2u, 2u, 1u); // 2プリミティブワークグループ
}

// メッシュ出力変数
var<workgroup> mesh_out: MeshOut;

@mesh(mesh_out)
@payload(task_payload)
@workgroup_size(4, 4, 1)
fn mesh_main(
  @builtin(local_invocation_index) loc_idx: u32, 
  @builtin(global_invocation_id) glob_id: vec3<u32>, 
) {
  if (loc_idx == 0u) {
    // ワークグループデータの確認
    workgroup_data = workgroup_data + 1.0;
  }
  if (loc_idx == 15u) {
    mesh_out.vertex_count = 64u;
    mesh_out.primitive_count = 32u;
  }

  for (var i: u32 = 0u; i < 4u; i = i + 1u) {
    let vert = 
      positions[i] + vec4<f32>(
        f32(loc_idx % 4u) * position_offset.x,
        0.0,
        f32(loc_idx / 4u) * position_offset.z,
        0.0
      ) + vec4<f32>(
        f32(glob_id.x / 4u) * 4.0 - 4.0,
        0.0,
        f32(glob_id.y / 4u) * 4.0 - 4.0,
        0.0
      );
    mesh_out.vertices[i + loc_idx * 4u].pos = 
      camera.view_proj * vert;
    mesh_out.vertices[i + loc_idx * 4u].color = colors[i];
  }
  for (var j: u32 = 0u; j < 2u; j = j + 1u) {
    mesh_out.primitives[j + loc_idx * 2u].indices = indices[j] + vec3<u32>(loc_idx * 4u);
    mesh_out.primitives[j + loc_idx * 2u].cull = task_payload.visible == 0u;
    mesh_out.primitives[j + loc_idx * 2u].color_mask = task_payload.color_mask;
  }
}

@fragment
fn frag_main(
  vertex: VertexOut, 
  primitive: PrimitiveIn,
) -> @location(0) vec4<f32> {
  return vertex.color * primitive.color_mask;
}