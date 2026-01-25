#![feature(duration_millis_float)]

macro_rules! either {
    ($t:expr => $s:expr ,, $f:expr) => {
        if $t { $s } else { $f }
    };
}

use crate::{
    b_elements::BElements, b_init::BInit, /*b_physics::physics::BPhysics,*/ b_player::BPlayer,
};
use bevy::{app::PluginGroupBuilder, prelude::*};

pub mod b_elements;
pub mod b_init;
//pub mod b_physics;
pub mod b_player;

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
