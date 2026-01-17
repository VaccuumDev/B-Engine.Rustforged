use b_engine::{BEngine, b_elements::*};
use bevy::prelude::*;

fn main() {
    //info!("Start.");
    App::new()
        .add_plugins(BEngine)
        .add_systems(Startup, setup)
        .run();
}

fn setup(
    mut bengine: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    bengine.spawn((
        Mesh3d(meshes.add(Cuboid::default())),
        MeshMaterial3d(materials.add(Color::WHITE)),
        Transform::from_xyz(0f32, 4f32, 0f32),
        Element::new(MaterialType::Metal, 255, 255),
    ));
    bengine.spawn((
        Mesh3d(meshes.add(Plane3d::default())),
        MeshMaterial3d(materials.add(Color::linear_rgb(0.1, 0.5, 0.8))),
        Transform::from_translation(Vec3::ZERO),
        Element::new(MaterialType::Metal, 255, 255),
    ));
}
