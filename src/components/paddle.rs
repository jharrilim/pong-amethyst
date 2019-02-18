use amethyst::ecs::{
    Component,
    DenseVecStorage
};

#[derive(PartialEq, Eq)]
pub enum Side {
    Left,
    Right,
}

pub struct Paddle {
    pub side: Side,
    pub width: f32,
    pub height: f32,
}

impl Paddle {
    pub fn new(side: Side) -> Paddle {
        Paddle {
            side,
            width: 1.0,
            height: 1.0,
        }
    }
}

impl Component for Paddle {
    // https://slide-rs.github.io/specs/05_storages.html#densevecstorage
    type Storage = DenseVecStorage<Self>;
}