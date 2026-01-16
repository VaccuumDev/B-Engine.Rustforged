use bevy::{post_process::motion_blur::MotionBlur, prelude::*, render::view::Hdr};

use crate::b_elements::{Element, MaterialType};

pub struct BInit;

impl Plugin for BInit {
    fn build(&self, game: &mut App) {
        game.add_systems(Startup, global_startup);
        //.init_resource::<GameSettings>();
        //.add_plugins(PhysicsPlugins::default());
    }
}

#[allow(unused_parens)]
fn global_startup(
    mut bengine: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    info!("Global Startup");
    bengine.spawn((
        Mesh3d(meshes.add(Cuboid::default())),
        MeshMaterial3d(materials.add(Color::WHITE)),
        Transform::from_translation(Vec3::ZERO),
        Element::new(MaterialType::Metal, 255, 255),
    ));
    bengine.spawn((
        Camera3d::default(),
        Transform::from_xyz(0.0, 10.0, 20.0).looking_at(Vec3::ZERO, Vec3::Y),
        Hdr,
        DistanceFog {
            color: Color::srgba_u8(2, 64, 200, 255),
            falloff: FogFalloff::Exponential { density: 0.5 },
            ..default()
        },
        MotionBlur::default(),
    ));

    bengine.spawn((
        DirectionalLight::default(),
        Transform::from_xyz(3.0, 3.0, 3.0).looking_at(Vec3::ZERO, Vec3::Y),
    ));
}
/*
#[derive(Resource)]
struct GameSettings {
    motion_blur: bool,
    bloom: u8,
}
*/
