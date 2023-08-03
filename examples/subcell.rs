use bevy_app::{App, Startup};
use bevy_doryen::doryen::{AppOptions, Image, TextAlign};
use bevy_doryen::{DoryenPlugin, DoryenPluginSettings, Render, RootConsole};
use bevy_ecs::system::{NonSendMut, ResMut};
use bevy_ecs::world::World;

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

pub fn main() {
    App::new()
        .insert_resource(DoryenPluginSettings {
            app_options: AppOptions {
                window_title: String::from("bevy_doryen subcell resolution demo"),
                ..Default::default()
            },
            ..Default::default()
        })
        .add_plugins(DoryenPlugin)
        .add_systems(Startup, init)
        .add_systems(Render, render)
        .run();
}

// Because `Image` is non-`Send` when compiling for WASM, we'll just
// use a "non-send" resource always for the sake of simplicity in this
// example.
fn init(world: &mut World) {
    world.insert_non_send_resource(SkullImage::default());
}

fn render(mut root_console: ResMut<RootConsole>, mut skull: NonSendMut<SkullImage>) {
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
