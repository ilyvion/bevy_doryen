use bevy_app::prelude::*;
use bevy_doryen::doryen::{color_add, color_blend, color_scale, Color, Image};
use bevy_doryen::prelude::*;
use bevy_ecs::prelude::*;
use doryen_fov::{FovAlgorithm, FovRestrictive, MapData};
use rand::{thread_rng, Rng};

use super::level::{LevelMap, LevelSize};
use super::noise::simplex;
use super::shared::{Character, Position};
use super::{is_running, RenderSet, BLACK};

pub const LIGHT_COEF: f32 = 1.5;

const LIGHT_COLOR: Color = (150, 174, 27, 255);

const TIME_SCALE: f32 = 0.05;
const LIGHT_INTENSITY: f32 = 1.5;
const LIGHT_FLICKER_MOVE: f32 = 2.0;
const LIGHT_FLICKER_INTENSITY: f32 = 0.4;
const LIGHT_FLICKER_RADIUS: f32 = 0.2;

pub struct LightPlugin;
impl Plugin for LightPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, init)
            .add_systems(Update, update_time.run_if(is_running()))
            .add_systems(
                Render,
                compute_lightmap
                    .in_set(RenderSet::LightMap)
                    .run_if(is_running()),
            );
    }
}

#[derive(Component)]
pub struct Light {
    pub radius: f32,
    pub intensity: f32,
    pub color: Color,
    pub t: f32,
    pub flicker: bool,
}

#[derive(Bundle)]
pub struct LightBundle {
    pub light: Light,
    pub character: Character,
    pub position: Position,
}
impl LightBundle {
    pub fn new(radius: f32, color: Color, flicker: bool) -> Self {
        Self {
            light: Light::new(radius, color, flicker),
            character: Character {
                ch: 15,
                name: "a flickering torch".to_owned(),
                color: LIGHT_COLOR,
                light: true,
            },
            position: Position::default(),
        }
    }
}

pub struct LightMap(pub Image);
impl Default for LightMap {
    fn default() -> Self {
        Self(Image::new_empty(0, 0))
    }
}

impl Light {
    pub fn new(radius: f32, color: Color, flicker: bool) -> Self {
        Self {
            radius,
            color,
            intensity: LIGHT_INTENSITY,
            // random t initial value so that all lights don't flicker in sync
            t: thread_rng().gen::<f32>() * 100. * 150.,
            flicker,
        }
    }
}

fn init(world: &mut World) {
    world.init_non_send_resource::<LightMap>();
}

fn update_time(mut query: Query<&mut Light>) {
    for mut light in &mut query {
        light.t += TIME_SCALE;
    }
}

fn compute_lightmap(
    mut light_map: NonSendMut<LightMap>,
    size: Res<LevelSize>,
    query: Query<(&Light, &Position)>,
    level_map: Res<LevelMap>,
) {
    let light_map = &mut light_map.0;
    *light_map = Image::new_empty(size.width as u32 * 2, size.height as u32 * 2);
    let mut fov = FovRestrictive::new();
    for (light, light_position) in &query {
        let (px, py, intensity, radius) = if light.flicker {
            // alter light position, radius and intensity over time
            (
                light_position.x + (LIGHT_FLICKER_MOVE * (simplex(light.t) - 0.5)),
                light_position.y + (LIGHT_FLICKER_MOVE * (simplex(light.t + 2.0) - 0.5)),
                light.intensity + LIGHT_FLICKER_INTENSITY * (simplex(light.t + 4.0) - 0.5),
                light.radius * (1.0 + LIGHT_FLICKER_RADIUS * (simplex(light.t + 6.0) - 0.5)),
            )
        } else {
            (
                light_position.x,
                light_position.y,
                light.intensity,
                light.radius,
            )
        };

        let minx = ((px - radius).floor() as i32).max(0) as u32;
        let maxx = ((px + radius).ceil() as i32).min(light_map.width() as i32 - 1) as u32;
        let miny = ((py - radius).floor() as i32).max(0) as u32;
        let maxy = ((py + radius).ceil() as i32).min(light_map.height() as i32 - 1) as u32;
        let width = maxx - minx + 1;
        let height = maxy - miny + 1;
        let mut map = MapData::new(width as usize, height as usize);
        for y in miny..=maxy {
            for x in minx..=maxx {
                map.set_transparent(
                    (x - minx) as usize,
                    (y - miny) as usize,
                    level_map.0.is_transparent(x as usize, y as usize),
                );
            }
        }
        fov.compute_fov(
            &mut map,
            px as usize - minx as usize,
            py as usize - miny as usize,
            radius as usize,
            true,
        );
        let light_color = color_scale(light.color, intensity);
        let radius_squared = radius * radius;
        let radius_coef = 1.0 / (1.0 + radius_squared / 20.0);
        for y in miny..=maxy {
            for x in minx..=maxx {
                if map.is_in_fov((x - minx) as usize, (y - miny) as usize) {
                    let dx = x as f32 - px;
                    let dy = y as f32 - py;
                    // good looking lights.
                    let squared_dist = dx * dx + dy * dy;
                    let intensity_coef = 1.0 / (1.0 + squared_dist / 20.0);
                    let intensity_coef = intensity_coef - radius_coef;
                    let intensity_coef = intensity_coef / (1.0 - radius_coef);
                    if intensity_coef > 0.0 {
                        let light = color_blend(BLACK, light_color, intensity_coef);
                        let cur_light = light_map.pixel(x, y).unwrap();
                        light_map.put_pixel(x, y, color_add(light, cur_light));
                    }
                }
            }
        }
    }
}
