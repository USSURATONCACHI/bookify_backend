use gloo::console::log;
use stylist::{yew::styled_component, Style};
use wasm_bindgen::JsCast;
use web_sys::HtmlInputElement;
use yew::prelude::*;

#[derive(PartialEq)]
pub enum SigmaType {
    Red,
    Blue,
    Orange,
}

#[derive(Properties, PartialEq)]
pub struct SigmaProps {
    pub kind: SigmaType,
    pub value: UseStateHandle<String>,
}

const STYLE: &'static str = include_str!("sigma.css");

#[styled_component(Sigma)]
pub fn sigma(props: &SigmaProps) -> Html {
    let stylesheet = Style::new(STYLE).unwrap();

    if props.value.len() == 0 {
        props.value.set("Drippy".to_owned());
    }

    let onchange = Callback::from({
        let state_value = props.value.clone();

        move |event: Event| {
            let value = event
                .target()
                .unwrap()
                .unchecked_into::<HtmlInputElement>()
                .value();
            log!(&value);
            state_value.set(value);
        }
    });

    html! {
        <ul class={stylesheet}>
            <li>{"Line A"}</li>
            <li>
                {"I like my cheese "}
                <span class={
                    match props.kind {
                        SigmaType::Orange => "orange",
                        SigmaType::Red => "red",
                        SigmaType::Blue => "blue",
                    }
                }>
                    {&*props.value}
                </span>
                {", bruh."}
            </li>
            <li>{"Line C"}</li>
            <li><input type="text" name="sigma" onchange={onchange} value={(*props.value).clone()}/></li>
        </ul>
    }
}
