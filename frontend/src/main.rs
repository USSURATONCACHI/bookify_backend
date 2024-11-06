pub mod components;

use components::pages;
use yew::prelude::*;

#[function_component]
fn App() -> Html {
    html! {
        <>
            <pages::Main/>
        </>
    }
}

fn main() {
    yew::Renderer::<App>::new().render();
}
