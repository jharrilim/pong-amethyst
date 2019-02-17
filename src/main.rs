extern crate amethyst;

use amethyst::prelude::*;
use amethyst::renderer::{
    DisplayConfig, DrawFlat2D, Event, Pipeline,
    RenderBundle, Stage, VirtualKeyCode
};

use amethyst::utils::application_root_dir;

pub struct Pong;

/**
 * Used by Amethyst's state machine for starting, stopping, and updating the game.
 *
 */
impl SimpleState for Pong {

}


// Return result so we can use ? operator
fn main() -> amethyst::Result<()> {
    let path = format!("{}/resources/display_config.ron", application_root_dir());

    // Load display_config.ron
    let config = DisplayConfig::load(&path);
    // Show logs during runtime
    amethyst::start_logger(Default::default());

    // Renders a background
    let pipe = Pipeline::build()
        .with_stage(
            Stage::with_backbuffer()
                .clear_target([0.0, 0.0, 0.0, 1.0], 1.0)
                .with_pass(DrawFlat2D::new()),
        );

    let game_data = GameDataBuilder::default()
        .with_bundle(
            RenderBundle::new(pipe, Some(config))
                .with_sprite_sheet_processor()
        )?;

    let mut game = Application::new("./", Pong, game_data)?;

    game.run();

    Ok(())
}