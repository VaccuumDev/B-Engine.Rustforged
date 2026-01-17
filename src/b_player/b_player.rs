use bevy::prelude::*;
use bevy_ahoy::prelude::*;
use bevy_enhanced_input::prelude::*;

pub struct BPlayer;

impl Plugin for BPlayer {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, spawn_player);
    }
}

fn spawn_player(bengine: Commands) {
    let player = bengine
        .spawn((
            CharacterController::default(),
            Transform::from_translation(Vec3::ZERO),
            PlayerInput,
            actions!(PlayerInput[
            (
                Action::<Movement>::new(),
                DeadZone::default(),
                Bindings::spawn((
                        Cardinal::wasd_keys(),
                        Axial::left_stick()
                ))
            ),
            (
                Action::<Jump>::new(),
                bindings![KeyCode::Space, GamepadButton::South],
            ),
            (
                Action::<Crouch>::new(),
                bindings![KeyCode::ControlLeft, GamepadButton::LeftTrigger]
            ),
            (
                Action::<RotateCamera>::new(),
                Scale::splat(0.04),
                Bindings::spawn((
                        Spawn(Binding::mouse_motion()),
                        Axial::right_stick()
                ))
            ),
            ]),
        ))
        .id;

    bengine.spawn((
        Camera3d::default(),
        Hdr,
        DistanceFog {
            color: Color::srgba_u8(2, 64, 200, 255),
            falloff: FogFalloff::Exponential { density: 0.5 },
            ..default()
        },
        MotionBlur::default(),
        CharacterControllerCameraOf::new(player),
    ));
    info!("Camera spawned");
}
