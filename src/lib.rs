use seed::{prelude::*, *};

#[derive(Default)]
struct Model {
    count: i32,
}

#[derive(Clone)]
enum Msg {
    Increment,
    Decrement,
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

fn update(msg: Msg, model: &mut Model, _orders: &mut impl Orders<Msg>) {
    match msg {
        Msg::Increment => model.count += 1,
        Msg::Decrement => model.count -= 1,
    }
}

#[wasm_bindgen(start)]
pub fn render() {
    App::builder(update, view).build_and_start();
}
