use stylist::yew::styled_component;
use stylist::Style;
use yew::prelude::*;
use yew_icons::{Icon, IconId};

#[derive(Debug, Properties, PartialEq)]
pub struct ActionsProps {}
const STYLE: &'static str = include_str!("actions.css");

#[styled_component(Actions)]
pub fn actions(_props: &ActionsProps) -> Html {
    let stylesheet = Style::new(STYLE).unwrap();

    html! {
        <>
            <div class={stylesheet}>
                <h3>{"Действия"}</h3>
                <div class="actions-list">
                    <button class="item">
                        <Icon class="icon" icon_id={IconId::BootstrapPlusLg}/>
                        <div class="gradient"></div>
                        <p>{"Добавить публикацию"}</p>
                    </button>
                    <button class="item">
                        <Icon class="icon" icon_id={IconId::LucideImport}/>
                        <div class="gradient"></div>
                        <p>{"Импорт из Лань"}</p>
                    </button>
                    <button class="item">
                        <Icon class="icon" icon_id={IconId::FontAwesomeSolidFileExport}/>
                        <div class="gradient"></div>
                        <p>{"Экспорт"}</p>
                    </button>
                </div>
            </div>
        </>
    }
}
