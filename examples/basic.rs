use bevy_app::{stage, App};
use bevy_doryen::{Doryen, DoryenPlugin, DoryenRenderSystemExtensions, DoryenSettings};
use bevy_ecs::{Bundle, IntoSystem, Resources, World};
use doryen_rs::{AppOptions, TextAlign};

const CONSOLE_WIDTH: u32 = 80;
const CONSOLE_HEIGHT: u32 = 45;

#[derive(Default, Copy, Clone, PartialEq)]
struct Position<C> {
    x: C,
    y: C,
}

#[derive(Default)]
struct Player;

#[derive(Bundle)]
struct PlayerBundle {
    player: Player,
    position: Position<i32>,
}

#[derive(Default)]
struct Mouse;

#[derive(Bundle)]
struct MouseBundle {
    mouse: Mouse,
    position: Position<f32>,
}

fn main() {
    App::build()
        .add_resource(DoryenSettings {
            // here are all the available options.
            // better practice is to use default values (see other examples)
            app_options: Some(AppOptions {
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
            }),
        })
        .add_plugin(DoryenPlugin)
        .add_startup_system(init.system())
        .add_system_to_stage(stage::PRE_UPDATE, input.system())
        .add_doryen_render_system(render.system())
        .run()
}

fn init(world: &mut World, res: &mut Resources) {
    let mut doryen = res.get_thread_local_mut::<Doryen>().unwrap();

    let con = doryen.con_mut();
    con.register_color("white", (255, 255, 255, 255));
    con.register_color("red", (255, 92, 92, 255));
    con.register_color("blue", (192, 192, 255, 255));

    world.spawn(PlayerBundle {
        player: Player,
        position: Position {
            x: (CONSOLE_WIDTH / 2) as i32,
            y: (CONSOLE_HEIGHT / 2) as i32,
        },
    });

    world.spawn(MouseBundle {
        mouse: Mouse,
        position: Position { x: 0., y: 0. },
    });
}

fn input(world: &mut World, res: &mut Resources) {
    let mut doryen = res.get_thread_local_mut::<Doryen>().unwrap();
    let (mut player_position, _) = world
        .query_mut::<(&mut Position<i32>, &Player)>()
        .next()
        .unwrap();

    let input = doryen.input_mut();
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

    let (mut mouse_position, _) = world
        .query_mut::<(&mut Position<f32>, &Mouse)>()
        .next()
        .unwrap();

    let new_mouse_position = input.mouse_pos();
    mouse_position.x = new_mouse_position.0;
    mouse_position.y = new_mouse_position.1;
}

fn render(world: &mut World, res: &mut Resources) {
    let mut doryen = res.get_thread_local_mut::<Doryen>().unwrap();

    let con = doryen.con_mut();

    con.rectangle(
        0,
        0,
        CONSOLE_WIDTH,
        CONSOLE_HEIGHT,
        Some((128, 128, 128, 255)),
        Some((0, 0, 0, 255)),
        Some('.' as u16),
    );
    con.area(
        10,
        10,
        5,
        5,
        Some((255, 64, 64, 255)),
        Some((128, 32, 32, 255)),
        Some('&' as u16),
    );

    let (player_position, _) = world.query::<(&Position<i32>, &Player)>().next().unwrap();

    con.ascii(player_position.x, player_position.y, '@' as u16);
    con.fore(player_position.x, player_position.y, (255, 255, 255, 255));
    con.print_color(
        (CONSOLE_WIDTH / 2) as i32,
        (CONSOLE_HEIGHT - 1) as i32,
        "#[red]arrows#[white] : move",
        TextAlign::Center,
        None,
    );

    let (mouse_position, _) = world.query::<(&Position<f32>, &Mouse)>().next().unwrap();

    con.print_color(
        (CONSOLE_WIDTH / 2) as i32,
        (CONSOLE_HEIGHT - 3) as i32,
        &format!(
            "#[white]Mouse coordinates: #[red]{}, {}",
            mouse_position.x, mouse_position.y
        ),
        TextAlign::Center,
        None,
    );
    con.print_color(
        5,
        5,
        "#[blue]This blue text contains a #[red]red#[] word",
        TextAlign::Left,
        None,
    );
    con.back(
        mouse_position.x as i32,
        mouse_position.y as i32,
        (255, 255, 255, 255),
    );
}
