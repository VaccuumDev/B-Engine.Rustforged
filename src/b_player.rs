use crate::b_physics::physics::PhysBody;
use bevy::{
    input::keyboard::KeyboardInput, post_process::motion_blur::MotionBlur, prelude::*,
    render::view::Hdr,
};

pub struct BPlayer;

impl Plugin for BPlayer {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, spawn_player)
            .add_systems(Update, controller_update);
    }
}

#[derive(Component)]
struct Controller;

impl Controller {
    pub fn new() -> Self {
        Controller
    }
}

fn spawn_player(mut bengine: Commands) {
    let transform: Transform = Transform::from_xyz(0.0, 2.0, 8.0).looking_at(Vec3::ZERO, Vec3::Y);
    bengine.spawn((
        Camera3d::default(),
        Hdr,
        DistanceFog {
            color: Color::srgb_u8(183, 251, 251),
            falloff: FogFalloff::Exponential { density: 0.1 },
            ..default()
        },
        MotionBlur::default(),
        transform,
        PhysBody::new(transform, vec3(0.5, 2.0, 0.5)),
        Controller::new(),
    ));
}

fn controller_update(
    key: Res<ButtonInput<KeyCode>>,
    mut query: Query<&mut PhysBody, With<Controller>>,
) {
    if (key.pressed(KeyCode::KeyW)) {
        for mut a in query.iter_mut() {
            a.apply_force(vec3(0f32, 0f32, 0.2));
        }
    }
}
