@group(0) @binding(0) var<uniform> grid: vec2f;
@group(0) @binding(1) var<storage> cellState: array<u32>;

@vertex
fn vertexMain(
  @location(0) pos: vec2<f32>,
  @builtin(instance_index) instance: u32
) -> @builtin(position) vec4<f32> {
  let i = f32(instance);
  let cell = vec2f(i % grid.x, floor(i / grid.x));
  let state = f32(cellState[instance]);
  let cellOffset = cell / grid * 2.0;
  let gridPos = (pos*state + 1.0) / grid - 1.0 + cellOffset;
  return vec4f(gridPos, 0.0, 1.0);
}

@fragment
fn fragmentMain() -> @location(0) vec4<f32> {
  return vec4(1.0, 0.0, 0.0, 1.0);
}