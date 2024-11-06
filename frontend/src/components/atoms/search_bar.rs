use stylist::{yew::styled_component, Style};
use yew::prelude::*;
use yew_icons::{Icon, IconId};

#[derive(Properties, PartialEq)]
pub struct SearchBarProps {
    #[prop_or_default]
    pub placeholder: Option<String>,
}

const STYLE: &'static str = include_str!("search_bar.css");

#[styled_component(SearchBar)]
pub fn search_bar(props: &SearchBarProps) -> Html {
    let stylesheet = Style::new(STYLE).unwrap();

    let descr = props.placeholder.clone().unwrap_or("Search...".to_string());

    html! {
        <div class={stylesheet}>
            <div class="searchbar">
                <input type="text" placeholder={descr}/>
                <Icon class="icon" icon_id={IconId::BootstrapSearch}/>
            </div>
        </div>
    }
}
