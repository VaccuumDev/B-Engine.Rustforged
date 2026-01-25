use crate::b_init::GameSettings;
use avian3d::prelude::{Collider, LinearVelocity, LockedAxes, RigidBody};
#[allow(unused_imports)]
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

#[allow(unused)]
fn spawn_player(
    mut bengine: Commands,
    mut scaterring: ResMut<Assets<ScatteringMedium>>,
    settings: Res<GameSettings>,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    let transform: Transform =
        Transform::from_xyz(0.0, 0.1, -1.0).looking_at(vec3(0f32, 4f32, 0.0), Vec3::Y);
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
            //MotionBlur::default(),
            Exposure { ev100: 13.0 },
            Tonemapping::AcesFitted,
            Bloom::NATURAL,
            Fxaa::default(),
        )],
        transform,
        //PhysBody::new(transform, vec3(0.5, 2.0, 0.5), 5f32),
        RigidBody::Dynamic,
        Collider::capsule(0.5, 2f32),
        LockedAxes::new().lock_rotation_z().lock_translation_x(),
        Controller::new(),
        Mesh3d(meshes.add(Cuboid::from_size(vec3(1f32, 2f32, 1f32)))),
        MeshMaterial3d(materials.add(StandardMaterial::default())),
    ));
}

fn controller_update(
    keyboard: Res<ButtonInput<KeyCode>>,
    mut p: Single<(&mut LinearVelocity, &mut Transform), With<Controller>>,
    mut mouse: MessageReader<MouseMotion>,
    mut cam: Single<&mut Transform, (With<Camera3d>, Without<Controller>)>,
    time: Res<Time>,
) {
    // Camera controls
    let mut cursor_delta = Vec2::ZERO;
    for e in mouse.read() {
        cursor_delta += e.delta * 0.001;
    }
    if !(cursor_delta == Vec2::ZERO) {
        let dt = time.delta_secs();
        let dx = cursor_delta.x * 100f32 * dt;
        let dy = cursor_delta.y * 100f32 * dt;

        p.1.rotate_y(-dx); // HACK: Idk looks kinda strange
        cam.rotate_local_x(-dy);
    }

    // Move by pressing WASD
    let mut v: Vec3A = Vec3A::ZERO;
    for (key, dir) in [
        (KeyCode::KeyW, Vec3A::NEG_Z),
        (KeyCode::KeyA, Vec3A::NEG_X),
        (KeyCode::KeyS, Vec3A::Z),
        (KeyCode::KeyD, Vec3A::X),
        //(KeyCode::ShiftLeft, Vec3::Y),
        (KeyCode::Space, Vec3A::Y),
    ]
    .iter()
    .cloned()
    {
        if keyboard.pressed(key) {
            v += dir;
        }
    }
    v = v.normalize_or_zero();
    v *= 7f32; // Player speed
    p.0.0 += v.to_vec3();
    let current_speed = p.0.length();
    if current_speed > 0.0 {
        // Apply friction
        p.0.0 = p.0.0 / current_speed
            * (current_speed - current_speed * 20f32 * time.delta_secs()).max(0.0)
    }

    // Some debug
    //info!("{}", p.1.translation);
}
