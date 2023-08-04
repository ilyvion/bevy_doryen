use bevy_app::prelude::*;
use bevy_doryen::doryen::{color_blend, Color, Image, TextAlign};
use bevy_doryen::prelude::*;
use bevy_ecs::prelude::*;
use doryen_fov::{FovAlgorithm, FovRestrictive, MapData};

use super::goblin::GoblinBundle;
use super::light::{LightBundle, LightMap, LIGHT_COEF};
use super::player::Player;
use super::shared::Position;
use super::{is_loading_level, is_running, GameState, RenderSet, UpdateSet, PLAYER_FOV_RADIUS};

const START_COLOR: Color = (255, 0, 0, 255);
const LIGHT_COLOR: Color = (255, 255, 0, 255);
const LIGHT_RADIUS: f32 = 15.0;
const WALL_COLOR: Color = (255, 255, 255, 255);
const GOBLIN_COLOR: Color = (0, 255, 0, 255);
const VISITED_BLEND_COLOR: Color = (10, 10, 40, 255);
const VISITED_BLEND_COEF: f32 = 0.8;

pub struct LevelPlugin;
impl Plugin for LevelPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<Visited2x>()
            .init_resource::<Walls>()
            .init_resource::<PlayerFov>()
            .init_resource::<LevelSize>()
            .add_systems(Startup, init)
            .add_systems(Update, load_level.run_if(is_loading_level()))
            .add_systems(OnExit(GameState::LoadingLevel), unload_level_image)
            .add_systems(OnEnter(GameState::Running), initial_compute_fov)
            .add_systems(
                Update,
                compute_fov
                    .in_set(UpdateSet::ComputeFov)
                    .run_if(is_running()),
            )
            .add_systems(Render, render_loading_level.run_if(is_loading_level()))
            .add_systems(
                Render,
                render_level.in_set(RenderSet::Level).run_if(is_running()),
            );
    }
}

struct LevelImage(Image);
struct GroundImage(Image);

#[derive(Default, Resource)]
struct Visited2x(Vec<bool>);

#[derive(Default, Resource)]
pub struct Walls {
    data: Vec<bool>,
    level_width: usize,
}
impl Walls {
    pub fn is_wall(&self, Position { x, y }: Position) -> bool {
        self.data[(x / 2.) as usize + (y / 2.) as usize * self.level_width]
    }
}

#[derive(Default, Resource)]
pub struct Start(pub Position);

#[derive(Resource)]
pub struct LevelMap(pub MapData);

#[derive(Default, Resource)]
pub struct PlayerFov(FovRestrictive);

#[derive(Debug, Default, Resource)]
pub struct LevelSize {
    pub width: usize,
    pub height: usize,
}

fn init(world: &mut World) {
    world.insert_non_send_resource(LevelImage(Image::new("demo/level.png")));
    world.insert_non_send_resource(GroundImage(Image::new("demo/level_color.png")));
}

fn load_level(
    mut level_image: NonSendMut<LevelImage>,
    mut ground_image: NonSendMut<GroundImage>,
    mut next_game_state: ResMut<NextState<GameState>>,
    mut visited_2x: ResMut<Visited2x>,
    mut walls: ResMut<Walls>,
    mut commands: Commands,
    mut size: ResMut<LevelSize>,
) {
    let level_image = &mut level_image.0;
    let level_image_try_load = level_image.try_load();
    if level_image_try_load {
        let image_size = level_image.try_get_size().unwrap();
        size.width = image_size.0 as usize / 2;
        size.height = image_size.1 as usize / 2;
        walls.data = vec![false; size.width * size.height];
        walls.level_width = size.width;
        let mut map = MapData::new(image_size.0 as usize, image_size.1 as usize);
        visited_2x.0.reserve((image_size.0 * image_size.1) as usize);
        for y in 0..image_size.1 {
            for x in 0..image_size.0 {
                let p = level_image.pixel(x, y).unwrap();
                map.set_transparent(x as usize, y as usize, p != WALL_COLOR);
                visited_2x.0.push(false);
                let pos_1x = (x as f32 / 2., y as f32 / 2.);
                match p {
                    START_COLOR => commands.insert_resource(Start((x as f32, y as f32).into())),
                    LIGHT_COLOR => {
                        commands.spawn(LightBundle {
                            position: Position {
                                x: x as f32,
                                y: y as f32,
                            },
                            ..LightBundle::new(LIGHT_RADIUS, LIGHT_COLOR, true)
                        });
                    }
                    GOBLIN_COLOR => {
                        let offset =
                            (pos_1x.0 as i32 + pos_1x.1 as i32 * size.width as i32) as usize;
                        walls.data[offset] = true;
                        commands.spawn(GoblinBundle {
                            position: Position {
                                x: x as f32,
                                y: y as f32,
                            },
                            ..Default::default()
                        });
                    }
                    _ => (),
                }
            }
        }

        for y in 0..size.height {
            for x in 0..size.width {
                let mut count = 0;
                let x2 = x * 2;
                let y2 = y * 2;
                if map.is_transparent(x2, y2) {
                    count += 1;
                }
                if map.is_transparent(x2 + 1, y2) {
                    count += 1;
                }
                if map.is_transparent(x2, y2 + 1) {
                    count += 1;
                }
                if map.is_transparent(x2 + 1, y2 + 1) {
                    count += 1;
                }
                if count < 2 {
                    let offset = x + y * size.width;
                    walls.data[offset] = true;
                }
            }
        }

        commands.insert_resource(LevelMap(map));
    }

    if level_image_try_load && ground_image.0.try_load() {
        next_game_state.set(GameState::Running);
    }
}

