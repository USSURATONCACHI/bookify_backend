use yew::prelude::*;
use gloo_net::http::Request;

struct Model {
    publications: Vec<Publication>,
}

struct Publication {
    uuid: String,
    title: String,
}

enum Msg {
    FetchData,
    SetData(Vec<Publication>),
}



fn main() {
    println!("Hello, world!");
}
