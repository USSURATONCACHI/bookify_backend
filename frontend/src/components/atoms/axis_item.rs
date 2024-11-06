use serde::{Deserialize, Serialize};
use yew::prelude::*;

#[derive(Debug, Serialize, Deserialize, Properties, PartialEq)]
pub struct AxisItemProps {
    #[prop_or_default]
    pub id: Option<usize>,

    pub value: f64,
    pub text: String,
}

#[yew::function_component(AxisItem)]
pub fn axis(props: &AxisItemProps) -> Html {
    html! {
        <span>{ &props.text }</span>
    }
}
