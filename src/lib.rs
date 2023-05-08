pub mod app;
pub mod components;
pub mod donut;
pub mod mdblogs;

use wasm_bindgen::prelude::*;
use app::App;

// When the `wee_alloc` feature is enabled, use `wee_alloc` as the global
// allocator.
#[cfg(feature = "wee_alloc")]
#[global_allocator]
static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;

// #[wasm_bindgen(module = "/www/index.js")]
// extern "C" {
//     fn highlight();
// }

#[wasm_bindgen]
pub fn run_app() -> Result<(), JsValue> {
    wasm_logger::init(wasm_logger::Config::default());
    yew::Renderer::<App>::new().render();
    Ok(())
}

