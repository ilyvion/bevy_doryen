#![allow(clippy::type_complexity)]

use bevy_app::prelude::*;
use bevy_doryen::doryen::{AppOptions, Color, TextAlign};
use bevy_doryen::prelude::*;
use bevy_doryen::{FpsInfo, PostRender};
use bevy_ecs::prelude::*;

mod goblin;
mod level;
mod light;
mod noise;
mod player;
mod shared;

// This is more or less a "raw" port of the `demo` example from doryen-rs, but
// when translated to an ECS, it's not really doing things in an ideal way.
// TODO: Port to be more "idiomatic" ECS/Bevy code.

const CONSOLE_WIDTH: u32 = 80;
const CONSOLE_HEIGHT: u32 = 45;
const PLAYER_FOV_RADIUS: usize = 40;
const BLACK: Color = (0, 0, 0, 255);
const WHITE: Color = (255, 255, 255, 255);

/// Groups the stages of the update process under shared labels. This helps us update
/// things in the correct order despite systems being theoretically independent.
#[derive(Clone, Debug, Eq, Hash, PartialEq, SystemSet)]
pub enum UpdateSet {
    MovePlayer,
    ComputeFov,
}

/// Groups the stages of the rendering process under shared labels. This helps us render
/// things in the correct order despite systems being theoretically independent.
#[derive(Clone, Debug, Eq, Hash, PartialEq, SystemSet)]
pub enum RenderSet {
    Clear,
    LightMap,
    Level,
    Entities,
    Player,
}

#[derive(Clone, Debug, Default, Eq, Hash, PartialEq, States)]
pub enum GameState {
    #[default]
    LoadingLevel,
    Running,
}

#[inline(always)]
fn is_running() -> impl FnMut(Option<Res<State<GameState>>>) -> bool + Clone {
    in_state(GameState::Running)
}

#[inline(always)]
fn is_loading_level() -> impl FnMut(Option<Res<State<GameState>>>) -> bool + Clone {
    in_state(GameState::LoadingLevel)
}

pub fn main() {
    App::new()
        .insert_resource(DoryenPluginSettings {
            app_options: AppOptions {
                window_title: String::from("bevy_doryen demo"),
                vsync: false,
                ..Default::default()
            },
            ..Default::default()
        })
        .add_plugins(DoryenPlugin)
        .add_plugins((
            level::LevelPlugin,
            light::LightPlugin,
            player::PlayerPlugin,
            shared::CharacterPlugin,
        ))
        .init_state::<GameState>()
        .add_systems(Startup, init)
        .add_systems(Render, clear.in_set(RenderSet::Clear).run_if(is_running()))
        .add_systems(PostRender, render_instructions.run_if(is_running()))
        .configure_sets(
            Update,
            (UpdateSet::MovePlayer, UpdateSet::ComputeFov).chain(),
        )
        .configure_sets(
            Render,
            (
                RenderSet::Clear,
                RenderSet::LightMap,
                RenderSet::Level,
                RenderSet::Entities,
                RenderSet::Player,
            )
                .chain(),
        )
        .run();
}

fn init(mut root_console: ResMut<RootConsole>) {
    root_console.register_color("white", WHITE);
    root_console.register_color("red", (255, 92, 92, 255));
    root_console.register_color("blue", (192, 192, 255, 255));
}

fn clear(mut root_console: ResMut<RootConsole>) {
    root_console.clear(Some(BLACK), Some(BLACK), Some(' ' as u16));
}

fn render_instructions(mut root_console: ResMut<RootConsole>, fps: Res<FpsInfo>) {
    root_console.print_color(
        (CONSOLE_WIDTH / 2) as i32,
        (CONSOLE_HEIGHT - 2) as i32,
        &format!(
            "#[white]Move with #[red]arrows or WSAD #[white]Fire with #[red]mouse   {:4} fps",
            fps.fps
        ),
        TextAlign::Center,
        None,
    );
}
