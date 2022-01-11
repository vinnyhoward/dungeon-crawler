pub use crate::prelude::*;

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Render {
    pub color: ColorPair,
    pub glyph: FontCharType,
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Player; // Tag

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Enemy; // Tag

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct MovingRandomly; // Tag

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct WantsToMove {
    pub entity: Entity,
    pub destination: Point,
}

pub struct Health {
    pub current: i32,
    pub max: i32,
}
