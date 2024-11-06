pub mod components;

use components::atoms::Axis;
use components::atoms::AxisItem;
use components::atoms::Sigma;
use components::atoms::SigmaType;
use components::organisms::Publications;

use yew::prelude::*;
use yew::virtual_dom::VChild;

#[function_component]
fn App() -> Html {
    let drippy = use_state(|| String::new());

    let items: Vec<VChild<AxisItem>> = (0..100)
        .into_iter()
        .map(|x| {
            html_nested! {
                <AxisItem value={x as f64} text={x.to_string()}/>
            }
        })
        .collect();

    html! {
        <>
            <Sigma kind={SigmaType::Orange} value={drippy}/>
            <Axis
                shown_min={5.0}
                shown_max={15.0}>
                {items}
            </Axis>
            <Publications/>
        </>
    }
}

fn main() {
    yew::Renderer::<App>::new().render();
}
