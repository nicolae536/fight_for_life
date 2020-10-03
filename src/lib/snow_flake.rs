use amethyst::core::ecs::{Component, DenseVecStorage};

pub const SNOW_FLAKE_WIDTH: f32 = 16.0;
pub const SNOW_FLAKE_HEIGHT: f32 = 16.0;

pub struct SnowFlake {
    width: f32,
    height: f32,
}

impl SnowFlake {
    pub fn new() -> SnowFlake {
        SnowFlake {
            height: SNOW_FLAKE_HEIGHT,
            width: SNOW_FLAKE_WIDTH,
        }
    }
}

impl Component for SnowFlake {
    type Storage = DenseVecStorage<Self>;
}