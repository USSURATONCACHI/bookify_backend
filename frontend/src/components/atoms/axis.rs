use stylist::yew::styled_component;
use stylist::Style;
use yew::prelude::*;

use super::axis_item::AxisItem;

#[derive(Debug, Properties, PartialEq)]
pub struct AxisProps {
    pub shown_min: f64,
    pub shown_max: f64,

    #[prop_or_default]
    pub min: Option<f64>,

    #[prop_or_default]
    pub max: Option<f64>,

    #[prop_or_default]
    pub children: ChildrenWithProps<AxisItem>,
}

const STYLE: &'static str = include_str!("axis.css");

#[styled_component(Axis)]
pub fn axis(props: &AxisProps) -> Html {
    let stylesheet = Style::new(STYLE).unwrap();

    let shown_min = use_state(|| props.shown_min);
    let shown_max = use_state(|| props.shown_max);

    let mut shown_items = (&props.children)
        .iter()
        .filter(|item| item.props.value >= *shown_min)
        .filter(|item| item.props.value <= *shown_max)
        .collect::<Vec<_>>();
    shown_items.sort_by(|a, b| a.props.value.total_cmp(&b.props.value));
    let shown_items = shown_items;

    let html_items: Vec<_> = shown_items
        .into_iter()
        .map(|item| {
            let val = (item.props.value - *shown_min) / (*shown_max - *shown_min);
            (item, val)
        })
        .map(|(item, pos)| {
            let style_text = format!(
                "left: {}%; opacity: {};",
                pos * 100.0,
                1.0 - (pos - 0.5).abs() * 2.0
            );
            let style = Style::new(style_text.as_str()).unwrap();

            html! {
                <li class={style}>
                    {item}
                </li>
            }
        })
        .collect();

    let onwheel = Callback::from({
        let shown_max = shown_max.clone();
        move |e: WheelEvent| {
            let delta = e.delta_x() - e.delta_y();
            shown_max.set(*shown_max + delta / 10.0);
            gloo::console::log!("Scrolled! ", *shown_max);
        }
    });

    html! {
        <div class={stylesheet}>
            <ul class="axis" {onwheel}>
                { html_items }
            </ul>
        </div>
    }
}
