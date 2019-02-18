use amethyst::prelude::*;
use amethyst::core::transform::Transform;
use amethyst::renderer::{
    SpriteSheetHandle,
    SpriteRender
};

use crate::pong::{
    ARENA_WIDTH, ARENA_HEIGHT,
};

use crate::components::ball::*;

pub fn initialise_ball(world: &mut World, sprite_sheet: SpriteSheetHandle) {
    // Create the translation.
    let mut local_transform = Transform::default();
    local_transform.set_xyz(ARENA_WIDTH / 2.0, ARENA_HEIGHT / 2.0, 0.0);

    // Assign the sprite for the ball
    let sprite_render = SpriteRender {
        sprite_sheet,
        sprite_number: 1, // ball is the second sprite on the sprite sheet
    };

    world
        .create_entity()
        .with(sprite_render)
        .with(Ball {
            radius: BALL_RADIUS,
            velocity: [BALL_VELOCITY_X, BALL_VELOCITY_Y],
        })
        .with(local_transform)
        .build();
}