# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.1.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [Unreleased]

## [0.6.0]

### Changed 🔧

-   Update to Bevy 0.14

## [0.5.0]

### Changed 🔧

-   Update to Bevy 0.13

## [0.4.1]

### Added ✨

-   Add Bevy version support table to README
-   Add `lowfps` example
-   Add a prelude
-   Add `demo` example
-   Add "Go fullscreen" button to example page

### Changed 🔧

-   Remove `--verbose` from `cargo publish` in Publish & Release workflow
-   Add custom `Debug` implementation for `doryen_rs::AppOptions`
-   Update to Bevy 0.12.1

## [0.3.0]

### Added ✨

-   Added constructor function to `SetFontPath`
-   Added new event, `Capture`, to support the new Doryen feature
-   `example_runner` workspace package for running examples on the web using `trunk`

### Changed 🔧

-   Update to Rust 2021 edition
-   Update to Bevy 0.11.0
-   Update to Doryen 1.3.0
-   Move away from custom "render system" and instead use custom Bevy schedules to facilitate update vs render systems. I.e. most previous uses of `add_doryen_render_system(foo.system())` should be replaced with `add_systems(Render, foo)`. See the documentation for `MainRender` for more details.
-   Cleaned up code from a few new lints
-   Process `AppExit` events before others; no point doing anything else if we're about to quit
-   Have the `swap_console` (internal implementation detail) ask for a 0,0 console instead of a 1,1 console, which avoids any associated allocations.
-   Minor adjustment of documentation for `SetFontPath`
-   Update `basic` example to match latest Doryen code

## [0.2.0] - 2021-04-07

### Added ✨

-   Instructions for targeting various versions

### Changed 🔧

-   Update to Bevy 0.5
-   Use `EventReader`/`EventWriter` instead of `ResMut<Events<T>>`
-   Improved `blit` example

## [0.1.1] - 2021-03-12

### Added ✨

-   Build badge to README
-   Repository URL and README to Cargo.toml

### Changed 🔧

-   Switch to nightly toolchain for tarpaulin

### Removed

-   Mac OS build from CI (build is broken on Mac)

## [0.1.0] - 2021-03-12

### Added ✨

-   First implementation of the plugin. Compatible with Bevy 0.4.0.

[Unreleased]: https://github.com/ilyvion/bevy_doryen/compare/v0.6.0...HEAD
[0.6.0]: https://github.com/ilyvion/bevy_doryen/compare/v0.5.0...v0.6.0
[0.5.0]: https://github.com/ilyvion/bevy_doryen/compare/v0.4.1...v0.5.0
[0.4.1]: https://github.com/ilyvion/bevy_doryen/compare/v0.3.0...v0.4.1
[0.3.0]: https://github.com/ilyvion/bevy_doryen/compare/v0.2.0...v0.3.0
[0.2.0]: https://github.com/ilyvion/bevy_doryen/compare/v0.1.1...v0.2.0
[0.1.1]: https://github.com/ilyvion/bevy_doryen/compare/v0.1.0...v0.1.1
[0.1.0]: https://github.com/ilyvion/bevy_doryen/releases/tag/v0.1.0
