enable wgpu_mesh_shader;

// タスクペイロード構造体
struct TaskPayload {
  color_mask: vec4<f32>,
  visible: u32, 
}

// 三角形の座標(固定)
const positions = array<vec4<f32>, 4> (
  vec4<f32>(-0.5,  0.5, 0.0, 1.0),
  vec4<f32>(-0.5, -0.5, 0.0, 1.0),
  vec4<f32>( 0.5, -0.5, 0.0, 1.0),
  vec4<f32>( 0.5,  0.5, 0.0, 1.0),
);

// 三角形の頂点カラー(固定)
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
  @builtin(vertices) vertices: array<VertexOut, 4>,
  @builtin(primitives) primitives: array<PrimitiveOut, 2>,
}

// タスクペイロード変数
var<task_payload> task_payload: TaskPayload;
var<workgroup> workgroup_data: f32;

@task
@payload(task_payload)
@workgroup_size(1)
fn task_main() -> @builtin(mesh_task_size) vec3<u32> {
  // ワークグループデータの初期化
  workgroup_data = 1.0;

  task_payload.color_mask = vec4<f32>(1.0, 1.0, 1.0, 1.0);
  task_payload.visible = 1u;

  return vec3<u32>(1u, 1u, 1u); // 1プリミティブ、1頂点バッファ
}

// メッシュ出力変数
var<workgroup> mesh_out: MeshOut;

@mesh(mesh_out)
@payload(task_payload)
@workgroup_size(1)
fn mesh_main(
  @builtin(local_invocation_index) index: u32, 
  @builtin(global_invocation_id) id: vec3<u32>, 
) {
  mesh_out.vertex_count = 4u;
  mesh_out.primitive_count = 2u;
  workgroup_data = 2.0;

  mesh_out.vertices[0].pos = positions[0];
  mesh_out.vertices[0].color = colors[0];
  mesh_out.vertices[1].pos = positions[1];
  mesh_out.vertices[1].color = colors[1];
  mesh_out.vertices[2].pos = positions[2];
  mesh_out.vertices[2].color = colors[2];
  mesh_out.vertices[3].pos = positions[3];
  mesh_out.vertices[3].color = colors[3];

  mesh_out.primitives[0].indices = vec3<u32>(0u, 1u, 2u);
  mesh_out.primitives[0].cull = task_payload.visible == 0u;
  mesh_out.primitives[0].color_mask = task_payload.color_mask;
  mesh_out.primitives[1].indices = vec3<u32>(0u, 2u, 3u);
  mesh_out.primitives[1].cull = task_payload.visible == 0u;
  mesh_out.primitives[1].color_mask = task_payload.color_mask;
}

@fragment
fn frag_main(
  vertex: VertexOut, 
  primitive: PrimitiveIn,
) -> @location(0) vec4<f32> {
  return vertex.color * primitive.color_mask;
}