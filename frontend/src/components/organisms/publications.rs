use stylist::yew::styled_component;
use stylist::Style;
use yew::prelude::*;
use yew_icons::{Icon, IconId};

use crate::components::atoms;

#[derive(Debug, Properties, PartialEq)]
pub struct PublicationsProps {}
const STYLE: &'static str = include_str!("publications.css");

#[styled_component(Publications)]
pub fn publications(_props: &PublicationsProps) -> Html {
    let stylesheet = Style::new(STYLE).unwrap();

    html! {
        <div class={stylesheet}>
            <h3>{"Публикации"}</h3>
            <atoms::SearchBar placeholder={"Поиск публикаций..."}/>
            <table>
                <tr class="header-row">
                    <th class="checkbox-column">{"⌷"}</th>
                    <th>{"Название"}</th>
                    <th>{"Описание"}</th>
                </tr>
                <atoms::Publication name="Alpha"   description="Alpha of the one"/>
                <atoms::Publication name="Beta"    description="Beta of state"/>
                <atoms::Publication name="Gamma"   description="Gamma of ski-buddies"/>
                <atoms::Publication name="Delta"   description="Delta of oh no..."/>
                <atoms::Publication name="Epsilon" description="Epsilon of those who know..."/>
            </table>
        </div>
    }
}
