use bevy_app::{App as BevyApp, AppBuilder, AppExit, EventReader, Events, Plugin};
use bevy_ecs::{Schedule, System, SystemStage};
use doryen_rs::{App as DoryenApp, AppOptions, Console, DoryenApi, Engine, InputApi, UpdateEvent};

#[derive(Default)]
pub struct DoryenPlugin;

/// DoryenPlugin settings
#[derive(Default)]
pub struct DoryenSettings {
    /// The [`AppOptions`] passed to the [`DoryenApp`].
    pub app_options: Option<AppOptions>,
}

#[derive(Default)]
pub struct Doryen {
    doryen_api: Option<&'static mut dyn DoryenApi>,
}

impl Doryen {
    pub fn con_mut(&mut self) -> &mut Console {
        self.doryen_api.as_mut().unwrap().con()
    }

    pub fn input_mut(&mut self) -> &mut dyn InputApi {
        self.doryen_api.as_mut().unwrap().input()
    }
}

pub mod stage {
    pub const FIRST: &str = "first";
    pub const PRE_RENDER: &str = "pre_render";
    pub const RENDER: &str = "render";
    pub const POST_RENDER: &str = "post_render";
    pub const LAST: &str = "last";
}

#[derive(Default)]
struct DoryenRenderSystems(Schedule);

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
        let mut doryen_render_systems = DoryenRenderSystems::default();
        let schedule = &mut doryen_render_systems.0;
        schedule
            .add_stage(stage::FIRST, SystemStage::serial())
            .add_stage(stage::PRE_RENDER, SystemStage::serial())
            .add_stage(stage::RENDER, SystemStage::serial())
            .add_stage(stage::POST_RENDER, SystemStage::serial())
            .add_stage(stage::LAST, SystemStage::serial());

        app.init_thread_local_resource::<Doryen>()
            .add_resource(doryen_render_systems)
            .set_runner(doryen_runner);
    }
}

struct DoryenPluginEngine {
    bevy_app: BevyApp,
    app_exit_event_reader: EventReader<AppExit>,
}

impl Engine for DoryenPluginEngine {
    fn update(&mut self, api: &mut dyn DoryenApi) -> Option<UpdateEvent> {
        let mut doryen_api_resource = self
            .bevy_app
            .resources
            .get_thread_local_mut::<Doryen>()
            .unwrap();
        doryen_api_resource.doryen_api = Some(unsafe { std::mem::transmute(api) });
        drop(doryen_api_resource);
        self.bevy_app.update();
        let mut doryen_api_resource = self
            .bevy_app
            .resources
            .get_thread_local_mut::<Doryen>()
            .unwrap();
        doryen_api_resource.doryen_api = None;

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
        let mut doryen_api_resource = self
            .bevy_app
            .resources
            .get_thread_local_mut::<Doryen>()
            .unwrap();
        doryen_api_resource.doryen_api = Some(unsafe { std::mem::transmute(api) });
        drop(doryen_api_resource);

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

        let mut doryen_api_resource = self
            .bevy_app
            .resources
            .get_thread_local_mut::<Doryen>()
            .unwrap();
        doryen_api_resource.doryen_api = None;
    }
}

pub fn doryen_runner(mut app: BevyApp) {
    let mut settings = app.resources.get_or_insert_with(DoryenSettings::default);
    let mut doryen_app = DoryenApp::new(settings.app_options.take().unwrap_or_default());
    drop(settings);

    doryen_app.set_engine(Box::new(DoryenPluginEngine {
        bevy_app: app,
        app_exit_event_reader: Default::default(),
    }));

    doryen_app.run();
}
