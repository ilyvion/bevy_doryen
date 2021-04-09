use bevy_app::App;
use bevy_doryen::doryen::{AppOptions, Image, TextAlign};
use bevy_doryen::{DoryenPlugin, DoryenPluginSettings, Render, RootConsole};
use bevy_ecs::system::{ResMut, Resource};

#[derive(Resource)]
struct SkullImage {
    skull: Image,
}

impl Default for SkullImage {
    fn default() -> Self {
        Self {
            skull: Image::new("skull.png"),
        }
    }
}

fn main() {
    App::new()
        .insert_resource(DoryenPluginSettings {
            app_options: AppOptions {
                window_title: String::from("bevy_doryen subcell resolution demo"),
                ..Default::default()
            },
            ..Default::default()
        })
        .add_plugins(DoryenPlugin)
        .init_resource::<SkullImage>()
        .add_systems(Render, render)
        .run();
}

fn render(mut root_console: ResMut<RootConsole>, mut skull: ResMut<SkullImage>) {
    root_console.clear(None, Some((0, 0, 0, 255)), None);
    skull
        .skull
        .blit_2x(&mut root_console, 23, 0, 0, 0, None, None, None);
    root_console.print(
        40,
        4,
        "Those pixels\nare twice smaller\nthan a console cell.\nMagic!",
        TextAlign::Center,
        Some((0, 0, 0, 255)),
        None,
    );
}
