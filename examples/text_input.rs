use bevy_app::App;
use bevy_doryen::doryen::{AppOptions, Color, TextAlign};
use bevy_doryen::{DoryenPlugin, DoryenPluginSettings, Input, RenderSystemExtensions, RootConsole};
use bevy_ecs::system::{IntoSystem, Res, ResMut};
use unicode_segmentation::UnicodeSegmentation;

const WHITE: Color = (255, 255, 255, 255);

struct TextInput {
    text: String,
    cursor: usize,
}

impl Default for TextInput {
    fn default() -> Self {
        Self {
            text: String::new(),
            cursor: 0,
        }
    }
}

fn main() {
    App::build()
        .insert_resource(DoryenPluginSettings {
            app_options: AppOptions {
                window_title: String::from("bevy_doryen subcell resolution demo"),
                ..Default::default()
            },
            ..Default::default()
        })
        .add_plugin(DoryenPlugin)
        .init_resource::<TextInput>()
        .add_system(update.system())
        .add_doryen_render_system(render.system())
        .run();
}

fn update(input: Res<Input>, mut text_input: ResMut<TextInput>) {
    // input.text returns the characters typed by the player since last update
    let text = input.text();
    if !text.is_empty() {
        text_input.text.push_str(&text);
    }
    // handle backspace
    if input.key_released("Backspace") && !text_input.text.is_empty() {
        // convoluted way to remove the last character of the string
        // in a way that also works with utf-8 graphemes
        // where one character != one byte
        let mut graphemes = text_input.text.graphemes(true).rev();
        graphemes.next();
        text_input.text = graphemes.rev().collect();
    }
    // handle tab
    if input.key_released("Tab") {
        text_input.text.push_str("   ");
    }
    text_input.cursor += 1;
}

fn render(mut root_console: ResMut<RootConsole>, text_input: Res<TextInput>) {
    root_console.clear(None, None, Some(' ' as u16));
    root_console.print(
        5,
        5,
        &format!(
            "Type some text : {}{}",
            text_input.text,
            // blinking cursor
            if text_input.cursor % 25 < 12 {
                '_'
            } else {
                ' '
            }
        ),
        TextAlign::Left,
        Some(WHITE),
        None,
    );
}
