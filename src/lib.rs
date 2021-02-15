mod input;
mod render_system;
mod root_console;

/// Re-export of the Doryen library types.
pub mod doryen {
    pub use doryen_rs::*;
}

pub use input::{Input, Keys, MouseButton};
pub use render_system::RenderSystemExtensions;
pub use root_console::RootConsole;

use crate::doryen::{AppOptions, Console};
use crate::render_system::DoryenRenderSystems;
use bevy_app::{App as BevyApp, AppBuilder, AppExit, EventReader, Events, Plugin};
use bevy_ecs::Schedule;
use doryen_rs::{App as DoryenApp, DoryenApi, Engine, UpdateEvent};

/// The Bevy Doryen plugin.
#[derive(Default)]
pub struct DoryenPlugin;

/// DoryenPlugin settings.
pub struct DoryenPluginSettings {
    /// The [`AppOptions`] passed to the [`DoryenApp`].
    pub app_options: AppOptions,
    /// Which mouse buttons to request input data for from Doryen during the
    /// input handling.
    /// Defaults to left, middle and right mouse buttons.
    pub mouse_button_listeners: Vec<MouseButton>,
    /// What to do when the Doryen window is resized.
    pub resize_mode: ResizeMode,
}

impl Default for DoryenPluginSettings {
    fn default() -> Self {
        Self {
            app_options: Default::default(),
            mouse_button_listeners: vec![
                MouseButton::Left,
                MouseButton::Middle,
                MouseButton::Right,
            ],
            resize_mode: ResizeMode::Nothing,
        }
    }
}

/// Constants for the Doryen plugin render stages
pub mod render_stage {
    pub const FIRST: &str = "first";
    pub const PRE_RENDER: &str = "pre_render";
    pub const RENDER: &str = "render";
    pub const POST_RENDER: &str = "post_render";
    pub const LAST: &str = "last";
}

impl Plugin for DoryenPlugin {
    fn build(&self, app: &mut AppBuilder) {
        app.init_resource::<RootConsole>()
            .init_resource::<Input>()
            .init_resource::<FpsInfo>()
            .add_event::<SetFontPath>()
            .add_event::<Resized>()
            .init_resource::<DoryenRenderSystems>()
            .set_runner(doryen_runner);
    }
}

struct DoryenPluginEngine {
    bevy_app: BevyApp,
    app_exit_event_reader: EventReader<AppExit>,
    set_font_path_event_reader: EventReader<SetFontPath>,
    swap_console: Option<Console>,
    mouse_button_listeners: Vec<MouseButton>,
    previous_screen_size: (u32, u32),
    previous_console_size: (u32, u32),
    resize_mode: ResizeMode,
}

impl DoryenPluginEngine {
    #[inline]
    fn take_root_console_ownership(&mut self, api: &mut dyn DoryenApi) {
        use std::mem::swap;

        // Take ownership of the Doryen root console
        swap(api.con(), &mut self.swap_console.as_mut().unwrap());

        // Insert it into the DoryenRootConsole resource
        let mut doryen_root_console = self.bevy_app.resources.get_mut::<RootConsole>().unwrap();
        doryen_root_console.0 = self.swap_console.take();
    }

    #[inline]
    fn restore_root_console_ownership(&mut self, api: &mut dyn DoryenApi) {
        use std::mem::swap;

        // Take the root console out of the DoryenRootConsole resource
        let mut doryen_root_console = self.bevy_app.resources.get_mut::<RootConsole>().unwrap();
        self.swap_console = doryen_root_console.0.take();

        // Hand ownership of the Doryen root console back to Doryen
        swap(api.con(), &mut self.swap_console.as_mut().unwrap());
    }

    #[inline]
    fn take_doryen_render_schedule(&mut self) -> Schedule {
        let mut doryen_render_systems = self
            .bevy_app
            .resources
            .get_mut::<DoryenRenderSystems>()
            .unwrap();
        doryen_render_systems.0.take().unwrap()
    }

    #[inline]
    fn restore_doryen_render_schedule(&mut self, doryen_render_schedule: Schedule) {
        let mut doryen_render_systems = self
            .bevy_app
            .resources
            .get_mut::<DoryenRenderSystems>()
            .unwrap();
        doryen_render_systems.0.replace(doryen_render_schedule);
    }

    #[inline]
    fn handle_input(&mut self, api: &mut dyn DoryenApi) {
        let mut doryen_input = self.bevy_app.resources.get_mut::<Input>().unwrap();
        let input = api.input();
        doryen_input.handle_input(&self.mouse_button_listeners, input);
    }
}

