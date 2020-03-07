use seed::{prelude::*, *};
use shared::RandomResponse;
use seed::browser::service::fetch::FailReason;

#[derive(Default)]
struct Model {
    response: Option<seed::fetch::ResponseDataResult<RandomResponse>>,
}

#[derive(Clone)]
enum Msg {
    StartFetching,
    DataFetched(seed::fetch::ResponseDataResult<RandomResponse>),
}

async fn fetch_data() -> Result<Msg, Msg> {
    let url = "http://localhost:8090/random";
    Request::new(url).fetch_json_data(Msg::DataFetched).await
}

fn update(msg: Msg, model: &mut Model, orders: &mut impl Orders<Msg>) {
    match msg {
        Msg::StartFetching => {
            orders.perform_cmd(fetch_data());
        }
        Msg::DataFetched(Ok(res)) => {
            model.response = Some(Ok(res));
        }
        Msg::DataFetched(Err(e)) => {
            model.response = Some(Err(e));
        }
    }
}

fn view(model: &Model) -> impl View<Msg> {
    let result = match &model.response {
        Some(Ok(e)) => {
            div![format!("Random Number: {}", e.number)]
        }
        Some(Err(e)) => {
            div![format!("Error fetching: {:?}", e)]
        },
        None => {
            div!["Press the button to generate a random number."]
        }
    };
    div![
        h1!["Secure Random Number Generator"],
        result,
        div![
            button![simple_ev(Ev::Click, Msg::StartFetching), "Generate Random Number"],
        ],
    ]
}

#[wasm_bindgen(start)]
pub fn render() {
    App::builder(update, view).build_and_start();
}
