# bevy_doryen
A [Bevy](https://bevyengine.org/) plugin that integrates the
[Doryen](https://github.com/jice-nospam/doryen-rs) roguelike library with Bevy.

[![Crates.io](https://img.shields.io/crates/v/bevy_doryen)](https://crates.io/crates/bevy_doryen)
[![Docs.io](https://docs.rs/bevy_doryen/badge.svg)](https://docs.rs/bevy_doryen)
[![GitHub](https://github.com/alexschrod/bevy_doryen/actions/workflows/rust.yml/badge.svg)](https://github.com/alexschrod/bevy_doryen)
[![Bevy tracking](https://img.shields.io/badge/Bevy%20tracking-released%20version-lightblue)](https://github.com/bevyengine/bevy/blob/main/docs/plugins_guidelines.md#main-branch-tracking)

## Usage

### Targeting Bevy 0.11
```toml
[dependencies]
bevy_app = "0.11"
bevy_ecs = "0.11"
bevy_doryen = "0.3"
```

```rust
App::new()
    // Insert a `DoryenPluginSettings` resource to configure the plugin.
    .insert_resource(DoryenPluginSettings {
        // `app_options` lets you configure Doryen just as if you were
        // using Doryen without Bevy. The default is `AppOptions::default()`.
        app_options: AppOptions {
            show_cursor: true,
            resizable: true,
            ..AppOptions::default()
        },
        // Lets you configure which mouse buttons to listen for. The default
        // is left, middle and right click.
        mouse_button_listeners: vec![
            MouseButton::Left,
            MouseButton::Middle,
            MouseButton::Right,
        ],
        // Lets you configure how the application should behave when resized.
        // The default is `ResizeMode::Nothing`. See `ResizeMode`'s
        // documentation for more information.
        resize_mode: ResizeMode::Nothing
    })
    // Add the `DoryenPlugin` to Bevy.
    .add_plugins(DoryenPlugin)
    // Add your Bevy systems like usual. Excluding startup systems, which
    // only run once, these systems are run during Doryen's update phase;
    // i.e. 60 times per second.
    .add_systems(Startup, init)
    .add_systems(Update, input)
    // The `Render` schedules lets you add systems that should
    // be run during Doryen's render phase.
    .add_systems(Render, render)
    .run();
```

See the [examples](https://github.com/alexschrod/bevy_doryen/tree/master/examples)
for more usage scenarios.

## License

Licensed under either of

* Apache License, Version 2.0, ([LICENSE-APACHE](LICENSE-APACHE) or http://www.apache.org/licenses/LICENSE-2.0)
* MIT license ([LICENSE-MIT](LICENSE-MIT) or http://opensource.org/licenses/MIT)

at your option.

### Contribution

Unless you explicitly state otherwise, any contribution intentionally submitted
for inclusion in the work by you, as defined in the Apache-2.0 license, shall be
dual licensed as above, without any additional terms or conditions.
