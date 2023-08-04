use bevy_app::prelude::*;
use bevy_doryen::doryen::{color_mul, color_scale, Color};
use bevy_doryen::prelude::*;
use bevy_ecs::prelude::*;

use super::super::level::LevelMap;
use super::super::light::{LightMap, LIGHT_COEF};
use super::super::{is_running, RenderSet};
use super::Position;

pub struct CharacterPlugin;
impl Plugin for CharacterPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(
            Render,
            render_character
                .in_set(RenderSet::Entities)
                .run_if(is_running()),
        );
    }
}

#[derive(Component)]
pub struct Character {
    /// console character for this entity
    pub ch: u16,
    pub name: String,
    pub color: Color,
    pub light: bool,
}

fn render_character(
    mut root_console: ResMut<RootConsole>,
    query: Query<(&Character, &Position)>,
    level_map: Res<LevelMap>,
    light_map: NonSend<LightMap>,
) {
    for (character, position) in &query {
        if !level_map
            .0
            .is_in_fov(position.x as usize, position.y as usize)
        {
            continue;
        }
        let (color, penumbra) = if character.light {
            (character.color, false)
        } else {
            let light = light_map
                .0
                .pixel(position.x as u32, position.y as u32)
                .unwrap();

            let penumbra = is_penumbra(light, 100);
            let mut color = color_mul(character.color, light);
            if penumbra {
                color = color_scale(color, LIGHT_COEF);
            }
            (color, penumbra)
        };
        root_console.ascii(
            position.character_x(),
            position.character_y(),
            if penumbra { '?' as u16 } else { character.ch },
        );
        root_console.fore(position.character_x(), position.character_y(), color);
    }
}

fn is_penumbra(color: Color, level: usize) -> bool {
    (color.0 as usize + color.1 as usize + color.2 as usize) < level
}
