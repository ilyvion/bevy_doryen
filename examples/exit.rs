use bevy_app::{App, AppExit, Events};
use bevy_doryen::doryen::{AppOptions, Color, TextAlign};
use bevy_doryen::{
    DoryenInput, DoryenPlugin, DoryenRenderSystemExtensions, DoryenRootConsole, DoryenSettings,
};
use bevy_ecs::{IntoSystem, Res, ResMut};

const WHITE: Color = (255, 255, 255, 255);

#[derive(Default)]
struct CloseRequested(bool);

fn main() {
    App::build()
        .add_resource(DoryenSettings {
            app_options: Some(AppOptions {
                intercept_close_request: true,
                ..Default::default()
            }),
            ..Default::default()
        })
        .add_plugin(DoryenPlugin)
        .init_resource::<CloseRequested>()
        .add_system(process_input.system())
        .add_doryen_render_system(render.system())
        .run();
}

fn process_input(
    input: Res<DoryenInput>,
    mut close_requested: ResMut<CloseRequested>,
    mut app_exit: ResMut<Events<AppExit>>,
) {
    if close_requested.0 {
        if input.key("KeyY") {
            app_exit.send(AppExit);
        } else if input.key("KeyN") {
            close_requested.0 = false;
        }
    } else if input.key("Escape") || input.close_requested() {
        close_requested.0 = true;
    }
}

fn render(mut root_console: ResMut<DoryenRootConsole>, close_requested: Res<CloseRequested>) {
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
            "Press ESC to exit",
            TextAlign::Left,
            Some(WHITE),
            None,
        );
    }
}