fn unload_level_image(world: &mut World) {
    world.remove_non_send_resource::<LevelImage>();
}

fn initial_compute_fov(
    player_query: Query<&Position, With<Player>>,
    mut map: ResMut<LevelMap>,
    mut fov: ResMut<PlayerFov>,
) {
    let player_position = player_query.single();
    map.0.clear_fov();
    fov.0.compute_fov(
        &mut map.0,
        player_position.x as usize,
        player_position.y as usize,
        PLAYER_FOV_RADIUS,
        true,
    );
}

fn compute_fov(
    player_query: Query<&Position, (With<Player>, Changed<Position>)>,
    mut map: ResMut<LevelMap>,
    mut fov: ResMut<PlayerFov>,
) {
    if let Ok(player_position) = player_query.get_single() {
        map.0.clear_fov();
        fov.0.compute_fov(
            &mut map.0,
            player_position.x as usize,
            player_position.y as usize,
            PLAYER_FOV_RADIUS,
            true,
        );
    }
}

fn render_loading_level(mut root_console: ResMut<RootConsole>) {
    let x = (root_console.get_width() / 2) as i32;
    let y = (root_console.get_height() / 2) as i32;

    root_console.print_color(x, y, "#[white]Loading#[red]...", TextAlign::Center, None);
}

fn render_level(
    mut root_console: ResMut<RootConsole>,
    ground: NonSend<GroundImage>,
    size: Res<LevelSize>,
    map: Res<LevelMap>,
    mut visited_2x: ResMut<Visited2x>,
    light_map: NonSend<LightMap>,
) {
    let mut render_output = Image::new_empty(size.width as u32 * 2, size.height as u32 * 2);
    let light_map = &light_map.0;
    for y in 0..size.height * 2 {
        for x in 0..size.width * 2 {
            let off = x + y * size.width * 2;
            if map.0.is_in_fov(x, y) && (map.0.is_transparent(x, y) || !visited_2x.0[off]) {
                visited_2x.0[off] = true;
                let ground_col = ground.0.pixel(x as u32, y as u32).unwrap();
                let light_col = light_map.pixel(x as u32, y as u32).unwrap();
                let mut r = f32::from(ground_col.0) * f32::from(light_col.0) * LIGHT_COEF / 255.0;
                let mut g = f32::from(ground_col.1) * f32::from(light_col.1) * LIGHT_COEF / 255.0;
                let mut b = f32::from(ground_col.2) * f32::from(light_col.2) * LIGHT_COEF / 255.0;
                r = r.min(255.0);
                g = g.min(255.0);
                b = b.min(255.0);
                render_output.put_pixel(x as u32, y as u32, (r as u8, g as u8, b as u8, 255));
            } else if visited_2x.0[off] {
                let col = ground.0.pixel(x as u32, y as u32).unwrap();
                let dark_col = color_blend(col, VISITED_BLEND_COLOR, VISITED_BLEND_COEF);
                render_output.put_pixel(x as u32, y as u32, dark_col);
            } else {
                render_output.put_pixel(x as u32, y as u32, (0, 0, 0, 255));
            }
        }
    }
    render_output.blit_2x(&mut root_console, 0, 0, 0, 0, None, None, None);
}