impl Engine for DoryenPluginEngine {
    fn update(&mut self, api: &mut dyn DoryenApi) -> Option<UpdateEvent> {
        let mut doryen_fps_info = self.bevy_app.resources.get_mut::<FpsInfo>().unwrap();
        doryen_fps_info.fps = api.fps();
        doryen_fps_info.average_fps = api.average_fps();
        drop(doryen_fps_info);

        self.handle_input(api);

        self.take_root_console_ownership(api);
        self.bevy_app.update();
        self.restore_root_console_ownership(api);

        // Process the latest SetFontPath event
        let doryen_set_font_path_events = self
            .bevy_app
            .resources
            .get_mut::<Events<SetFontPath>>()
            .unwrap();
        if let Some(doryen_set_font_path) = self
            .set_font_path_event_reader
            .latest(&doryen_set_font_path_events)
        {
            api.set_font_path(&doryen_set_font_path.0);
        }

        if let Some(app_exit_events) = self.bevy_app.resources.get_mut::<Events<AppExit>>() {
            if self
                .app_exit_event_reader
                .latest(&app_exit_events)
                .is_some()
            {
                return Some(UpdateEvent::Exit);
            }
        }

        None
    }

    fn render(&mut self, api: &mut dyn DoryenApi) {
        self.take_root_console_ownership(api);

        let mut doryen_render_schedule = self.take_doryen_render_schedule();
        doryen_render_schedule
            .initialize_and_run(&mut self.bevy_app.world, &mut self.bevy_app.resources);
        self.restore_doryen_render_schedule(doryen_render_schedule);

        self.restore_root_console_ownership(api);
    }

    fn resize(&mut self, api: &mut dyn DoryenApi) {
        let (previous_width, previous_height) = self.previous_screen_size;
        let (new_width, new_height) = api.get_screen_size();

        let mut resized_events = self
            .bevy_app
            .resources
            .get_mut::<Events<Resized>>()
            .unwrap();
        let resized = Resized {
            previous_width,
            previous_height,
            new_width,
            new_height,
        };
        resized_events.send(resized);
        drop(resized_events);

        match self.resize_mode {
            ResizeMode::Nothing => (),
            ResizeMode::Automatic => {
                let (previous_console_width, previous_console_height) = self.previous_console_size;

                let w_ratio = previous_width / previous_console_width;
                let h_ratio = previous_height / previous_console_height;

                let new_console_width = new_width / w_ratio;
                let new_console_height = new_height / h_ratio;
                api.con().resize(new_console_width, new_console_height);
            }
            ResizeMode::Callback(callback) => {
                self.take_root_console_ownership(api);
                callback(&mut *self.bevy_app.resources.get_mut().unwrap(), resized);
                self.restore_root_console_ownership(api);
            }
        }

        self.previous_screen_size = (new_width, new_height);
        self.previous_console_size = api.con().get_size();
    }
}

fn doryen_runner(mut app: BevyApp) {
    let mut resource_settings = app
        .resources
        .get_or_insert_with(DoryenPluginSettings::default);
    let DoryenPluginSettings {
        app_options,
        mouse_button_listeners,
        resize_mode,
    } = std::mem::take(&mut *resource_settings);
    drop(resource_settings);

    let AppOptions {
        screen_height,
        screen_width,
        console_height,
        console_width,
        ..
    } = app_options;

    let mut doryen_app = DoryenApp::new(app_options);

    doryen_app.set_engine(Box::new(DoryenPluginEngine {
        bevy_app: app,
        app_exit_event_reader: Default::default(),
        set_font_path_event_reader: Default::default(),
        swap_console: Some(Console::new(1, 1)),
        mouse_button_listeners,
        previous_screen_size: (screen_width, screen_height),
        previous_console_size: (console_width, console_height),
        resize_mode,
    }));

    doryen_app.run();
}

#[derive(Default)]
pub struct FpsInfo {
    pub fps: u32,
    pub average_fps: u32,
}

pub struct SetFontPath(pub String);

#[derive(Debug, Clone, Copy)]
pub struct Resized {
    pub previous_width: u32,
    pub previous_height: u32,
    pub new_width: u32,
    pub new_height: u32,
}

/// How the [`DoryenPlugin`] reacts to the resize event from Doryen.
pub enum ResizeMode {
    /// Do nothing when the window is resized. This is the default behavior.
    Nothing,
    /// Set the console size to match the window size automatically. This
    /// retains the ratio defined between the console size and the screen size
    /// as given in the [`AppOptions`] at the start of the program.
    Automatic,
    /// Call the given function when the resize event is triggered. Because
    /// Doryen is sensitive to when the root console is resized, the safest
    /// place to make a call to do so and always have the correct behavior is
    /// during this resize callback which comes directly from Doryen itself. The
    /// [`Resized`] event is useful for reacting to resizing within Bevy
    /// systems for other reasons, but will arrive at a point that is too late
    /// to do the root console resizing correctly.
    Callback(fn(&mut RootConsole, Resized)),
}
