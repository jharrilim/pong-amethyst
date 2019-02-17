extern crate amethyst;

use amethyst::prelude::*;
use amethyst::core::transform::TransformBundle;
use amethyst::renderer::{
    DisplayConfig, DrawFlat2D, Pipeline,
    RenderBundle, Stage
};

mod pong;

use crate::pong::Pong;

use amethyst::utils::application_root_dir;

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
        )?
        .with_bundle(TransformBundle::new())?;

    let mut game = Application::new("./", Pong, game_data)?;

    game.run();

    Ok(())
}