use bevy::{ecs::name::Name, prelude::Bundle};
use bevy_lunex::prelude::*;

#[derive(Bundle)]
pub struct BUiRoot {
    name: Name,
    root: UiLayoutRoot,
    fetch: UiFetchFromCamera<0>,
}
impl Default for BUiRoot {
    fn default() -> Self {
        BUiRoot {
            name: Name::new("UiRoot"),
            root: UiLayoutRoot::new_2d(),
            fetch: UiFetchFromCamera::<0>,
        }
    }
}

#[derive(Bundle)]
pub struct BUiBlock {
    layout: UiLayout,
}
impl BUiBlock {
    pub fn new(anchor: Anchor, pos: (f32, f32), size: (f32, f32)) -> Self {
        BUiBlock {
            layout: UiLayout::window().anchor(anchor).pos(pos).size(size).pack(),
        }
    }
}
