use avian3d::prelude::{AngularVelocity, Collider, RigidBody};
use b_engine::{BEngine, b_elements::* /*b_physics::physics::PhysBody*/};
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
        Mesh3d(meshes.add(Cuboid::from_length(1f32))),
        MeshMaterial3d(materials.add(Color::WHITE)),
        Element::new(MaterialType::Metal, 255, 255),
        t,
        RigidBody::Dynamic,
        Collider::cuboid(1f32, 1f32, 1f32),
        AngularVelocity(vec3(2.5, 3.5, 1.5)),
    ));
    bengine.spawn((
        Mesh3d(meshes.add(Plane3d::new(Vec3::Y, vec2(12f32, 12f32)))),
        MeshMaterial3d(materials.add(Color::linear_rgb(0f32, 1f32, 0f32))),
        Transform::from_translation(Vec3::ZERO),
        Element::new(MaterialType::Metal, 255, 255),
        RigidBody::Static,
        Collider::cuboid(24f32, 0.1, 24f32),
    ));
}
