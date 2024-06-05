use yew::prelude::*;
mod app;
use crate::app::*;
fn main() {
    yew::Renderer::<App>::new().render();
}
