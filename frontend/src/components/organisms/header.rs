use stylist::yew::styled_component;
use stylist::Style;
use yew::prelude::*;

#[derive(Debug, Properties, PartialEq)]
pub struct HeaderProps {}
const STYLE: &'static str = include_str!("header.css");

#[styled_component(Header)]
pub fn header(_props: &HeaderProps) -> Html {
    let stylesheet = Style::new(STYLE).unwrap();

    html! {
        <div class={stylesheet}>
            <div class="header">
                <h2>{"Цифровые публикации"}</h2>
            </div>
        </div>
    }
}
