use bevy_app::prelude::*;
use bevy_doryen::doryen::Color;
use bevy_doryen::prelude::*;
use bevy_ecs::prelude::*;

use super::level::{Start, Walls};
use super::light::{Light, LightMap};
use super::shared::{Position, Speed};
use super::{is_running, GameState, RenderSet, UpdateSet};

const PLAYER_SPEED: f32 = 0.2;
const PLAYER_LIGHT_RADIUS: f32 = 8.0;
const PLAYER_LIGHT_COLOR: Color = (150, 150, 150, 255);

#[derive(Component)]
pub struct Player;

pub struct PlayerPlugin;
impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, init)
            .add_systems(OnEnter(GameState::Running), set_start_position)
            .add_systems(
                Update,
                move_player
                    .in_set(UpdateSet::MovePlayer)
                    .run_if(is_running()),
            )
            .add_systems(
                Render,
                render_player.in_set(RenderSet::Player).run_if(is_running()),
            );
    }
}

fn init(mut commands: Commands) {
    commands.spawn((
        Player,
        Position::default(),
        Speed(PLAYER_SPEED),
        Light::new(PLAYER_LIGHT_RADIUS, PLAYER_LIGHT_COLOR, false),
    ));
}

fn set_start_position(start: Res<Start>, mut query: Query<&mut Position, With<Player>>) {
    let player_position = &mut *query.single_mut();
    player_position.x = start.0.x;
    player_position.y = start.0.y;
}

const DEFAULT_COEFFICIENT: f32 = 1.0 / std::f32::consts::SQRT_2;
fn move_player(
    input: Res<Input>,
    mut query: Query<(&mut Position, &Speed), With<Player>>,
    walls: Res<Walls>,
) {
    let mut movement = (0, 0);
    if input.key("ArrowLeft") || input.key("KeyA") {
        movement.0 = -2;
    } else if input.key("ArrowRight") || input.key("KeyD") {
        movement.0 = 2;
    }
    if input.key("ArrowUp") || input.key("KeyW") {
        movement.1 = -2;
    } else if input.key("ArrowDown") || input.key("KeyS") {
        movement.1 = 2;
    }
    if movement == (0, 0) {
        return;
    }

    let mut coef = DEFAULT_COEFFICIENT;
    let (mut player_position, player_speed) = query.single_mut();

    let next_x_pos = player_position.next_pos((movement.0 as f32, 0.));

    if walls.is_wall(next_x_pos) {
        movement.0 = 0;
        coef = 1.0;
    }
    let next_y_pos = player_position.next_pos((0., movement.1 as f32));

    if walls.is_wall(next_y_pos) {
        movement.1 = 0;
        coef = 1.0;
    }
    player_position.move_by(movement, *player_speed, coef);
}

fn render_player(
    mut root_console: ResMut<RootConsole>,
    light_map: NonSend<LightMap>,
    query: Query<&Position, With<Player>>,
) {
    let player_position = query.single();
    let light = light_map
        .0
        .pixel(player_position.x as u32, player_position.y as u32)
        .unwrap();
    root_console.ascii(
        player_position.character_x(),
        player_position.character_y(),
        '@' as u16,
    );
    root_console.fore(
        player_position.character_x(),
        player_position.character_y(),
        light,
    );
}
