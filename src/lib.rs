use crate::{b_elements::BElements, b_init::BInit, b_player::BPlayer};
use bevy::{app::PluginGroupBuilder, prelude::*};

pub mod b_elements;
pub mod b_init;
pub mod b_player;
pub mod b_ui;

pub struct BEngine;

impl PluginGroup for BEngine {
    fn build(self) -> PluginGroupBuilder {
        PluginGroupBuilder::start::<Self>()
            .add(BInit)
            .add(BElements)
            .add(BPlayer)
        //.add(BPhysics)
    }
}
