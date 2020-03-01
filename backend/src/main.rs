use actix_web::{get, web, App, HttpResponse, HttpServer};
use rand::prelude::*;
use shared::RandomResponse;

#[get("/random")]
async fn index(_info: web::Path<()>) -> HttpResponse {
    let mut rng = rand::thread_rng();
    let number = rng.gen();
    HttpResponse::Ok()
        .header("Access-Control-Allow-Origin", "*")
        .json(RandomResponse { number })
}

#[actix_rt::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| App::new().service(index))
        .bind("127.0.0.1:8090")?
        .run()
        .await
}
