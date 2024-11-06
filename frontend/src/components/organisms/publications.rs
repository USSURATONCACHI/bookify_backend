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
                    <th>{"Название"}</th>
                    <th>{"Описание"}</th>
                </tr>
                <atoms::Publication name="Sigma"             description="The one"/>
                <atoms::Publication name="Ohio"              description="State"/>
                <atoms::Publication name="Rizzler"           description="Skibidi"/>
                <atoms::Publication name="Still Water"       description="Oh no..."/>
                <atoms::Publication name="Mango Mango Mango" description="Those who know..."/>
            </table>
        </div>
    }
}
