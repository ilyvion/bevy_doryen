mod input;
mod root_console;

pub mod doryen {
    pub use doryen_rs::*;
}

pub use input::{DoryenInput, Keys};
pub use root_console::DoryenRootConsole;

use crate::doryen::{AppOptions, Console};
use bevy_app::{App as BevyApp, AppBuilder, AppExit, EventReader, Events, Plugin};
use bevy_ecs::{Schedule, System, SystemStage};
use doryen_rs::{App as DoryenApp, DoryenApi, Engine, UpdateEvent};

#[derive(Default)]
pub struct DoryenPlugin;

/// DoryenPlugin settings
pub struct DoryenSettings {
    /// The [`AppOptions`] passed to the [`DoryenApp`].
    pub app_options: Option<AppOptions>,
    /// Which mouse buttons to request input data for from Doryen.
    /// Defaults to 0 (left), 1 (middle) and 2 (right)
    pub mouse_button_listeners: Vec<usize>,
}

impl Default for DoryenSettings {
    fn default() -> Self {
        Self {
            app_options: Default::default(),
            mouse_button_listeners: vec![0, 1, 2],
        }
    }
}

pub mod stage {
    pub const FIRST: &str = "first";
    pub const PRE_RENDER: &str = "pre_render";
    pub const RENDER: &str = "render";
    pub const POST_RENDER: &str = "post_render";
    pub const LAST: &str = "last";
}

struct DoryenRenderSystems(Schedule);
impl Default for DoryenRenderSystems {
    fn default() -> Self {
        let mut doryen_render_systems = Self(Default::default());

        let schedule = &mut doryen_render_systems.0;
        schedule
            .add_stage(stage::FIRST, SystemStage::serial())
            .add_stage(stage::PRE_RENDER, SystemStage::serial())
            .add_stage(stage::RENDER, SystemStage::serial())
            .add_stage(stage::POST_RENDER, SystemStage::serial())
            .add_stage(stage::LAST, SystemStage::serial());

        doryen_render_systems
    }
}

pub trait DoryenRenderSystemExtensions {
    fn add_doryen_render_system<S: System<In = (), Out = ()>>(&mut self, system: S) -> &mut Self;
    fn add_doryen_render_system_to_stage<S: System<In = (), Out = ()>>(
        &mut self,
        stage_name: &'static str,
        system: S,
    ) -> &mut Self;
}

impl DoryenRenderSystemExtensions for AppBuilder {
    fn add_doryen_render_system<S: System<In = (), Out = ()>>(&mut self, system: S) -> &mut Self {
        let mut doryen_render_systems =
            self.app.resources.get_mut::<DoryenRenderSystems>().unwrap();
        doryen_render_systems
            .0
            .add_system_to_stage(stage::RENDER, system);
        drop(doryen_render_systems);

        self
    }

    fn add_doryen_render_system_to_stage<S: System<In = (), Out = ()>>(
        &mut self,
        stage_name: &'static str,
        system: S,
    ) -> &mut Self {
        let mut doryen_render_systems =
            self.app.resources.get_mut::<DoryenRenderSystems>().unwrap();
        doryen_render_systems
            .0
            .add_system_to_stage(stage_name, system);
        drop(doryen_render_systems);

        self
    }
}

impl Plugin for DoryenPlugin {
    fn build(&self, app: &mut AppBuilder) {
        app.init_resource::<DoryenRootConsole>()
            .init_resource::<DoryenInput>()
            .init_resource::<DoryenRenderSystems>()
            .set_runner(doryen_runner);
    }
}

struct DoryenPluginEngine {
    bevy_app: BevyApp,
    app_exit_event_reader: EventReader<AppExit>,
    swap_console: Option<Console>,
    mouse_button_listeners: Vec<usize>,
}

impl DoryenPluginEngine {
    fn take_root_console_ownership(&mut self, api: &mut dyn DoryenApi) {
        use std::mem::swap;

        // Take ownership of the Doryen root console
        swap(api.con(), &mut self.swap_console.as_mut().unwrap());

        // Insert it into the DoryenRootConsole resource
        let mut doryen_root_console = self
            .bevy_app
            .resources
            .get_mut::<DoryenRootConsole>()
            .unwrap();
        doryen_root_console.0 = self.swap_console.take();
    }

    fn restore_root_console_ownership(&mut self, api: &mut dyn DoryenApi) {
        use std::mem::swap;

        // Take the root console out of the DoryenRootConsole resource
        let mut doryen_root_console = self
            .bevy_app
            .resources
            .get_mut::<DoryenRootConsole>()
            .unwrap();
        self.swap_console = doryen_root_console.0.take();

        // Hand ownership of the Doryen root console back to Doryen
        swap(api.con(), &mut self.swap_console.as_mut().unwrap());
    }

    fn handle_input(&mut self, api: &mut dyn DoryenApi) {
        let mut doryen_input = self.bevy_app.resources.get_mut::<DoryenInput>().unwrap();
        let input = api.input();
        doryen_input.handle_input(&self.mouse_button_listeners, input);
    }
}

impl Engine for DoryenPluginEngine {
    fn update(&mut self, api: &mut dyn DoryenApi) -> Option<UpdateEvent> {
        self.handle_input(api);

        self.take_root_console_ownership(api);
        self.bevy_app.update();
        self.restore_root_console_ownership(api);

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

        // SAFETY: Doing this is okay because
        // 1) This `doryen_render_systems` reference won't outlive `Resources`
        // 2) No systems running in `doryen_render_systems` can get a hold of a
        //    reference to `doryen_render_systems`.
        let doryen_render_systems = unsafe {
            &mut *(&mut *self
                .bevy_app
                .resources
                .get_mut::<DoryenRenderSystems>()
                .unwrap() as *mut DoryenRenderSystems)
        };
        doryen_render_systems
            .0
            .initialize_and_run(&mut self.bevy_app.world, &mut self.bevy_app.resources);

        self.restore_root_console_ownership(api);
    }
}

pub fn doryen_runner(mut app: BevyApp) {
    let mut settings = app.resources.get_or_insert_with(DoryenSettings::default);
    let mut doryen_app = DoryenApp::new(settings.app_options.take().unwrap_or_default());
    let mouse_button_listeners = settings.mouse_button_listeners.clone();
    drop(settings);

    doryen_app.set_engine(Box::new(DoryenPluginEngine {
        bevy_app: app,
        app_exit_event_reader: Default::default(),
        swap_console: Some(Console::new(1, 1)),
        mouse_button_listeners,
    }));

    doryen_app.run();
}
