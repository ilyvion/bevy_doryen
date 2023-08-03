use bevy_app::App;
use bevy_doryen::doryen::{AppOptions, TextAlign};
use bevy_doryen::{DoryenPlugin, DoryenPluginSettings, FpsInfo, Render, ResizeMode, RootConsole};
use bevy_ecs::system::{Res, ResMut, Resource};

#[derive(Resource)]
struct PerfTest {
    seed: u64,
}

impl PerfTest {
    pub fn new() -> Self {
        Self { seed: 0xdead_beef }
    }
    fn rnd(&mut self) -> u64 {
        self.seed = 214_013u64.wrapping_mul(self.seed).wrapping_add(2_531_011);
        self.seed
    }
}

pub fn main() {
    App::new()
        .insert_resource(DoryenPluginSettings {
            app_options: AppOptions {
                window_title: String::from("bevy_doryen performance test"),
                vsync: false,
                ..Default::default()
            },
            resize_mode: ResizeMode::Automatic,
            ..Default::default()
        })
        .add_plugins(DoryenPlugin)
        .insert_resource(PerfTest::new())
        .add_systems(Render, render)
        .run();
}

fn render(
    mut root_console: ResMut<RootConsole>,
    mut perf_test: ResMut<PerfTest>,
    fps_info: Res<FpsInfo>,
) {
    let fps = fps_info.fps;
    let con_width = root_console.get_width();
    let con_height = root_console.get_height();
    for y in 0..con_height as i32 {
        for x in 0..con_width as i32 {
            let val = perf_test.rnd();
            root_console.back(
                x,
                y,
                (
                    (val & 0xFF) as u8,
                    ((val >> 8) & 0x5F) as u8,
                    ((val >> 16) & 0x3F) as u8,
                    255,
                ),
            );
            root_console.fore(
                x,
                y,
                (
                    ((val >> 16) & 0xFF) as u8,
                    ((val >> 24) & 0xFF) as u8,
                    ((val >> 32) & 0xFF) as u8,
                    255,
                ),
            );
            root_console.ascii(x, y, ((val >> 40) & 0xFF) as u16);
        }
    }
    root_console.rectangle(
        (con_width / 2 - 10) as i32,
        (con_height / 2 - 2) as i32,
        20,
        5,
        Some((255, 255, 255, 255)),
        Some((0, 0, 0, 255)),
        Some(' ' as u16),
    );
    root_console.print(
        (con_width / 2) as i32,
        (con_height / 2) as i32,
        &format!("{} fps", fps),
        TextAlign::Center,
        Some((255, 255, 255, 255)),
        None,
    );
}
