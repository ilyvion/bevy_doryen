use bevy_app::App;
use bevy_doryen::doryen::AppOptions;
use bevy_doryen::{DoryenPlugin, DoryenPluginSettings, RenderSystemExtensions, RootConsole};
use bevy_ecs::bundle::Bundle;
use bevy_ecs::entity::Entity;
use bevy_ecs::system::{Commands, IntoSystem, Query, Res, ResMut};

#[derive(Default)]
struct Circle;

#[derive(Default, Copy, Clone, PartialEq)]
struct Position {
    x: f32,
    y: f32,
}

#[derive(Default, Copy, Clone, PartialEq)]
struct Radius(f32);

#[derive(Default, Copy, Clone, PartialEq)]
struct Angle(f32);

#[derive(Bundle)]
struct CircleBundle {
    circle: Circle,
    position: Position,
    radius: Radius,
    angle: Angle,
}

struct Entities {
    circle: Entity,
}

fn main() {
    App::build()
        .insert_resource(DoryenPluginSettings {
            app_options: AppOptions {
                window_title: String::from("alpha test"),
                ..Default::default()
            },
            ..Default::default()
        })
        .add_plugin(DoryenPlugin)
        .add_startup_system(init.system())
        .add_system(update_circle.system())
        .add_doryen_render_system(render.system())
        .run();
}

fn init(mut commands: Commands) {
    let circle = commands
        .spawn(CircleBundle {
            circle: Circle,
            position: Position { x: 0., y: 0. },
            radius: Radius(10.),
            angle: Angle(0.),
        })
        .current_entity()
        .unwrap();
    commands.insert_resource(Entities { circle });
}

fn update_circle(
    root_console: Res<RootConsole>,
    entities: Res<Entities>,
    mut circle_query: Query<(&mut Position, &mut Radius, &mut Angle, &Circle)>,
) {
    let (mut position, mut radius, mut angle, _) = circle_query.get_mut(entities.circle).unwrap();

    // update the circle radius and center position
    angle.0 += 0.6;
    radius.0 = 10.0 + 3.0 * (angle.0 / 10.0).sin();
    let cs = (angle.0 / 20.0).cos();
    let sn = (angle.0 / 15.0).sin();
    position.x = (root_console.get_width() / 2) as f32 + cs * 15.0;
    position.y = (root_console.get_height() / 2) as f32 + sn * 15.0;
}

fn render(
    entities: Res<Entities>,
    mut root_console: ResMut<RootConsole>,
    circle_query: Query<(&Position, &Radius, &Angle, &Circle)>,
) {
    // fill the console with transparent black. The more opaque it is, the faster the previous frames will fade to black.
    // replace alpha with a lower value, like 10 or 5 and the effect will last longer.
    root_console.clear(None, Some((0, 0, 0, 20)), None);
    let (position, radius, angle, _) = circle_query.get(entities.circle).unwrap();
    // here we render current frame (only a circle of blue dots)
    for r in 0..10 {
        let angle = angle.0 + r as f32 * std::f32::consts::PI * 2.0 / 10.0;
        let cs = angle.cos();
        let sn = angle.sin();
        let x = position.x + radius.0 * cs;
        let y = position.y + radius.0 * sn;
        root_console.back(x as i32, y as i32, (0, 0, 255, 255));
    }
}
