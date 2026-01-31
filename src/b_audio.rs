use bevy::prelude::*;
use bevy_kira_audio::prelude::*;

pub struct BAudio;

impl bevy::prelude::Plugin for BAudio {
    fn build(&self, app: &mut bevy::app::App) {
        app.add_plugins(AudioPlugin)
            .add_systems(PostStartup, audio_setup);
    }
}

#[derive(Resource)]
pub struct StepSounds {
    pub sounds: [Handle<bevy_kira_audio::AudioSource>; 4],
}

#[derive(Resource)]
pub struct WhooshInstanceHandle {
    pub o: Handle<AudioInstance>,
}

fn audio_setup(mut bengine: Commands, audio: Res<Audio>, asset_server: Res<AssetServer>) {
    let whoosh = asset_server.load("sounds/player/s_player_whoosh_1.ogg");
    let handle = audio.play(whoosh).looped().handle();
    bengine.insert_resource(WhooshInstanceHandle { o: handle });
    bengine.insert_resource(StepSounds {
        sounds: [
            asset_server.load("sounds/player/s_player_footstep_a1.ogg"),
            asset_server.load("sounds/player/s_player_footstep_a2.ogg"),
            asset_server.load("sounds/player/s_player_footstep_a3.ogg"),
            asset_server.load("sounds/player/s_player_footstep_a4.ogg"),
        ],
    });
}
/*
                asset_server.load("sounds/player/s_player_footstep_a1.ogg"),
                asset_server.load("sounds/player/s_player_footstep_a2.ogg"),
                asset_server.load("sounds/player/s_player_footstep_a3.ogg"),
                asset_server.load("sounds/player/s_player_footstep_a4.ogg"),
*/
