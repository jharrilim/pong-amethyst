use amethyst::{
    assets::Loader,
    prelude::*,
    ui::{Anchor, TtfFormat, UiText, UiTransform},
};

use crate::components::scoreboard::*;

pub fn initialise_scoreboard(world: &mut World) {
    let font = world.read_resource::<Loader>().load(
        "assets/font/square.ttf",
        TtfFormat,
        Default::default(),
        (),
        &world.read_resource(),
    );
    let p1_transform = UiTransform::new(
        "P1".to_string(), Anchor::TopMiddle,
        -50., -50., 1., 200., 50., 0,
    );
    let p2_transform = UiTransform::new(
        "P2".to_string(), Anchor::TopMiddle,
        50., -50., 1., 200., 50., 0,
    );

    let p1_score = world
        .create_entity()
        .with(p1_transform)
        .with(UiText::new(
            font.clone(),
            "0".to_string(),
            [1., 1., 1., 1.],
            50.,
        )).build();

    let p2_score = world
        .create_entity()
        .with(p2_transform)
        .with(UiText::new(
            font.clone(),
            "0".to_string(),
            [1., 1., 1., 1.],
            50.,
        )).build();

    world.add_resource(ScoreText { p1_score, p2_score });
}