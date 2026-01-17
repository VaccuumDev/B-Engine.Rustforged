use bevy::prelude::*;

pub struct BElements;

impl Plugin for BElements {
    fn build(&self, _app: &mut App) {}
}

#[derive(Component)]
pub struct Element {
    material: MaterialType,
    durability: u8,
    state: u8,
}
impl Element {
    pub fn new(material: MaterialType, durability: u8, state: u8) -> Self {
        Element {
            material: material,
            durability: durability,
            state: state,
        }
    }
    pub fn default() -> Self {
        Element {
            material: MaterialType::Wood,
            durability: 255,
            state: 0,
        }
    }
}

pub enum MaterialType {
    Wood = 0,
    Stone = 1,
    Metal = 2,
    Dirt = 3,
}
