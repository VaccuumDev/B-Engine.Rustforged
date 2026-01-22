struct Params {
  width: u32,
  height: u32,
  octaves: u32,
  seed: u32,
  lacunarity: f32,
  gain: f32,
  scale: f32,
  _pad: f32,
};
@group(0) @binding(0) var<uniform> params: Params;
@group(0) @binding(1) var rw_tex: texture_storage_2d<rgba8unorm, write>;

// integer wang hash
fn wang_hash(u: u32) -> u32 {
  var x = u;
  x = (x ^ 61u) ^ (x >> 16u);
  x = x + (x << 3u);
  x = x ^ (x >> 4u);
  x = x * 0x27d4eb2du;
  x = x ^ (x >> 15u);
  return x;
}

fn rand_u(u: u32, v: u32, seed: u32) -> f32 {
  let h = u * 374761393u + v * 668265263u + seed * 1274126177u;
  let w = wang_hash(h);
  // keep lower 24 bits -> [0,1)
  let frac = f32(w & 0x00FFFFFFu) / 16777216.0;
  return frac;
}

fn noise(grid_x: i32, grid_y: i32, seed: u32) -> f32 {
  return rand_u(u32(grid_x), u32(grid_y), seed);
}

fn smooth_noise(xf: f32, yf: f32, frequency: f32, seed: u32) -> f32 {
  let ux = xf * frequency;
  let uy = yf * frequency;
  let ix = i32(floor(ux));
  let iy = i32(floor(uy));
  let fx = fract(ux);
  let fy = fract(uy);
  let a = noise(ix, iy, seed);
  let b = noise(ix + 1, iy, seed);
  let c = noise(ix, iy + 1, seed);
  let d = noise(ix + 1, iy + 1, seed);
  let u = fx * fx * (3.0 - 2.0 * fx);
  let v = fy * fy * (3.0 - 2.0 * fy);
  let lerp1 = mix(a, b, u);
  let lerp2 = mix(c, d, u);
  return mix(lerp1, lerp2, v);
}

@compute @workgroup_size(8,8)
fn main(@builtin(global_invocation_id) gid: vec3<u32>) {
  if (gid.x >= params.width || gid.y >= params.height) {
    return;
  }
  let ix = gid.x;
  let iy = gid.y;
  let xf = f32(ix) / f32(params.width);
  let yf = f32(iy) / f32(params.height);

  var amplitude = 1.0;
  var frequency = 1.0;
  var sum = 0.0;
  var maxv = 0.0;

  for (var o: u32 = 0u; o < params.octaves; o = o + 1u) {
    let n = smooth_noise(xf, yf, frequency * params.scale, params.seed + o * 9781u);
    sum = sum + n * amplitude;
    maxv = maxv + amplitude;
    amplitude = amplitude * params.gain;
    frequency = frequency * params.lacunarity;
  }
  var value = sum / maxv;
  value = clamp(value, 0.0, 1.0);
  textureStore(rw_tex, vec2<i32>(i32(ix), i32(iy)), vec4<f32>(value, value, value, 1.0));
}

