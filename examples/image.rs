use bevy_app::App;
use bevy_doryen::doryen::{AppOptions, Image};
use bevy_doryen::{DoryenPlugin, DoryenRenderSystemExtensions, DoryenRootConsole, DoryenSettings};
use bevy_ecs::{IntoSystem, ResMut};

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
    App::build()
        .add_resource(DoryenSettings {
            app_options: AppOptions {
                window_title: String::from("bevy_doryen image demo"),
                ..Default::default()
            },
            ..Default::default()
        })
        .add_plugin(DoryenPlugin)
        .init_resource::<SkullImage>()
        .add_system(update.system())
        .add_doryen_render_system(render.system())
        .run();
}

fn update(mut skull: ResMut<SkullImage>) {
    skull.angle += 0.01;
    skull.scale_time += 0.01;
}

fn render(mut root_console: ResMut<DoryenRootConsole>, mut skull: ResMut<SkullImage>) {
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
