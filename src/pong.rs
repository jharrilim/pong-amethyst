use amethyst::assets::{AssetStorage, Loader};
use amethyst::prelude::*;
use amethyst::renderer::{
    PngFormat, SpriteSheet, SpriteSheetFormat,
    SpriteSheetHandle, Texture, TextureMetadata,
};

use crate::entities::{
    paddle::{initialise_paddles},
    camera::{initialise_camera}
};

pub const ARENA_HEIGHT: f32 = 100.0;
pub const ARENA_WIDTH: f32 = 100.0;


pub struct Pong;

/**
 * Used by Amethyst's state machine for starting, stopping, and updating the game.
 *
 */
impl SimpleState for Pong {
    fn on_start(&mut self, data: StateData<'_, GameData<'_, '_>>) {
        let world = data.world;
        let spritesheet_handle = load_sprite_sheet(world);

        // For a Component to be used, there must be a Storage<ComponentType>
        // resource set up in the World.
        // world.register::<Paddle>(); // This can be handled implicitly when initializing
        initialise_paddles(world, spritesheet_handle);
        initialise_camera(world);
    }
}



fn load_sprite_sheet(world: &mut World) -> SpriteSheetHandle {
    // Load the sprite sheet necessary to render the graphics.
    // The texture is the pixel data
    // `texture_handle` is a cloneable reference to the texture
    let loader = world.read_resource::<Loader>();
    let texture_handle = {
        let texture_storage = world.read_resource::<AssetStorage<Texture>>();
        // Asset loader; can load .obj, .png, etc
        loader.load(
            "texture/pong_spritesheet.png",
            PngFormat,
            TextureMetadata::srgb_scale(),
            (),
            &texture_storage,
        )
    };

    let sprite_sheet_store = world.read_resource::<AssetStorage<SpriteSheet>>();
    loader.load(
        "texture/pong_spritesheet.ron", // Here we load the associated ron file
        SpriteSheetFormat,
        texture_handle, // We pass it the handle of the texture we want it to use
        (),
        &sprite_sheet_store,
    )

}