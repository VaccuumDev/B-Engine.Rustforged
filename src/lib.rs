use crate::{b_elements::BElements, b_init::BInit};
use bevy::{app::PluginGroupBuilder, prelude::*};

pub mod b_elements;
pub mod b_init;
pub mod b_player {
    pub mod b_player;
}
/*pub mod b_physics {
    pub mod b_physics;
}*/

pub struct BEngine;

impl PluginGroup for BEngine {
    fn build(self) -> PluginGroupBuilder {
        PluginGroupBuilder::start::<Self>()
            .add(BInit)
            .add(BElements)
    }
}
/*
 * Надо сделать подключениие физики иименно в приложениеи в одном из плагинов
 *
 *
 *
 *
 *
 */
