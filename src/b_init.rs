use bevy::{
    diagnostic::{DiagnosticPath, FrameTimeDiagnosticsPlugin, SystemInformationDiagnosticsPlugin},
    prelude::*,
    window::{CursorGrabMode, CursorOptions},
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
        .add_plugins(FrameTimeDiagnosticsPlugin::default())
        .add_plugins(SystemInformationDiagnosticsPlugin::default())
        .insert_resource(PipelinesReady::default())
        .insert_resource(ClearColor(Color::linear_rgb(0.3, 0.3, 0.6)))
        .insert_resource(GameSettings {
            atmosphere: false,
            motion_blur: true,
        })
        .add_systems(Startup, (global_startup, draw_ui))
        .add_systems(Update, update_dmenu);
        //.init_resource::<GameSettings>();
    }
}

const FRAME_TIME_DIAGNOSTICS: [DiagnosticPath; 2] = [
    FrameTimeDiagnosticsPlugin::FPS,
    FrameTimeDiagnosticsPlugin::FRAME_TIME,
];

const SYSTEM_INFO_DIAGNOSTICS: [DiagnosticPath; 4] = [
    SystemInformationDiagnosticsPlugin::PROCESS_CPU_USAGE,
    SystemInformationDiagnosticsPlugin::PROCESS_MEM_USAGE,
    SystemInformationDiagnosticsPlugin::SYSTEM_CPU_USAGE,
    SystemInformationDiagnosticsPlugin::SYSTEM_MEM_USAGE,
];

#[allow(unused_parens)]
fn global_startup(mut bengine: Commands) {
    info!("Global Startup");

    bengine.spawn((
        DirectionalLight::default(),
        Transform::from_xyz(3.0, 3.0, 3.0).looking_at(Vec3::ZERO, Vec3::Y),
    ));
}

#[allow(unused_parens)]
fn draw_ui(mut bengine: Commands) {
    bengine.spawn((
        Node::default(),
        children![(Text::new("B-Engine v0.1.1")), (Text::new("dmenu"), DText)],
    ));
    /*if (PipelinesReady::get() != 0) {
    despawn();
    }*/
}

fn update_dmenu(dtext: Single<&mut Text, With<DText>>) {
    //    if let Some(fps) = diagnostics.get(&FrameTimeDiagnosticsPlugin::FPS)
    //        && let Some(value) = fps.smoothed()
    //    {dtext.0 = }
}

#[derive(Component)]
struct DText;

#[derive(Resource)]
pub struct GameSettings {
    pub atmosphere: bool,
    pub motion_blur: bool,
}
