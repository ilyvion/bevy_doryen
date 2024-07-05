use bevy_app::{App, AppExit, Update};
use bevy_doryen::doryen::{AppOptions, Color, TextAlign};
use bevy_doryen::prelude::*;
use bevy_ecs::prelude::EventWriter;
use bevy_ecs::system::{Res, ResMut, Resource};

const WHITE: Color = (255, 255, 255, 255);

#[derive(Default, Resource)]
struct CloseRequested(bool);

pub fn main() {
    App::new()
        .insert_resource(DoryenPluginSettings {
            app_options: AppOptions {
                intercept_close_request: true,
                ..Default::default()
            },
            ..Default::default()
        })
        .add_plugins(DoryenPlugin)
        .init_resource::<CloseRequested>()
        .add_systems(Update, process_input)
        .add_systems(Render, render)
        .run();
}

fn process_input(
    input: Res<Input>,
    mut close_requested: ResMut<CloseRequested>,
    mut app_exit: EventWriter<AppExit>,
) {
    if close_requested.0 {
        if input.key("KeyY") {
            app_exit.send(AppExit::Success);
        } else if input.key("KeyN") {
            close_requested.0 = false;
        }
    } else if input.key("Escape") || input.close_requested() {
        close_requested.0 = true;
    }
}

fn render(mut root_console: ResMut<RootConsole>, close_requested: Res<CloseRequested>) {
    root_console.clear(None, None, Some(' ' as u16));
    if close_requested.0 {
        root_console.print(
            5,
            5,
            "Exit game ? (press Y or N)",
            TextAlign::Left,
            Some(WHITE),
            None,
        );
    } else {
        root_console.print(
            5,
            5,
            "Press ESC to exit (on web, this does nothing)",
            TextAlign::Left,
            Some(WHITE),
            None,
        );
    }
}
