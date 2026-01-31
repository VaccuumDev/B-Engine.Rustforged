use bevy::{
    prelude::*,
    window::{CursorGrabMode, CursorOptions},
};
use bevy_pipelines_ready::PipelinesReady;

pub struct BInit;

impl Plugin for BInit {
    fn build(&self, game: &mut App) {
        game.add_plugins((
            DefaultPlugins.set(WindowPlugin {
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
            }),
            avian3d::PhysicsPlugins::default(),
        ))
        .insert_resource(PipelinesReady::default())
        .insert_resource(ClearColor(Color::linear_rgb(0.3, 0.3, 0.6)));
    }
}
