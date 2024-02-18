# bevy_doryen

A [Bevy](https://bevyengine.org/) plugin that integrates the
[Doryen](https://github.com/jice-nospam/doryen-rs) roguelike library with Bevy.

[![Crates.io](https://img.shields.io/crates/v/bevy_doryen)](https://crates.io/crates/bevy_doryen)
[![Docs.io](https://docs.rs/bevy_doryen/badge.svg)](https://docs.rs/bevy_doryen)
[![Docs master](https://img.shields.io/static/v1?label=docs&message=master&color=5479ab)](https://ilyvion.github.io/bevy_doryen/doc)
[![GitHub](https://github.com/ilyvion/bevy_doryen/actions/workflows/rust.yml/badge.svg)](https://github.com/ilyvion/bevy_doryen)
[![Following released Bevy versions](https://img.shields.io/badge/Bevy%20tracking-released%20version-lightblue)](https://bevyengine.org/learn/quick-start/plugin-development/#main-branch-tracking)
[![codecov](https://codecov.io/github/ilyvion/bevy_doryen/branch/master/graph/badge.svg?token=08P69DV34A)](https://codecov.io/github/ilyvion/bevy_doryen)

## Usage

### Targeting Bevy 0.13

```toml
[dependencies]
bevy_app = "0.13"
bevy_ecs = "0.13"
bevy_doryen = "0.5"
```

```rust
use bevy_doryen::prelude::*;

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

See the [examples](https://github.com/ilyvion/bevy_doryen/tree/master/examples)
for more usage scenarios, as well as live demos.

## Bevy Version Support

I intend to track the latest release version of Bevy. PRs supporting this are welcome!

| bevy   | bevy_doryen |
| ------ | ----------- |
| 0.13   | 0.5         |
| 0.12.1 | 0.4.1       |
| 0.11   | 0.3         |
| 0.5    | 0.2         |
| 0.4    | 0.1         |

## License

All code in this repository is dual-licensed under either:

-   Apache License, Version 2.0 ([LICENSE-APACHE](LICENSE-APACHE) or http://www.apache.org/licenses/LICENSE-2.0)
-   MIT license ([LICENSE-MIT](LICENSE-MIT) or http://opensource.org/licenses/MIT)

at your option. This means you can select the license you prefer.

### Contribution

Unless you explicitly state otherwise, any contribution intentionally submitted
for inclusion in the work by you, as defined in the Apache-2.0 license, shall be
dual licensed as above, without any additional terms or conditions.
