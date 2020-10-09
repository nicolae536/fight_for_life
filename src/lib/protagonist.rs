use amethyst::core::ecs::{Component, DenseVecStorage};

pub const PROTAGONIST_WIDTH: f32 = 65.0;
pub const PROTAGONIST_HEIGHT: f32 = 65.0;

pub struct Protagonist {
    width: f32,
    height: f32,
}

impl Protagonist {
    pub fn new() -> Protagonist {
        Protagonist {
            height: PROTAGONIST_HEIGHT,
            width: PROTAGONIST_WIDTH,
        }
    }
}

impl Component for Protagonist {
    type Storage = DenseVecStorage<Self>;
}