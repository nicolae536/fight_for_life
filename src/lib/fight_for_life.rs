use amethyst::{GameData, SimpleState, StateData};
use amethyst::assets::{AssetStorage, Handle, Loader};
use amethyst::core::shred::World;
use amethyst::core::Transform;
use amethyst::prelude::*;
use amethyst::renderer::{Camera, ImageFormat, SpriteSheet, SpriteSheetFormat, Texture, SpriteRender};
use amethyst::core::math::Vector3;

use crate::lib::snow_flake::{SNOW_FLAKE_WIDTH, SNOW_FLAKE_HEIGHT};
use crate::lib::{Protagonist};
use crate::lib::protagonist::{PROTAGONIST_WIDTH, PROTAGONIST_HEIGHT};

pub const ARENA_HEIGHT: f32 = 100.0;
pub const ARENA_WIDTH: f32 = 100.0;

pub struct FightForLife;


// SimpleState is equivalent to State<GameData<'static, 'static>, StateEvent>
impl SimpleState for FightForLife {
    fn on_start(&mut self, data: StateData<'_, GameData<'_, '_>>) {
        let world = data.world;
        let protagonist_sheet_handle = load_sprite_sheet(world);

        world.register::<Protagonist>();

        initialize_protagonist(world, protagonist_sheet_handle);
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

fn initialize_protagonist(world: &mut World, sprite_sheet_handle: Handle<SpriteSheet>) -> () {
    let mut transform = Transform::default();

    transform.set_translation_xyz(ARENA_WIDTH / 2.0, ARENA_HEIGHT / 2.0, 0.0);
    let sprite_reader = SpriteRender::new(sprite_sheet_handle, 0);

    world
        .create_entity()
        .with(sprite_reader.clone())
        .with(Protagonist::new())
        .with(transform)
        .build();
}

fn load_sprite_sheet(world: &mut World) -> Handle<SpriteSheet> {
    // Load the sprite sheet necessary to render the graphics.
    // The texture is the pixel data
    // `texture_handle` is a cloneable reference to the texture
    let texture_handle = {
        let loader = world.read_resource::<Loader>();
        let texture_storage = world.read_resource::<AssetStorage<Texture>>();
        loader.load(
            "sprites/protagonist.png",
            ImageFormat::default(),
            (),
            &texture_storage,
        )
    };

    let loader = world.read_resource::<Loader>();
    let sprite_sheet_store = world.read_resource::<AssetStorage<SpriteSheet>>();
    return loader.load(
        "sprites/protagonist.ron",
        SpriteSheetFormat(texture_handle),
        (),
        &sprite_sheet_store,
    );
}