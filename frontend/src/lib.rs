use seed::{prelude::*, *};
use shared::RandomResponse;

#[derive(Default)]
struct Model {
    response: Option<RandomResponse>,
}

#[derive(Clone)]
enum Msg {
    StartFetching,
    DataFetched(seed::fetch::ResponseDataResult<RandomResponse>),
}

fn view(model: &Model) -> impl View<Msg> {
    let fetch_result = match &model.response {
        None => div!["Press the button to generate a random number."],
        Some(response) => div![format!("Random Number: {}", response.number)],
    };
    div![
        h1!["Secure Random Number Generator"],
        fetch_result,
        button![
            simple_ev(Ev::Click, Msg::StartFetching),
            "Generate Random Number"
        ]
    ]
}

fn update(msg: Msg, model: &mut Model, orders: &mut impl Orders<Msg>) {
    match msg {
        Msg::StartFetching => {
            orders.perform_cmd(fetch_data());
        }
        Msg::DataFetched(Ok(result)) => {
            model.response = Some(result);
        }
        Msg::DataFetched(Err(_)) => {
            model.response = None;
        }
    }
}

async fn fetch_data() -> Result<Msg, Msg> {
    let url = "http://localhost:8090/random";
    Request::new(url).fetch_json_data(Msg::DataFetched).await
}

#[wasm_bindgen(start)]
pub fn render() {
    App::builder(update, view).build_and_start();
}
