mod app;
mod game;
use app::App;

fn main() {
    yew::Renderer::<App>::new().render();
}
