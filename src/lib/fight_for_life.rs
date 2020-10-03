use amethyst::{GameData, SimpleState, StateData};
use amethyst::core::shred::World;
use amethyst::core::Transform;
use amethyst::prelude::*;
use amethyst::renderer::Camera;

use crate::lib::snow_flake::SNOW_FLAKE_WIDTH;
use crate::lib::SnowFlake;

pub const ARENA_HEIGHT: f32 = 100.0;
pub const ARENA_WIDTH: f32 = 100.0;

pub struct FightForLife;


// SimpleState is equivalent to State<GameData<'static, 'static>, StateEvent>
impl SimpleState for FightForLife {
    fn on_start(&mut self, data: StateData<'_, GameData<'_, '_>>) {
        let world = data.world;

        world.register::<SnowFlake>();
        initialize_snow_flake(world);
        initialize_camera(world);
    }
}

fn initialize_camera(world: &mut World) -> () {
    // Setup camera in a way that our screen covers whole arena and (0, 0) is in the bottom left.
    let mut transform = Transform::default();
    transform.set_translation_xyz(ARENA_WIDTH * 0.5, ARENA_HEIGHT * 0.5, 1.0);

    // create the camera
    world
        .create_entity()
        .with(Camera::standard_2d(ARENA_WIDTH, ARENA_HEIGHT))
        .with(transform)
        .build();
}

pub fn initialize_snow_flake(world: &mut World) -> () {
    let mut transform = Transform::default();

    let y = ARENA_HEIGHT / 2.0;
    transform.set_translation_xyz(SNOW_FLAKE_WIDTH * 0.5, y, 0.0);

    world
        .create_entity()
        .with(SnowFlake::new())
        .with(transform)
        .build();
}