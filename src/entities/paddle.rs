use amethyst::prelude::*;
use amethyst::core::transform::Transform;
use amethyst::renderer::{
    SpriteSheetHandle,
    SpriteRender,
    Flipped
};

use crate::states::pong::{
    ARENA_WIDTH, ARENA_HEIGHT
};

use crate::components::paddle::*;

pub const PADDLE_HEIGHT: f32 = 16.0;
pub const PADDLE_WIDTH: f32 = 4.0;

/// Initialises one paddle on the left, and one paddle on the right.
pub fn initialise_paddles(world: &mut World, sprite_sheet: SpriteSheetHandle) {
    let mut left_transform = Transform::default();
    let mut right_transform = Transform::default();

    // Only need one since 2nd paddle is just flipped
    let sprite_render = SpriteRender {
        sprite_sheet: sprite_sheet.clone(),
        sprite_number: 0, // paddle is the first sprite in the sprite_sheet
    };

    // Correctly position the paddles.
    let y = ARENA_HEIGHT / 2.0;
    left_transform.set_xyz(PADDLE_WIDTH * 0.5, y, 0.0);
    right_transform.set_xyz(ARENA_WIDTH - PADDLE_WIDTH * 0.5, y, 0.0);

    // Create a left plank entity.
    world
        .create_entity()
        .with(Paddle::new(Side::Left))
        .with(left_transform)
        .with(sprite_render.clone())
        .build();

    // Create right plank entity.
    world
        .create_entity()
        .with(Paddle::new(Side::Right))
        .with(right_transform)
        .with(sprite_render.clone())
        .with(Flipped::Horizontal)
        .build();
}