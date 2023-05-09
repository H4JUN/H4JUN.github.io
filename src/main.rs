mod app;
mod components;
mod donut;
mod mdblogs;

use app::App;

pub fn main() {
    yew::Renderer::<App>::new().render();
}
