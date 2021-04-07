use bevy_app::App;
use bevy_doryen::doryen::{AppOptions, Color, Console as DoryenConsole, TextAlign};
use bevy_doryen::{DoryenPlugin, DoryenPluginSettings, RenderSystemExtensions, RootConsole};
use bevy_ecs::bundle::Bundle;
use bevy_ecs::system::{Commands, IntoSystem, Query, Res, ResMut};

struct Console(DoryenConsole);

#[derive(Default, Copy, Clone, PartialEq)]
struct Position {
    x: i32,
    y: i32,
}

#[derive(Default, Copy, Clone, PartialEq)]
struct Speed {
    x: i32,
    y: i32,
}

#[derive(Default, Copy, Clone, PartialEq)]
struct Alpha {
    value: f32,
    step: f32,
    inverted: bool,
}

#[derive(Default, Copy, Clone, PartialEq)]
struct KeyColor(Option<Color>);

#[derive(Bundle)]
struct ConsoleBundle {
    console: Console,
    position: Position,
    speed: Speed,
    alpha: Alpha,
    key_color: KeyColor,
}

#[derive(Default, Copy, Clone, PartialEq)]
struct Step(usize);

fn main() {
    App::build()
        .insert_resource(DoryenPluginSettings {
            app_options: AppOptions {
                window_title: String::from("blitting demo"),
                ..Default::default()
            },
            ..Default::default()
        })
        .add_plugin(DoryenPlugin)
        .init_resource::<Step>()
        .add_startup_system(init.system())
        .add_system(update_position_and_speed.system())
        .add_system(update_alpha.system())
        .add_system(update_step.system())
        .add_doryen_render_system(render.system())
        .run();
}

fn init(mut commands: Commands) {
    let mut c1 = DoryenConsole::new(20, 20);
    let mut c2 = DoryenConsole::new(20, 20);
    for y in 0..20 {
        for x in 0..20 {
            c1.back(x, y, (((x + y * 10) % 255) as u8, 0, 0, 255));
            c2.back(
                x,
                y,
                if (x - 10) * (x - 10) + (y - 10) * (y - 10) < 100 {
                    (255, 192, 32, 255 - x as u8 * 10)
                } else {
                    (0, 0, 0, 255)
                },
            );
        }
    }
    c1.print(10, 10, "Hello", TextAlign::Center, None, None);
    c2.print(10, 10, "Circle", TextAlign::Center, None, None);

    commands.spawn_bundle(ConsoleBundle {
        console: Console(c1),
        position: Position { x: 5, y: 5 },
        speed: Speed { x: 1, y: 1 },
        alpha: Alpha {
            value: 1.0,
            step: 0.01,
            inverted: false,
        },
        key_color: KeyColor(None),
    });

    commands.spawn_bundle(ConsoleBundle {
        console: Console(c2),
        position: Position { x: 15, y: 20 },
        speed: Speed { x: -1, y: 1 },
        alpha: Alpha {
            value: 1.0,
            step: 0.01,
            inverted: true,
        },
        key_color: KeyColor(Some((0, 0, 0, 255))),
    });
}

fn update_position_and_speed(
    root_console: Res<RootConsole>,
    step: Res<Step>,
    mut position_speed_query: Query<(&mut Position, &mut Speed)>,
) {
    if step.0 == 0 {
        for (mut position, mut speed) in position_speed_query.iter_mut() {
            let size = (
                root_console.get_width() as i32,
                root_console.get_height() as i32,
            );
            position.x += speed.x;
            if position.x == size.0 - 20 || position.x == 0 {
                speed.x = -speed.x;
            }
            position.y += speed.y;
            if position.y == size.1 - 20 || position.y == 0 {
                speed.y = -speed.y;
            }
        }
    }
}

fn update_alpha(mut console_query: Query<(&mut Alpha, &Console)>) {
    for (mut alpha, _) in console_query.iter_mut() {
        if alpha.value <= 0.0 || alpha.value >= 1.0 {
            alpha.step = -alpha.step;
        }
        alpha.value += alpha.step;
    }
}

fn update_step(mut step: ResMut<Step>) {
    step.0 = (step.0 + 1) % 10;
}

fn render(
    mut root_console: ResMut<RootConsole>,
    console_query: Query<(&Position, &Alpha, &KeyColor, &Console)>,
) {
    let root_console = &mut **root_console;
    root_console.clear(Some((0, 0, 0, 255)), None, Some(' ' as u16));
    for x in 0..root_console.get_width() as i32 {
        for y in 0..root_console.get_height() as i32 {
            root_console.back(
                x,
                y,
                if (x + y) & 1 == 1 {
                    (96, 64, 32, 255)
                } else {
                    (32, 64, 96, 255)
                },
            );
        }
    }
    root_console.print(
        (root_console.get_width() / 2) as i32,
        (root_console.get_height() / 2) as i32,
        "You create offscreen consoles\nand blit them on other consoles",
        TextAlign::Center,
        Some((255, 255, 255, 255)),
        None,
    );

    for (position, alpha, key_color, console) in console_query.iter() {
        let alpha = if alpha.inverted {
            1.0 - alpha.value
        } else {
            alpha.value
        };
        console.0.blit(
            position.x,
            position.y,
            root_console,
            alpha,
            alpha,
            key_color.0,
        );
    }
}
