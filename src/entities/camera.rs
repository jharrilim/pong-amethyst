use amethyst::prelude::*;
use amethyst::core::transform::Transform;
use amethyst::renderer::{
    Camera,
    Projection,
};

use crate::states::pong::{ARENA_WIDTH, ARENA_HEIGHT};

pub fn initialise_camera(world: &mut World) {
    let mut transform = Transform::default();
    transform.set_z(1.0);
    world
        .create_entity()
        .with(Camera::from(Projection::orthographic(
            0.0,
            ARENA_WIDTH,
            0.0,
            ARENA_HEIGHT,
        )))
        .with(transform)
        .build();
}