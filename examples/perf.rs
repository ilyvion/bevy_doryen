use bevy_app::{App, EventReader, Events};
use bevy_doryen::doryen::{AppOptions, TextAlign};
use bevy_doryen::{
    DoryenFpsInfo, DoryenPlugin, DoryenRenderSystemExtensions, DoryenRootConsole, DoryenSettings,
    Resized,
};
use bevy_ecs::{IntoSystem, Local, Res, ResMut};

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

fn main() {
    App::build()
        .add_resource(DoryenSettings {
            app_options: Some(AppOptions {
                window_title: String::from("bevy_doryen performance test"),
                vsync: false,
                ..Default::default()
            }),
            ..Default::default()
        })
        .add_plugin(DoryenPlugin)
        .add_resource(PerfTest::new())
        .add_system(resize.system())
        .add_doryen_render_system(render.system())
        .run();
}

fn resize(
    mut root_console: ResMut<DoryenRootConsole>,
    events: Res<Events<Resized>>,
    mut event_reader: Local<EventReader<Resized>>,
) {
    if let Some(resized) = event_reader.latest(&events) {
        let new_width = resized.width / 8;
        let new_height = resized.height / 8;
        root_console.resize(new_width, new_height);
    }
}

fn render(
    mut root_console: ResMut<DoryenRootConsole>,
    mut perf_test: ResMut<PerfTest>,
    fps_info: Res<DoryenFpsInfo>,
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
