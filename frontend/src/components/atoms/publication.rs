use gloo::console::log;
use stylist::{yew::styled_component, Style};
use yew::prelude::*;

#[derive(Properties, PartialEq)]
pub struct PublicationProps {
    #[prop_or_default]
    pub uuid: Option<String>,

    pub name: String,

    #[prop_or_default]
    pub description: Option<String>,

    #[prop_or_default]
    pub cover_url: Option<String>,

    #[prop_or_default]
    pub links: Option<Vec<String>>,
}

const STYLE: &'static str = include_str!("publication.css");

#[styled_component(Publication)]
pub fn publication(props: &PublicationProps) -> Html {
    let stylesheet = Style::new(STYLE).unwrap();

    let descr: &str = props
        .description
        .as_ref()
        .map(|x| x.as_str())
        .unwrap_or("sigma");

    html! {
        <tr class={stylesheet}>
            <td>{&props.name}</td>
            <td>{descr}</td>
        </tr>
    }
}
