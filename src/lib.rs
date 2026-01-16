use crate::{b_elements::BElements, b_init::BInit};
use bevy::{app::PluginGroupBuilder, prelude::*};

mod b_elements;
mod b_init;

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
