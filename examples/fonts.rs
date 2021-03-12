use bevy_app::{App, Events};
use bevy_doryen::doryen::{AppOptions, TextAlign};
use bevy_doryen::{
    DoryenPlugin, DoryenPluginSettings, Input, RenderSystemExtensions, RootConsole, SetFontPath,
};
use bevy_ecs::{IntoSystem, Res, ResMut};

const CONSOLE_WIDTH: u32 = 40;
const CONSOLE_HEIGHT: u32 = 25;

const FONTS: [&str; 19] = [
    "terminal_8x8.png",
    "terminal_colored_8x8.png",
    "terminal_8x12.png",
    "terminal_10x16.png",
    "terminal_12x12.png",
    "SmoothWalls_9x9.png",
    "Aesomatica_16x16.png",
    "Bisasam_20x20.png",
    "Buddy--graphical_10x10.png",
    "Cheepicus_8x8.png",
    "Cheepicus_15x15.png",
    "Cheepicus_16x16.png",
    "Herrbdog_12x12.png",
    "Kein_5x5.png",
    "Mkv_curses_6x6.png",
    "Runeset_24x24.png",
    "Teeto_K_18x18.png",
    "Terbert_7x7.png",
    "Yayo_tunur_13x13.png",
];

#[derive(Default)]
struct Font {
    current_font: usize,
    current_font_name: &'static str,
}

fn main() {
    App::build()
        .add_resource(DoryenPluginSettings {
            app_options: AppOptions {
                console_width: CONSOLE_WIDTH,
                console_height: CONSOLE_HEIGHT,
                screen_width: CONSOLE_WIDTH * 24,
                screen_height: CONSOLE_HEIGHT * 24,
                window_title: String::from("bevy_doryen font test"),
                ..Default::default()
            },
            ..Default::default()
        })
        .add_plugin(DoryenPlugin)
        .init_resource::<Font>()
        .add_system(update.system())
        .add_doryen_render_system(render.system())
        .run();
}

fn update(
    mut font: ResMut<Font>,
    input: Res<Input>,
    mut set_font_path: ResMut<Events<SetFontPath>>,
) {
    let mut font_path = None;
    if input.key_released("PageDown") {
        font.current_font = (font.current_font + 1) % FONTS.len();
        font_path = Some(FONTS[font.current_font]);
    } else if input.key_released("PageUp") {
        font.current_font = (font.current_font + FONTS.len() - 1) % FONTS.len();
        font_path = Some(FONTS[font.current_font]);
    }

    if let Some(font_path) = font_path {
        font.current_font_name = font_path;
        set_font_path.send(SetFontPath(String::from(font_path).into()));
    }
}

fn render(mut root_console: ResMut<RootConsole>, font: Res<Font>) {
    root_console.rectangle(
        0,
        0,
        CONSOLE_WIDTH,
        CONSOLE_HEIGHT,
        Some((128, 128, 128, 255)),
        None,
        Some('.' as u16),
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
    root_console.ascii(
        (CONSOLE_WIDTH / 2) as i32,
        (CONSOLE_HEIGHT / 2 - 10) as i32,
        '@' as u16,
    );
    root_console.fore(
        (CONSOLE_WIDTH / 2) as i32,
        (CONSOLE_HEIGHT / 2 - 10) as i32,
        (255, 255, 255, 255),
    );
    root_console.rectangle(
        (CONSOLE_WIDTH / 2 - 20) as i32,
        (CONSOLE_HEIGHT / 2 - 2) as i32,
        40,
        7,
        Some((255, 255, 255, 255)),
        Some((0, 0, 0, 255)),
        Some(' ' as u16),
    );
    root_console.print(
        (CONSOLE_WIDTH / 2) as i32,
        (CONSOLE_HEIGHT / 2) as i32,
        &font.current_font_name,
        TextAlign::Center,
        Some((255, 255, 255, 255)),
        None,
    );
    root_console.print(
        (CONSOLE_WIDTH / 2) as i32,
        (CONSOLE_HEIGHT / 2) as i32 + 2,
        "PageUp/PageDown to change font",
        TextAlign::Center,
        Some((255, 192, 128, 255)),
        None,
    );
}
