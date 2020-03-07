use seed::{
    browser::service::storage::{self, Storage},
    prelude::*,
    *,
};
use serde::{Deserialize, Serialize};

const STORAGE_KEY: &str = "lisbon-rust";

#[derive(Default, Deserialize, Serialize)]
struct Counter {
    count: i32,
}

struct Model {
    counter: Counter,
    local_storage: Storage,
}

#[derive(Clone)]
enum Msg {
    Increment,
    Decrement,
}

fn update(msg: Msg, model: &mut Model, _orders: &mut impl Orders<Msg>) {
    match msg {
        Msg::Increment => model.counter.count += 1,
        Msg::Decrement => model.counter.count -= 1,
    }
    storage::store_data(&model.local_storage, STORAGE_KEY, &model.counter);

}

fn view(model: &Model) -> impl View<Msg> {
    let plural = if model.counter.count == 1 { "" } else { "s" };

    div![
        h1!["The Grand Total"],
        div![
            h3![format!("{} click{} so far", model.counter.count, plural)],
            button![simple_ev(Ev::Click, Msg::Increment), "+"],
            button![simple_ev(Ev::Click, Msg::Decrement), "-"],
        ],
    ]
}

fn after_mount(_: Url, _: &mut impl Orders<Msg>) -> AfterMount<Model> {
    let local_storage = storage::get_storage().expect("get `LocalStorage`");
    let counter = storage::load_data(&local_storage, STORAGE_KEY).unwrap_or_default();

    AfterMount::new(Model {
        local_storage,
        counter
    })
}

#[wasm_bindgen(start)]
pub fn render() {
    App::builder(update, view)
        .after_mount(after_mount)
        .build_and_start();
}
