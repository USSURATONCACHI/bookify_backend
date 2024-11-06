use stylist::yew::styled_component;
use stylist::Style;
use yew::prelude::*;

#[derive(Debug, Properties, PartialEq)]
pub struct PublicationsProps {}
const STYLE: &'static str = include_str!("publications.css");

#[styled_component(Publications)]
pub fn publications(_props: &PublicationsProps) -> Html {
    let stylesheet = Style::new(STYLE).unwrap();

    html! {
        <div class={stylesheet}>
            <h2>{"Publications"}</h2>
            <p>{"300 digital publications"}</p>
            <p>{"145 types of physical publications, 54412 books"}</p>
        </div>
    }
}
