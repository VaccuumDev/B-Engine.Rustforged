use b_engine::{BEngine, b_elements::*, b_physics::physics::PhysBody};
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
    let t = Transform::from_xyz(0f32, 10f32, 0f32);
    bengine.spawn((
        Mesh3d(meshes.add(Cuboid::default())),
        MeshMaterial3d(materials.add(Color::WHITE)),
        Element::new(MaterialType::Metal, 255, 255),
        t,
        PhysBody::new(t, vec3(0.5, 0.5, 0.5)),
    ));
    bengine.spawn((
        Mesh3d(meshes.add(Plane3d::default())),
        MeshMaterial3d(materials.add(Color::linear_rgb(0f32, 1f32, 0f32))),
        Transform::from_scale(Vec3::new(12f32, 1f32, 12f32)),
        Element::new(MaterialType::Metal, 255, 255),
    ));
}
