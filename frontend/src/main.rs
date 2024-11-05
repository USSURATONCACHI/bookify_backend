use gloo::console::log;
use gloo::dialogs::alert;
use serde::{Deserialize, Serialize};
use stylist::style;
use stylist::yew::styled_component;
use wasm_bindgen::JsCast;
use web_sys::HtmlInputElement;
use yew::prelude::*;

#[derive(Serialize, Deserialize)]
struct MyObject {
    name: String,
    surname: String,
}

#[derive(PartialEq)]
enum SigmaType {
    Red,
    Blue,
    Orange,
}

#[derive(Properties, PartialEq)]
struct SigmaProps {
    kind: SigmaType,
    value: UseStateHandle<String>,
}

#[styled_component(Sigma)]
fn sigma(props: &SigmaProps) -> Html {
    let stylesheet = style!(
        r#"
            li {
                display: block;
                text-align: center;
                font-size: 48px;
                font-weight: 900;
            }
            
            span {
                font-weight: 900;
            }

            .orange {
                color: orange;
            }

            .blue {
                color: blue;
            }

            .red {
                color: red;
            }

            input {
                background-color: white;
            }
        "#
    )
    .unwrap();

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

#[function_component]
fn App() -> Html {
    let obj = MyObject {
        name: "Paul".to_owned(),
        surname: "Owen".to_owned(),
    };
    log!(serde_json::to_string_pretty(&obj).unwrap());

    let counter = use_state(|| 0);
    let onclick = {
        let counter = counter.clone();
        move |_| {
            let value = *counter + 1;
            counter.set(value);
        }
    };

    let list = vec!["aaa", "bbb", "ccc", "ddd"];
    let list: Vec<Html> = list
        .into_iter()
        .map(|x| html! {x})
        .map(|x| html! { <li>{x}</li> })
        .collect();

    let drippy = use_state(|| String::new());

    html! {
        <>
            <p>{ "Skibidi sigma (" }{&*drippy}{ ")" }</p>
            <button {onclick}>{ "+1" }</button>
            <p>{ *counter }</p>

            if *counter % 2 == 0 {
                <p>{"A"}</p>
            } else {
                <p>{"B"}</p>
            }

            <ul>
                {list}
            </ul>

            <Sigma kind={SigmaType::Orange} value={drippy}/>
        </>
    }
}

fn main() {
    yew::Renderer::<App>::new().render();
}
