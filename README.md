# bevy_doryen
A [Bevy](https://bevyengine.org/) plugin that integrates the
[Doryen](https://github.com/jice-nospam/doryen-rs) roguelike library with Bevy.

[![Crates.io](https://img.shields.io/crates/v/bevy_doryen)](https://crates.io/crates/bevy_doryen)
[![Crates.io](https://img.shields.io/crates/l/bevy_doryen)](https://crates.io/crates/bevy_doryen)
[![Crates.io](https://img.shields.io/crates/d/bevy_doryen)](https://crates.io/crates/bevy_doryen)
[![Docs.io](https://docs.rs/bevy_doryen/badge.svg)](https://docs.rs/bevy_doryen)
[![GitHub](https://github.com/alexschrod/bevy_doryen/actions/workflows/rust.yml/badge.svg)](https://github.com/alexschrod/bevy_doryen)

## Usage
```toml
[dependencies]
bevy_doryen = "0.1"
```

```rust
App::build()
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
    .add_plugin(DoryenPlugin)
    // Add your Bevy systems like usual. Excluding startup systems, which
    // only run once, these systems are run during Doryen's update phase;
    // i.e. 60 times per second.
    .add_startup_system(init.system())
    .add_system(input.system())
    // The `RenderSystemExtensions` trait lets you add systems that should
    // be run during Doryen's render phase.
    .add_doryen_render_system(render.system())
.run();
```

See the examples for more usage instructions.

## License

Licensed under either of

* Apache License, Version 2.0, ([LICENSE-APACHE](LICENSE-APACHE) or http://www.apache.org/licenses/LICENSE-2.0)
* MIT license ([LICENSE-MIT](LICENSE-MIT) or http://opensource.org/licenses/MIT)

at your option.

### Contribution

Unless you explicitly state otherwise, any contribution intentionally submitted
for inclusion in the work by you, as defined in the Apache-2.0 license, shall be
dual licensed as above, without any additional terms or conditions.
