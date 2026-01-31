use bevy::{audio::*, prelude::*};

pub struct BAudio;

impl bevy::prelude::Plugin for BAudio {
    fn build(&self, app: &mut bevy::app::App) {
        app.add_systems(PostStartup, audio_setup);
    }
}

fn audio_setup(mut bengine: Commands, asset_server: Res<AssetServer>) {
    let whoosh = asset_server.load("sounds/player/s_player_whoosh_1.wav");
    if asset_server
        .get_load_state(whoosh.id())
        .unwrap()
        .is_loaded()
    {
        info!("Spawned audio");
        bengine.spawn((AudioPlayer::new(whoosh), PlaybackSettings::LOOP));
    }
}
