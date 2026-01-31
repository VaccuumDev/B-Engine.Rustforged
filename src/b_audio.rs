use bevy::{audio::*, prelude::*};

pub struct BAudio;

impl bevy::prelude::Plugin for BAudio {
    fn build(&self, app: &mut bevy::app::App) {
        app.add_systems(PostStartup, audio_setup);
    }
}

#[derive(Component)]
pub struct WhooshSource;

#[derive(Component)]
pub struct StepSounds {
    pub sounds: [Handle<AudioSource>; 4],
}

fn audio_setup(mut bengine: Commands, asset_server: Res<AssetServer>) {
    let whoosh = asset_server.load("sounds/player/s_player_whoosh_1.ogg");
    bengine.spawn((
        AudioPlayer::new(whoosh),
        PlaybackSettings::LOOP,
        WhooshSource,
    ));
    bengine.spawn((
        StepSounds {
            sounds: [
                asset_server.load("sounds/player/s_player_footstep_a1.ogg"),
                asset_server.load("sounds/player/s_player_footstep_a2.ogg"),
                asset_server.load("sounds/player/s_player_footstep_a3.ogg"),
                asset_server.load("sounds/player/s_player_footstep_a4.ogg"),
            ],
        },
        AudioPlayer::new(asset_server.load("sounds/player/s_player_footstep_a1.ogg")),
    ));
}
