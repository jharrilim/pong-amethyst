use amethyst::{
    core::transform::Transform,
    ecs::prelude::{Join, System, WriteStorage},
};

use crate::pong::{ARENA_WIDTH};
use crate::components::ball::Ball;

pub struct WinnerSystem;

impl<'s> System<'s> for WinnerSystem {
    type SystemData = (
        WriteStorage<'s, Ball>,
        WriteStorage<'s, Transform>,
    );

    fn run(&mut self, (mut balls, mut locals): Self::SystemData) {
        for (ball, transform) in (&mut balls, &mut locals).join() {
            let ball_x = transform.translation().x;

            let did_hit = if ball_x <= ball.radius {
                // Right player scored on the left side.
                println!("Player 2 Scores!");
                true
            } else if ball_x >= ARENA_WIDTH - ball.radius {
                // Left player scored on the right side.
                println!("Player 1 Scores!");
                true
            } else {
                false
            };

            if did_hit {
                ball.velocity[0] = -ball.velocity[0]; // Reverse Direction
                transform.set_x(ARENA_WIDTH / 2.0); // Reset Position
            }
        }
    }
}