extern crate amethyst;

mod systems;
mod components;
mod entities;
mod states;

use amethyst::prelude::*;
use amethyst::config::ConfigError;
use amethyst::core::transform::TransformBundle;
use amethyst::input::InputBundle;
use amethyst::renderer::{
    DisplayConfig, DrawFlat2D, Pipeline,
    RenderBundle, Stage,
};
use amethyst::ui::{DrawUi, UiBundle};

use systems::{
    PaddleSystem,
    BallMovementSystem,
    BounceSystem,
};

use states::pong::Pong;

use amethyst::utils::application_root_dir;


// Return result so we can use ? operator
fn main() -> amethyst::Result<()> {
    // Show logs during runtime
    amethyst::start_logger(Default::default());
    let render_bundle = {
        let path = format!("{}/assets/config/display_config.ron", application_root_dir());
        let pipe = Pipeline::build()
            .with_stage(
                Stage::with_backbuffer()
                    .clear_target([0.0, 0.0, 0.0, 1.0], 1.0)
                    .with_pass(DrawFlat2D::new())
                    .with_pass(DrawUi::new()),
            );
        // Load display_config.ron
        let config = DisplayConfig::load(&path);
        RenderBundle::new(pipe, Some(config))
            .with_sprite_sheet_processor()
    };

    let game_data = GameDataBuilder::default()
        .with_bundle(render_bundle)?
        .with_bundle(TransformBundle::new())?
        .with_bundle(input_bundle()?)?
        .with_bundle(UiBundle::<String, String>::new())?
        .with(PaddleSystem, "paddle_system", &["input_system"])
        .with(BallMovementSystem, "ball_system", &[])
        .with(BounceSystem, "bounce_system", &["paddle_system", "ball_system"])
        .with(systems::WinnerSystem, "winner_system", &["ball_system"]);

    let mut game = Application::new("./", Pong, game_data)?;

    game.run();
    Ok(())
}

fn input_bundle() -> Result<InputBundle<String, String>, ConfigError> {
    InputBundle::<String, String>::new()
        .with_bindings_from_file(
            format!(
                "{}/assets/config/bindings_config.ron",
                application_root_dir()
            )
        )
}