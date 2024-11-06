use stylist::yew::styled_component;
use stylist::Style;
use yew::prelude::*;
use yew_icons::{Icon, IconId};

#[derive(Debug, Properties, PartialEq)]
pub struct PublicationsProps {}
const STYLE: &'static str = include_str!("publications.css");

#[styled_component(Publications)]
pub fn publications(_props: &PublicationsProps) -> Html {
    let stylesheet = Style::new(STYLE).unwrap();

    html! {
        <>
            <div class={stylesheet}>
                <div class="bootstrap-wrapper">
                    <div class="container">
                        <div class="row">
                            <div class="col-12">
                                <h2>{"Публикации"}</h2>
                                <p>{"300 digital publications"}</p>
                                <p>{"145 types of physical publications, 54412 books"}</p>

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
                        </div>
                    </div>
                </div>

            </div>
        </>
    }
}
