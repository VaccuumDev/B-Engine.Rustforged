use b_engine::BEngine;
use bevy::prelude::*;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_plugin_setup() {
        info!("Start.");
        App::new()
            .add_plugins(BEngine)
            .add_plugins(DefaultPlugins)
            .run();
    }
}
