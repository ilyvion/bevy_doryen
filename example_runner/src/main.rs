use serde::{Deserialize, Serialize};

mod examples {
    #[path = "../../../examples/alpha.rs"]
    pub mod alpha;
    #[path = "../../../examples/basic.rs"]
    pub mod basic;
    #[path = "../../../examples/blit.rs"]
    pub mod blit;
    #[path = "../../../examples/demo/main.rs"]
    pub mod demo;
    #[path = "../../../examples/exit.rs"]
    pub mod exit;
    #[path = "../../../examples/fonts.rs"]
    pub mod fonts;
    #[path = "../../../examples/image.rs"]
    pub mod image;
    #[path = "../../../examples/lowfps.rs"]
    pub mod lowfps;
    #[path = "../../../examples/perf.rs"]
    pub mod perf;
    #[path = "../../../examples/resize.rs"]
    pub mod resize;
    #[path = "../../../examples/subcell.rs"]
    pub mod subcell;
    #[path = "../../../examples/text_input.rs"]
    pub mod text_input;
    #[path = "../../../examples/unicode.rs"]
    pub mod unicode;
}

#[derive(Debug, Deserialize, PartialEq, Serialize)]
struct Query {
    example: String,
}

fn main() {
    #[cfg(target_arch = "wasm32")]
    console_error_panic_hook::set_once();

    let example;
    #[cfg(target_arch = "wasm32")]
    {
        use gloo_utils::document;
        let query_string = document()
            .location()
            .expect("document.location failed")
            .search()
            .expect("document.location.search failed");

        let query: Query = match query_string
            .strip_prefix('?')
            .ok_or(())
            .and_then(|query| serde_qs::from_str(query).map_err(|_| ()))
        {
            Ok(query) => query,
            Err(_) => {
                let _ = gloo_utils::window().alert_with_message(
                    "Page requires query string with `example=<desired example>`",
                );
                return;
            }
        };

        example = query.example;
    }
    #[cfg(not(target_arch = "wasm32"))]
    {
        example = std::env::args()
            .nth(1)
            .expect("Desired example must be passed in as an argument to this program");
    }

    match example.as_str() {
        "alpha" => examples::alpha::main(),
        "basic" => examples::basic::main(),
        "blit" => examples::blit::main(),
        "demo" => examples::demo::main(),
        "exit" => examples::exit::main(),
        "fonts" => examples::fonts::main(),
        "image" => examples::image::main(),
        "lowfps" => examples::lowfps::main(),
        "perf" => examples::perf::main(),
        "resize" => examples::resize::main(),
        "subcell" => examples::subcell::main(),
        "text_input" => examples::text_input::main(),
        "unicode" => examples::unicode::main(),
        example => report_missing_example(example),
    }
}

#[cfg(not(target_arch = "wasm32"))]
fn report_missing_example(example: &str) {
    println!("Example `{example}` was requested, but no such example exists");
}

#[cfg(target_arch = "wasm32")]
fn report_missing_example(example: &str) {
    let _ = gloo_utils::window().alert_with_message(&format!(
        "Example `{example}` was requested, but no such example exists"
    ));
}
