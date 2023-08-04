use bevy_doryen::doryen::Color;
use bevy_ecs::prelude::*;

use super::shared::{Character, Position};

const GOBLIN_COLOR: Color = (80, 150, 70, 255);

#[derive(Component, Default)]
pub struct Goblin;

#[derive(Bundle)]
pub struct GoblinBundle {
    pub goblin: Goblin,
    pub character: Character,
    pub position: Position,
}

impl Default for GoblinBundle {
    fn default() -> Self {
        Self {
            goblin: Default::default(),
            character: Character {
                ch: 'g' as u16,
                name: "a petrified goblin".to_owned(),
                color: GOBLIN_COLOR,
                light: false,
            },
            position: Default::default(),
        }
    }
}
