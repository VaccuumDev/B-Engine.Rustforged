use avian3d::prelude::{Collider, RigidBody};
use b_engine::{
    BEngine,
    b_player::{Player, PlayerCamera},
    b_ui::blocks::{BUiBlock, BUiRoot},
};
use bevy::{prelude::*, sprite::Anchor};

fn main() {
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
    //Spawn player
    bengine.spawn((Player::default(), children![(PlayerCamera::default())]));

    //Spawn ui
    bengine.spawn((
        BUiRoot::default(),
        children![BUiBlock::new(Anchor::CENTER, (50.0, 50.0), (50.0, 50.0))],
    ));

    // Spawn scene
    bengine.spawn((
        Mesh3d(meshes.add(Plane3d::new(Vec3::Y, vec2(12f32, 12f32)))),
        MeshMaterial3d(materials.add(Color::linear_rgb(0f32, 1f32, 0f32))),
        Transform::from_translation(Vec3::NEG_Y),
        RigidBody::Static,
        Collider::cuboid(24f32, 0.1, 24f32),
    ));
    bengine.spawn(DirectionalLight::default());
}
