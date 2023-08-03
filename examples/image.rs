use bevy_app::{App, Startup, Update};
use bevy_doryen::doryen::{AppOptions, Image};
use bevy_doryen::{DoryenPlugin, DoryenPluginSettings, Render, RootConsole};
use bevy_ecs::system::{NonSendMut, ResMut};
use bevy_ecs::world::World;

struct SkullImage {
    skull: Image,
    angle: f32,
    scale_time: f32,
}

impl Default for SkullImage {
    fn default() -> Self {
        Self {
            skull: Image::new("skull.png"),
            angle: 0.0,
            scale_time: 0.0,
        }
    }
}

pub fn main() {
    App::new()
        .insert_resource(DoryenPluginSettings {
            app_options: AppOptions {
                window_title: String::from("bevy_doryen image demo"),
                ..Default::default()
            },
            ..Default::default()
        })
        .add_plugins(DoryenPlugin)
        .add_systems(Startup, init)
        .add_systems(Update, update)
        .add_systems(Render, render)
        .run();
}

// Because `Image` is non-`Send` when compiling for WASM, we'll just
// use a "non-send" resource always for the sake of simplicity in this
// example.
fn init(world: &mut World) {
    world.insert_non_send_resource(SkullImage::default());
}

fn update(mut skull: NonSendMut<SkullImage>) {
    skull.angle += 0.01;
    skull.scale_time += 0.01;
}

fn render(mut root_console: ResMut<RootConsole>, mut skull: NonSendMut<SkullImage>) {
    let root_console = &mut **root_console;
    let skull = &mut *skull;
    let scale = skull.scale_time.cos();
    root_console.clear(None, Some((0, 0, 0, 255)), None);
    skull.skull.blit_ex(
        root_console,
        (root_console.get_width() / 2) as f32,
        (root_console.get_height() / 2) as f32,
        scale,
        scale,
        skull.angle,
        None,
    );
}
