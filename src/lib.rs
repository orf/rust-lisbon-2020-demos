use seed::{browser::service::storage, prelude::*, *};
use serde::{Deserialize, Serialize};

const STORAGE_KEY: &str = "lisbon-rust";

#[derive(Default, Deserialize, Serialize)]
struct Model {
    count: i32,
}

#[derive(Clone)]
enum Msg {
    Increment,
    Decrement,
}

fn update(msg: Msg, model: &mut Model, _orders: &mut impl Orders<Msg>) {
    match msg {
        Msg::Increment => model.count += 1,
        Msg::Decrement => model.count -= 1,
    }
    let storage = storage::get_storage().expect("get localstorage");
    storage::store_data(&storage, STORAGE_KEY, &model);
}

fn view(model: &Model) -> impl View<Msg> {
    div![
        h1!["The Grand Total"],
        div![
            h3![format!("Click count: {}", model.count)],
            button![simple_ev(Ev::Click, Msg::Increment), "+"],
            button![simple_ev(Ev::Click, Msg::Decrement), "-"],
        ],
    ]
}

fn after_mount(_: Url, _: &mut impl Orders<Msg>) -> AfterMount<Model> {
    let local_storage = storage::get_storage().expect("get `LocalStorage`");
    let counter = storage::load_data(&local_storage, STORAGE_KEY).unwrap_or_default();

    AfterMount::new(counter)
}

#[wasm_bindgen(start)]
pub fn render() {
    App::builder(update, view)
        .after_mount(after_mount)
        .build_and_start();
}
