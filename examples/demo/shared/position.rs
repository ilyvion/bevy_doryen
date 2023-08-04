use bevy_ecs::prelude::*;

use super::Speed;

#[derive(Copy, Component, Clone, Debug, Default, PartialEq)]
pub struct Position {
    pub x: f32,
    pub y: f32,
}
impl Position {
    pub fn next_pos(self, (x, y): (f32, f32)) -> Self {
        Self {
            x: self.x + x,
            y: self.y + y,
        }
    }

    pub fn move_by(&mut self, mov: (i32, i32), speed: Speed, coef: f32) -> bool {
        let old_x = self.x as i32;
        let old_y = self.y as i32;
        self.x += speed.0 * mov.0 as f32 * coef;
        self.y += speed.0 * mov.1 as f32 * coef;

        old_x == self.x as i32 && old_y == self.y as i32
    }

    #[inline]
    pub fn character_x(&self) -> i32 {
        self.x as i32 / 2
    }
    #[inline]
    pub fn character_y(&self) -> i32 {
        self.y as i32 / 2
    }
}

impl From<(f32, f32)> for Position {
    fn from((x, y): (f32, f32)) -> Self {
        Self { x, y }
    }
}
impl From<Position> for (f32, f32) {
    fn from(Position { x, y }: Position) -> Self {
        (x, y)
    }
}
