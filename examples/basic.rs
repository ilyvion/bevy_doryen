use bevy_app::{App, Startup, Update};
use bevy_doryen::doryen::{AppOptions, TextAlign};
use bevy_doryen::{Capture, DoryenPlugin, DoryenPluginSettings, Input, Render, RootConsole};
use bevy_ecs::bundle::Bundle;
use bevy_ecs::entity::Entity;
use bevy_ecs::prelude::{Component, EventWriter};
use bevy_ecs::system::{Commands, Query, Res, ResMut, Resource};

const CONSOLE_WIDTH: u32 = 80;
const CONSOLE_HEIGHT: u32 = 45;

#[derive(Copy, Component, Clone, Default, PartialEq)]
struct Position<C> {
    x: C,
    y: C,
}

#[derive(Component, Default)]
struct Player;

#[derive(Bundle)]
struct PlayerBundle {
    player: Player,
    position: Position<i32>,
}

#[derive(Component, Default)]
struct Mouse;

#[derive(Bundle)]
struct MouseBundle {
    mouse: Mouse,
    position: Position<f32>,
}

#[derive(Resource)]
struct Entities {
    player: Entity,
    mouse: Entity,
}

#[derive(Default, Resource)]
struct ScreenshotIndex(usize);

pub fn main() {
    App::new()
        .insert_resource(DoryenPluginSettings {
            // here are all the available options.
            // better practice is to use default values (see other examples)
            app_options: AppOptions {
                console_width: CONSOLE_WIDTH,
                console_height: CONSOLE_HEIGHT,
                screen_width: CONSOLE_WIDTH * 8,
                screen_height: CONSOLE_HEIGHT * 8,
                window_title: String::from("my roguelike"),
                font_path: String::from("terminal_8x8.png"),
                vsync: true,
                fullscreen: false,
                show_cursor: true,
                resizable: true,
                intercept_close_request: false,
                max_fps: 0,
            },
            ..Default::default()
        })
        .add_plugins(DoryenPlugin)
        .init_resource::<ScreenshotIndex>()
        .add_systems(Startup, init)
        .add_systems(Update, input)
        .add_systems(Render, render)
        .run();
}

fn init(mut root_console: ResMut<RootConsole>, mut commands: Commands) {
    root_console.register_color("white", (255, 255, 255, 255));
    root_console.register_color("red", (255, 92, 92, 255));
    root_console.register_color("blue", (192, 192, 255, 255));

    let player = commands
        .spawn(PlayerBundle {
            player: Player,
            position: Position {
                x: (CONSOLE_WIDTH / 2) as i32,
                y: (CONSOLE_HEIGHT / 2) as i32,
            },
        })
        .id();

    let mouse = commands
        .spawn(MouseBundle {
            mouse: Mouse,
            position: Position { x: 0., y: 0. },
        })
        .id();

    commands.insert_resource(Entities { player, mouse });
}

fn input(
    input: Res<Input>,
    entities: Res<Entities>,
    mut player_query: Query<(&mut Position<i32>, &Player)>,
    mut mouse_query: Query<(&mut Position<f32>, &Mouse)>,
    mut screenshot_index: ResMut<ScreenshotIndex>,
    mut capture_events: EventWriter<Capture>,
) {
    let mut player_position = player_query
        .get_component_mut::<Position<i32>>(entities.player)
        .unwrap();

    if input.key("ArrowLeft") {
        player_position.x = (player_position.x - 1).max(1);
    } else if input.key("ArrowRight") {
        player_position.x = (player_position.x + 1).min(CONSOLE_WIDTH as i32 - 2);
    }
    if input.key("ArrowUp") {
        player_position.y = (player_position.y - 1).max(1);
    } else if input.key("ArrowDown") {
        player_position.y = (player_position.y + 1).min(CONSOLE_HEIGHT as i32 - 2);
    }

    let mut mouse_position = mouse_query
        .get_component_mut::<Position<f32>>(entities.mouse)
        .unwrap();

    let new_mouse_position = input.mouse_pos();
    mouse_position.x = new_mouse_position.0;
    mouse_position.y = new_mouse_position.1;

    if input.key("ControlLeft") && input.key_pressed("KeyS") {
        screenshot_index.0 += 1;
        capture_events.send(Capture::new(format!(
            "screenshot_{:03}.png",
            screenshot_index.0
        )));
    }
}

fn render(
    entities: Res<Entities>,
    mut root_console: ResMut<RootConsole>,
    player_query: Query<(&Position<i32>, &Player)>,
    mouse_query: Query<(&Position<f32>, &Mouse)>,
) {
    root_console.rectangle(
        0,
        0,
        CONSOLE_WIDTH,
        CONSOLE_HEIGHT,
        Some((128, 128, 128, 255)),
        Some((0, 0, 0, 255)),
        Some('.' as u16),
    );
    root_console.area(
        10,
        10,
        5,
        5,
        Some((255, 64, 64, 255)),
        Some((128, 32, 32, 255)),
        Some('&' as u16),
    );

    let player_position = player_query
        .get_component::<Position<i32>>(entities.player)
        .unwrap();

    root_console.ascii(player_position.x, player_position.y, '@' as u16);
    root_console.fore(player_position.x, player_position.y, (255, 255, 255, 255));
    root_console.print_color(
        (CONSOLE_WIDTH / 2) as i32,
        (CONSOLE_HEIGHT - 1) as i32,
        "#[red]arrows#[white] : move - #[red]CTRL-S#[white] : save screenshot",
        TextAlign::Center,
        None,
    );

    let mouse_position = mouse_query
        .get_component::<Position<f32>>(entities.mouse)
        .unwrap();

    root_console.print_color(
        (CONSOLE_WIDTH / 2) as i32,
        (CONSOLE_HEIGHT - 3) as i32,
        &format!(
            "#[white]Mouse coordinates: #[red]{}, {}",
            mouse_position.x, mouse_position.y
        ),
        TextAlign::Center,
        None,
    );
    root_console.print_color(
        5,
        5,
        "#[blue]This blue text contains a #[red]red#[] word",
        TextAlign::Left,
        None,
    );
    root_console.back(
        mouse_position.x as i32,
        mouse_position.y as i32,
        (255, 255, 255, 255),
    );
}
