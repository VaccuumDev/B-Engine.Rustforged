use bevy::{prelude::*, render::render_resource::AsBindGroup};

const FRAGMENT_PATH: &str = "assets/shaders/ui/glass/glass.frag";
const VERTEX_PATH: &str = "assets/shaders/ui/glass/glass.vert";

#[derive(Asset, AsBindGroup, TypePath, Clone)]
pub struct GlassUiMat {
    #[uniform(0)]
    tint: LinearRgba,
}
impl Material for GlassUiMat {
    fn vertex_shader() -> bevy::shader::ShaderRef {
        VERTEX_PATH.into()
    }
    fn fragment_shader() -> bevy::shader::ShaderRef {
        FRAGMENT_PATH.into()
    }
}
