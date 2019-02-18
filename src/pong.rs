use amethyst::assets::{AssetStorage, Loader};
use amethyst::input::is_key_down;
use amethyst::prelude::*;
use amethyst::renderer::{
    PngFormat, SpriteSheet, SpriteSheetFormat,
    SpriteSheetHandle, Texture, TextureMetadata,
    VirtualKeyCode
};

use crate::entities::{
    paddle::initialise_paddles,
    camera::initialise_camera,
    ball::initialise_ball,
    scoreboard::initialise_scoreboard
};

use crate::states::paused::PausedState;

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

        initialise_ball(world, spritesheet_handle.clone());
        initialise_paddles(world, spritesheet_handle.clone());
        initialise_camera(world);
        initialise_scoreboard(world);
    }
}

impl EmptyState for Pong {
    fn handle_event(&mut self, _data: StateData<()>, event: StateEvent) -> EmptyTrans {
        if let StateEvent::Window(event) = &event {
            if is_key_down(&event, VirtualKeyCode::Space) {
                return Trans::Push(Box::new(PausedState));
            }
        }
        Trans::None
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
            "assets/textures/pong_spritesheet.png",
            PngFormat,
            TextureMetadata::srgb_scale(),
            (),
            &texture_storage,
        )
    };

    let sprite_sheet_store = world.read_resource::<AssetStorage<SpriteSheet>>();
    loader.load(
        "assets/textures/pong_spritesheet.ron", // Here we load the associated ron file
        SpriteSheetFormat,
        texture_handle, // We pass it the handle of the texture we want it to use
        (),
        &sprite_sheet_store,
    )

}