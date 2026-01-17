struct CameraData {
    viewMatrix: mat4x4f,
    projectionMatrix: mat4x4f,
};

struct VertexData {
    @builtin(position) position: vec4f,
    @location(0) texcoord: vec3f,
};

const vertices = array<vec3f, 8>(
    vec3f(-1, -1, -1),
    vec3f( 1, -1, -1),
    vec3f( 1, -1,  1),
    vec3f(-1, -1,  1),
    vec3f(-1,  1,  1),
    vec3f( 1,  1,  1),
    vec3f( 1,  1, -1),
    vec3f(-1,  1, -1)
);

const indices = array<u32, 36>(
    0, 1, 2, 2, 3, 0,
    0, 3, 4, 4, 7, 0,
    3, 2, 5, 5, 4, 3,
    2, 1, 6, 6, 5, 2,
    1, 0, 7, 7, 6, 1,
    4, 5, 6, 6, 7, 4
);

@group(0) @binding(0) var<uniform> cameraData: CameraData;

@group(1) @binding(0) var cubemapTexture: texture_cube<f32>;
@group(1) @binding(1) var cubemapSampler: sampler;

@vertex fn cubemap_vert(@builtin(vertex_index) vertexId: u32) -> VertexData {
    var out: VertexData;

    var viewMatrix = cameraData.viewMatrix;
    // Reset translation
    viewMatrix[3] = vec4f(0, 0, 0, 1);

    var viewProjectionMatrix = cameraData.projectionMatrix * viewMatrix;
    var pos = vertices[indices[vertexId]];
    out.position = viewProjectionMatrix * vec4f(pos, 1);
    out.texcoord = pos;

    return out;
}

@fragment fn cubemap_frag(in: VertexData) -> @location(0) vec4f {
    return textureSample(cubemapTexture, cubemapSampler, in.texcoord);
}