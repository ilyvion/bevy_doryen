use crate::render_stage;
use bevy_app::AppBuilder;
use bevy_ecs::schedule::{Schedule, SystemStage};
use bevy_ecs::system::System;

pub(crate) struct DoryenRenderSystems(pub(crate) Option<Schedule>);
impl Default for DoryenRenderSystems {
    fn default() -> Self {
        let mut doryen_render_systems = Self(Some(Schedule::default()));

        let schedule = doryen_render_systems.0.as_mut().unwrap();
        schedule
            .add_stage(render_stage::FIRST, SystemStage::single_threaded())
            .add_stage_after(
                render_stage::FIRST,
                render_stage::PRE_RENDER,
                SystemStage::single_threaded(),
            )
            .add_stage_after(
                render_stage::PRE_RENDER,
                render_stage::RENDER,
                SystemStage::single_threaded(),
            )
            .add_stage_after(
                render_stage::RENDER,
                render_stage::POST_RENDER,
                SystemStage::single_threaded(),
            )
            .add_stage_after(
                render_stage::POST_RENDER,
                render_stage::LAST,
                SystemStage::single_threaded(),
            );

        doryen_render_systems
    }
}

/// Adds methods to the [`AppBuilder`] for adding systems to the Doryen
/// [`render`](crate::doryen::Engine::render) schedule.
pub trait RenderSystemExtensions {
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

impl RenderSystemExtensions for AppBuilder {
    fn add_doryen_render_system<S: System<In = (), Out = ()>>(&mut self, system: S) -> &mut Self {
        let mut doryen_render_systems = self
            .app
            .world
            .get_resource_mut::<DoryenRenderSystems>()
            .unwrap();
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
        let mut doryen_render_systems = self
            .app
            .world
            .get_resource_mut::<DoryenRenderSystems>()
            .unwrap();
        doryen_render_systems
            .0
            .as_mut()
            .unwrap()
            .add_system_to_stage(stage_name, system);
        drop(doryen_render_systems);

        self
    }
}
