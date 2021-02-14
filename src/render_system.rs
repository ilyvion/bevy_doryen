use crate::render_stage;
use bevy_app::AppBuilder;
use bevy_ecs::{Schedule, System, SystemStage};

pub(crate) struct DoryenRenderSystems(pub(crate) Option<Schedule>);
impl Default for DoryenRenderSystems {
    fn default() -> Self {
        let mut doryen_render_systems = Self(Some(Default::default()));

        let schedule = doryen_render_systems.0.as_mut().unwrap();
        schedule
            .add_stage(render_stage::FIRST, SystemStage::serial())
            .add_stage(render_stage::PRE_RENDER, SystemStage::serial())
            .add_stage(render_stage::RENDER, SystemStage::serial())
            .add_stage(render_stage::POST_RENDER, SystemStage::serial())
            .add_stage(render_stage::LAST, SystemStage::serial());

        doryen_render_systems
    }
}

/// Adds methods to the [`AppBuilder`] for adding systems to the Doryen
/// [`render`](crate::doryen::Engine::render) schedule.
pub trait DoryenRenderSystemExtensions {
    /// Adds a system to the [`RENDER`](crate::render_stage::RENDER) stage of the
    /// render schedule.
    fn add_doryen_render_system<S: System<In = (), Out = ()>>(&mut self, system: S) -> &mut Self;
    /// Adds a system to the given stage of the render schedule.
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
            .as_mut()
            .unwrap()
            .add_system_to_stage(render_stage::RENDER, system);
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
            .as_mut()
            .unwrap()
            .add_system_to_stage(stage_name, system);
        drop(doryen_render_systems);

        self
    }
}
