use std::fmt;

use doryen_rs::AppOptions;

// Because `AppOptions` doesn't support `Debug`, but I would like to, I'm just
// going to do it myself.
pub(crate) struct DebugAppOptions<'a>(pub(crate) &'a AppOptions);
impl fmt::Debug for DebugAppOptions<'_> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let AppOptions {
            console_width,
            console_height,
            screen_width,
            screen_height,
            window_title,
            font_path,
            vsync,
            fullscreen,
            show_cursor,
            resizable,
            intercept_close_request,
            max_fps,
            ..
        } = self.0;
        f.debug_struct("AppOptions")
            .field("console_width", console_width)
            .field("console_height", console_height)
            .field("screen_width", screen_width)
            .field("screen_height", screen_height)
            .field("window_title", window_title)
            .field("font_path", font_path)
            .field("vsync", vsync)
            .field("fullscreen", fullscreen)
            .field("show_cursor", show_cursor)
            .field("resizable", resizable)
            .field("intercept_close_request", intercept_close_request)
            .field("max_fps", max_fps)
            .finish()
    }
}
