use bevy_app::{App, EventReader, Events};
use bevy_doryen::doryen::{AppOptions, TextAlign, DEFAULT_CONSOLE_HEIGHT, DEFAULT_CONSOLE_WIDTH};
use bevy_doryen::{
    DoryenPlugin, DoryenPluginSettings, Input, RenderSystemExtensions, ResizeMode, Resized,
    RootConsole,
};
use bevy_ecs::system::{IntoSystem, Local, Res, ResMut};

struct ResizeData {
    width: u32,
    height: u32,
    mouse_pos: (f32, f32),
}

fn main() {
    App::build()
        .insert_resource(DoryenPluginSettings {
            app_options: AppOptions {
                window_title: String::from("resizable console"),
                vsync: false,
                ..Default::default()
            },
            resize_mode: ResizeMode::Callback(resize_callback),
            ..Default::default()
        })
        .add_plugin(DoryenPlugin)
        .insert_resource(ResizeData {
            width: DEFAULT_CONSOLE_WIDTH,
            height: DEFAULT_CONSOLE_HEIGHT,
            mouse_pos: (0.0, 0.0),
        })
        .add_system(update_mouse_position.system())
        .add_system(resize_events.system())
        .add_doryen_render_system(render.system())
        .run();
}

fn update_mouse_position(mut resize_data: ResMut<ResizeData>, input: Res<Input>) {
    resize_data.mouse_pos = input.mouse_pos();
}

fn resize_callback(root_console: &mut RootConsole, resized: Resized) {
    root_console.resize(resized.new_width / 8, resized.new_height / 8)
}

fn resize_events(
    mut resize_data: ResMut<ResizeData>,
    //events: Res<Events<Resized>>,
    mut event_reader: EventReader<Resized>,
) {
    if let Some(resized) = event_reader.iter().last() {
        resize_data.width = resized.new_width / 8;
        resize_data.height = resized.new_height / 8;
    }
}

fn render(mut root_console: ResMut<RootConsole>, resize_data: Res<ResizeData>) {
    root_console.rectangle(
        0,
        0,
        resize_data.width,
        resize_data.height,
        Some((128, 128, 128, 255)),
        Some((0, 0, 0, 255)),
        Some(' ' as u16),
    );
    root_console.area(
        10,
        10,
        5,
        5,
        Some((255, 64, 64, 255)),
        Some((128, 32, 32, 255)),
        Some('&' as u16),
    );
    root_console.print(
        (resize_data.width / 2) as i32,
        (resize_data.height / 2) as i32,
        &format!("{} x {}", resize_data.width, resize_data.height),
        TextAlign::Center,
        None,
        None,
    );
    root_console.back(
        resize_data.mouse_pos.0 as i32,
        resize_data.mouse_pos.1 as i32,
        (255, 255, 255, 255),
    );
}
