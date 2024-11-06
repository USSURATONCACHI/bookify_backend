use stylist::yew::styled_component;
use stylist::Style;
use yew::prelude::*;

use crate::components::organisms;

#[derive(Debug, Properties, PartialEq)]
pub struct MainProps {}
const STYLE: &'static str = include_str!("main.css");

#[styled_component(Main)]
pub fn main(_props: &MainProps) -> Html {
    let stylesheet = Style::new(STYLE).unwrap();

    html! {
        <>
            <div class={stylesheet}>
                <div class="bootstrap-wrapper">
                    <div class="container">
                        <div class="row">
                            <div class="col-12">
                                <organisms::Header/>
                            </div>
                        </div>
                        <div class="row">
                            <div class="col-12">
                                <organisms::Publications/>
                            </div>
                        </div>
                    </div>
                </div>

            </div>
        </>
    }
}
