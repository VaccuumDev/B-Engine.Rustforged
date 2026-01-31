use std::time::Duration;

use crate::{
    b_audio::{StepSounds, WhooshInstanceHandle},
    b_init::BRNG,
};
use avian3d::prelude::{
    Collider, Friction, LinearVelocity, LockedAxes, RigidBody, SpatialQuery, SpatialQueryFilter,
};
use bevy::camera::visibility::RenderLayers;
use bevy_kira_audio::{AudioControl, AudioEasing, AudioInstance, AudioTween};

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
use bevy_lunex::UiSourceCamera;
use rand::Rng;

const TRANSITION: AudioTween = AudioTween::new(Duration::new(2, 0), AudioEasing::Linear);

pub struct BPlayer;

impl Plugin for BPlayer {
    fn build(&self, app: &mut App) {
        app.add_systems(FixedUpdate, controller_update)
            .insert_resource(StepTimer(Timer::from_seconds(0.5, TimerMode::Repeating)));
    }
}

#[derive(Resource)]
struct StepTimer(Timer);

#[derive(Bundle)]
pub struct Player {
    controller: Controller,
    transform: Transform,
    rb: RigidBody,
    friction: Friction,
    col: Collider,
    lock: LockedAxes,
}
impl Default for Player {
    fn default() -> Self {
        Player {
            controller: Controller::new(3.5, 50),
            transform: Transform::from_rotation(Quat::IDENTITY),
            rb: RigidBody::Dynamic,
            friction: Friction::new(1.2),
            col: Collider::capsule(0.5, 2f32),
            lock: LockedAxes::new()
                .lock_rotation_z()
                .lock_rotation_x()
                .lock_rotation_y(),
        }
    }
}
impl Player {
    pub fn new(
        controller: Controller,
        transform: Transform,
        rb: RigidBody,
        friction: Friction,
        col: Collider,
        lock: LockedAxes,
    ) -> Self {
        Player {
            controller: controller,
            transform: transform,
            rb: rb,
            friction: friction,
            col: col,
            lock: lock,
        }
    }
}

#[derive(Bundle)]
pub struct PlayerCamera {
    transform: Transform,
    cam: Camera3d,
    hdr: Hdr,
    dist_fog: DistanceFog,
    projection: Projection,
    exposure: Exposure,
    tonemapping: Tonemapping,
    bloom: Bloom,
    fxaa: Fxaa,
    uisource: UiSourceCamera<0>,
    layers: RenderLayers,
}
impl Default for PlayerCamera {
    fn default() -> Self {
        PlayerCamera {
            transform: Transform::from_rotation(Quat::IDENTITY),

            cam: Camera3d::default(),
            hdr: Hdr,
            dist_fog: DistanceFog {
                color: Color::srgb_u8(183, 251, 251),
                falloff: FogFalloff::Exponential { density: 0.02 },
                ..default()
            },
            projection: Projection::from(PerspectiveProjection {
                fov: 90.0_f32.to_radians(),
                ..default()
            }),
            exposure: Exposure { ev100: 13.0 },
            tonemapping: Tonemapping::AcesFitted,
            bloom: Bloom::NATURAL,
            fxaa: Fxaa::default(),
            uisource: UiSourceCamera::<0>,
            layers: RenderLayers::from_layers(&[0, 2]),
        }
    }
}

#[allow(unused)]
#[derive(Component)]
pub struct Controller {
    max_speed: f32,
    max_slope: i8,
}
impl Controller {
    pub fn new(max_speed: f32, max_slope: i8) -> Self {
        Controller {
            max_speed: max_speed,
            max_slope: max_slope,
        }
    }
}

fn controller_update(
    keyboard: Res<ButtonInput<KeyCode>>,
    mut p: Single<(&mut LinearVelocity, &mut Transform, &Controller, Entity), With<Controller>>,
    mut mouse: MessageReader<MouseMotion>,
    mut cam: Single<&mut Transform, (With<Camera3d>, Without<Controller>)>,
    time: Res<Time>,
    sq: SpatialQuery,
    mut rng: ResMut<BRNG>,
    whoosh: Res<WhooshInstanceHandle>,
    mut audio_instances: ResMut<Assets<AudioInstance>>,
    audio: Res<bevy_kira_audio::prelude::Audio>,
    step_sounds: Res<StepSounds>,
    mut step_timer: ResMut<StepTimer>,
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

        p.1.rotate_y(-dx);
        cam.rotate_local_x(-dy);
    }

    // Move by pressing WASD
    let mut a: Vec3A = Vec3A::ZERO;
    for (key, dir) in [
        (KeyCode::KeyW, p.1.forward().to_vec3a()),
        (KeyCode::KeyA, p.1.left().to_vec3a()),
        (KeyCode::KeyS, p.1.back().to_vec3a()),
        (KeyCode::KeyD, p.1.right().to_vec3a()),
        //(KeyCode::ShiftLeft, Vec3::Y),
    ]
    .iter()
    .cloned()
    {
        if keyboard.pressed(key) {
            a += dir;
            step_timer.0.tick(time.delta());
            if step_timer.0.just_finished() {
                let index: usize = rng.r.random_range(..=3);
                audio
                    .play(step_sounds.sounds[index].clone())
                    .with_volume(-12.);
            }
        }
    }

    if keyboard.pressed(KeyCode::Space)
        && sq.cast_ray(
            p.1.translation,
            Dir3::NEG_Y,
            1.51,
            false,
            &SpatialQueryFilter::from_excluded_entities([p.3]),
        ) != None
    {
        p.0.0 += Vec3::Y * 5f32;
    }

    a = a.normalize_or_zero();
    a *= 0.2;
    p.0.0 += a.to_vec3();
    p.0.0.x = p.0.0.x.clamp(-p.2.max_speed, p.2.max_speed);
    p.0.0.z = p.0.0.z.clamp(-p.2.max_speed, p.2.max_speed);

    if let Some(instance) = audio_instances.get_mut(&whoosh.o) {
        if p.0.length() > 15f32 {
            instance.resume(TRANSITION);
        } else {
            instance.pause(AudioTween::default());
        }
    };
}
