use bevy_app::{App, Plugin};
use bevy_ecs::schedule::{ExecutorKind, InternedScheduleLabel, Schedule, ScheduleLabel};
use bevy_ecs::system::Resource;
use bevy_ecs::world::{Mut, World};

/// The schedule that contains the render logic that is evaluated each tick of [`doryen_rs::Engine::render`].
///
/// By default, it will run the following schedules in the given order:
///
/// * [`First`]
/// * [`PreRender`]
/// * [`Render`]
/// * [`PostRender`]
/// * [`Last`]
#[derive(ScheduleLabel, Clone, Copy, Debug, Eq, Hash, PartialEq)]
pub struct MainRender;

/// Runs first in the schedule.
/// This is run by the [`MainRender`] schedule.
#[derive(ScheduleLabel, Clone, Copy, Debug, Eq, Hash, PartialEq)]
pub struct First;

/// The schedule that contains logic that must run before [`Render`].
///
/// This is run by the [`MainRender`] schedule.
#[derive(ScheduleLabel, Clone, Copy, Debug, Eq, Hash, PartialEq)]
pub struct PreRender;

/// The schedule that contains render logic.
///
/// This is run by the [`MainRender`] schedule.
#[derive(ScheduleLabel, Clone, Copy, Debug, Eq, Hash, PartialEq)]
pub struct Render;

/// The schedule that contains logic that must run after [`Render`].
///
/// This is run by the [`MainRender`] schedule.
#[derive(ScheduleLabel, Clone, Copy, Debug, Eq, Hash, PartialEq)]
pub struct PostRender;

/// Runs last in the schedule.
/// This is run by the [`MainRender`] schedule.
#[derive(ScheduleLabel, Clone, Copy, Debug, Eq, Hash, PartialEq)]
pub struct Last;

/// Defines the schedules to be run for the [`Render`] schedule, including
/// their order.
#[derive(Debug, Resource)]
pub struct MainRenderScheduleOrder {
    /// The labels to run for the [`MainRender`] schedule (in the order they will be run).
    pub labels: Vec<InternedScheduleLabel>,
}

impl Default for MainRenderScheduleOrder {
    fn default() -> Self {
        Self {
            labels: vec![
                First.intern(),
                PreRender.intern(),
                Render.intern(),
                PostRender.intern(),
                Last.intern(),
            ],
        }
    }
}

impl MainRender {
    /// A system that runs the "main render schedule"
    pub fn run_main_render(world: &mut World) {
        #[allow(clippy::shadow_unrelated)]
        world.resource_scope(|world, order: Mut<'_, MainRenderScheduleOrder>| {
            for &label in &order.labels {
                let _ = world.try_run_schedule(label);
            }
        });
    }
}

/// Initializes the [`Render`] schedule, sub schedules, and resources for a given [`App`].
#[derive(Copy, Clone, Debug)]
pub struct RenderSchedulePlugin;

impl Plugin for RenderSchedulePlugin {
    fn build(&self, app: &mut App) {
        // simple "facilitator" schedules benefit from simpler single threaded scheduling
        let mut render_schedule = Schedule::new(MainRender);
        render_schedule.set_executor_kind(ExecutorKind::SingleThreaded);

        app.add_schedule(render_schedule)
            .init_resource::<MainRenderScheduleOrder>()
            .add_systems(MainRender, MainRender::run_main_render);
    }
}
