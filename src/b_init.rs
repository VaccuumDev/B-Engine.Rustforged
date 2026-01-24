use bevy::{
    prelude::*,
    window::{CursorGrabMode, CursorOptions, PrimaryWindow},
};
use bevy_pipelines_ready::PipelinesReady;

pub struct BInit;

impl Plugin for BInit {
    fn build(&self, game: &mut App) {
        game.add_plugins(DefaultPlugins.set(WindowPlugin {
            primary_window: Some(Window {
                title: "B-World".to_string(),
                resizable: true,
                decorations: false,
                ..default()
            }),
            primary_cursor_options: Some(CursorOptions {
                grab_mode: CursorGrabMode::Confined,
                visible: false,
                ..default()
            }),
            ..default()
        }))
        .insert_resource(PipelinesReady::default())
        .insert_resource(ClearColor(Color::linear_rgb(0.3, 0.3, 0.6)))
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
    bengine.spawn((Text::new("B-Engine v0.1.1")));
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
