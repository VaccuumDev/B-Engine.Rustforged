use bevy::{post_process::motion_blur::MotionBlur, prelude::*, render::view::Hdr};
use bevy_ahoy::prelude::AhoyPlugin;
use bevy_pipelines_ready::PipelinesReady;

pub struct BInit;

impl Plugin for BInit {
    fn build(&self, game: &mut App) {
        game.add_plugins(DefaultPlugins)
            .add_plugins(AhoyPlugin)
            .insert_resource(PipelinesReady::default())
            .add_systems(Startup, (global_startup, splash_screen));
        //.init_resource::<GameSettings>();
    }
}

#[allow(unused_parens)]
fn global_startup(mut bengine: Commands) {
    info!("Global Startup");

    bengine.spawn((
        DirectionalLight::default(),
        Transform::from_xyz(3.0, 3.0, 3.0).looking_at(Vec3::ZERO, Vec3::Y),
    ));
}

#[allow(unused_parens)]
fn splash_screen(mut bengine: Commands) {
    bengine.spawn((Text::new("B-Engine")));
    /*if (PipelinesReady::get() != 0) {
        despawn();
    }*/
}
/*
#[derive(Resource)]
struct GameSettings {
    motion_blur: bool,
    bloom: u8,
}
*/
