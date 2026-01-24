use crate::{b_init::GameSettings, b_physics::physics::PhysBody};
use bevy::{
    anti_alias::fxaa::Fxaa,
    camera::Exposure,
    core_pipeline::tonemapping::Tonemapping,
    input::mouse::MouseMotion,
    pbr::{Atmosphere, AtmosphereSettings, ScatteringMedium},
    post_process::{bloom::Bloom, motion_blur::MotionBlur},
    prelude::*,
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

fn spawn_player(
    mut bengine: Commands,
    mut scaterring: ResMut<Assets<ScatteringMedium>>,
    settings: Res<GameSettings>,
) {
    let transform: Transform =
        Transform::from_xyz(0.0, 4.0, -1.0).looking_at(vec3(0f32, 4f32, 0.0), Vec3::Y);
    bengine.spawn((
        children![(
            transform,
            Camera3d::default(),
            Hdr,
            DistanceFog {
                color: Color::srgb_u8(183, 251, 251),
                falloff: FogFalloff::Exponential { density: 0.1 },
                ..default()
            },
            Projection::from(PerspectiveProjection {
                fov: 90.0_f32.to_radians(),
                ..default()
            }),
            /*either!(settings.atmosphere => // TODO: settings
                    Some(Atmosphere::earthlike(scaterring.add(ScatteringMedium::default())))
                    ,,
                    None
            ),
            AtmosphereSettings::default(),
            either! (settings.motion_blur =>
                Some(MotionBlur::default()) ,, None),*/
            MotionBlur::default(),
            Exposure { ev100: 13.0 },
            Tonemapping::AcesFitted,
            Bloom::NATURAL,
            Fxaa::default(),
        )],
        transform,
        PhysBody::new(transform, vec3(0.5, 2.0, 0.5), 5f32),
        Controller::new(),
    ));
}

fn controller_update(
    keyboard: Res<ButtonInput<KeyCode>>,
    mut p: Single<(&mut PhysBody, &mut Transform, &mut Controller), With<Controller>>,
    mut mouse: MessageReader<MouseMotion>,
    mut cam: Single<&mut Transform, (With<Camera3d>, Without<Controller>)>,
    time: Res<Time>,
) {
    // Rotate camera by moving mouse
    let mut cursor_delta = Vec2::ZERO;
    for e in mouse.read() {
        cursor_delta += e.delta * 0.001;
    }
    if !(cursor_delta == Vec2::ZERO) {
        info!("{}", cam.rotation);
        //cam.rotation.y += cursor_delta.x;
        //cam.rotation.z += cursor_delta.y;

        let dt = time.delta_secs();
        let dx = cursor_delta.x * 100f32 * dt;
        let dy = cursor_delta.y * 100f32 * dt;

        cam.rotate_y(-dx);
        cam.rotate_local_x(-dy);
    }

    // Move by pressing WASD
    for (key, dir) in [
        (KeyCode::KeyW, Vec3::Z),
        (KeyCode::KeyA, Vec3::X),
        (KeyCode::KeyS, -Vec3::Z),
        (KeyCode::KeyD, -Vec3::X),
        (KeyCode::ShiftLeft, -Vec3::Y),
        (KeyCode::Space, Vec3::new(0f32, 15f32, 0f32)),
    ]
    .iter()
    .cloned()
    {
        if keyboard.pressed(key) {
            p.0.apply_acc(dir * 20f32);
            //p.0.apply_moment(dir.to_vec3a() * 0.001);
        }
    }

    // Some debug
    //info!("{}", p.1.translation);
}
