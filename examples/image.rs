use bevy_app::{App, Update};
use bevy_doryen::doryen::{AppOptions, Image};
use bevy_doryen::{DoryenPlugin, DoryenPluginSettings, Render, RootConsole};
use bevy_ecs::system::{ResMut, Resource};

#[derive(Resource)]
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

fn main() {
    App::new()
        .insert_resource(DoryenPluginSettings {
            app_options: AppOptions {
                window_title: String::from("bevy_doryen image demo"),
                ..Default::default()
            },
            ..Default::default()
        })
        .add_plugins(DoryenPlugin)
        .init_resource::<SkullImage>()
        .add_systems(Update, update)
        .add_systems(Render, render)
        .run();
}

fn update(mut skull: ResMut<SkullImage>) {
    skull.angle += 0.01;
    skull.scale_time += 0.01;
}

fn render(mut root_console: ResMut<RootConsole>, mut skull: ResMut<SkullImage>) {
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
