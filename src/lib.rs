use seed::{*, prelude::*};

#[derive(Default)]
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
}


fn view(model: &Model) -> impl View<Msg> {
    let plural = if model.count == 1 {""} else {"s"};

    div![
        h1![ "The Grand Total" ],
        div![
            h3![ format!("{} click{} so far", model.count, plural) ],
            button![ simple_ev(Ev::Click, Msg::Increment), "+" ],
            button![ simple_ev(Ev::Click, Msg::Decrement), "-" ],
        ],
    ]
}


#[wasm_bindgen(start)]
pub fn render() {
    App::builder(update, view)
        .build_and_start();
}