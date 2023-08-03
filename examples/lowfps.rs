use bevy_app::{App, Startup};
use bevy_doryen::doryen::{AppOptions, TextAlign};
use bevy_doryen::{DoryenPlugin, DoryenPluginSettings, FpsInfo, Render, RootConsole};
use bevy_ecs::system::{Res, ResMut};

/*
This example shows how you can lower the number of frames per second to limit CPU consumption
using the max_fps field in the AppOptions parameter.
Note that this has no effect on the tickrate at which update() is called which still is 60 times per second.
You can lower the tickrate by calling your world updating code only once every n calls.
*/

pub fn main() {
    App::new()
        .insert_resource(DoryenPluginSettings {
            app_options: AppOptions {
                window_title: String::from("lowfps test"),
                vsync: false,
                max_fps: 10,
                ..Default::default()
            },
            ..Default::default()
        })
        .add_plugins(DoryenPlugin)
        .add_systems(Startup, init)
        .add_systems(Render, render)
        .run();
}

fn init(mut root_console: ResMut<RootConsole>) {
    root_console.register_color("red", (255, 92, 92, 255));
}

fn render(fps: Res<FpsInfo>, mut root_console: ResMut<RootConsole>) {
    let fps = fps.fps;

    let x = (root_console.get_width() / 2) as i32;
    let y = (root_console.get_height() / 2) as i32;

    root_console.print_color(
        x,
        y,
        &format!("Frames since last second : #[red]{}", fps),
        TextAlign::Center,
        None,
    );
}
