use actix_web::{get, post, web, App, HttpResponse, HttpServer, Responder};
use std::sync::Mutex;

struct AppState {
    counter: Mutex<i32>,
}

#[get("/")]
async fn index(data: web::Data<AppState>) -> impl Responder {
    let mut counter = data.counter.lock().unwrap();
    *counter += 1;

    HttpResponse::Ok().body(format!("Counted: {:?}", counter))
}

#[post("/token")]
async fn token() -> impl Responder {
    HttpResponse::Ok().body(format!("{:?}", uuid::Uuid::new_v4()))
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let addr = "127.0.0.1:8080";
    println!("Listening on: {:?}", addr);

    let counter = web::Data::new(AppState {
        counter: Mutex::new(0),
    });

    HttpServer::new(move || {
        App::new()
            // Note: app_data is singular, data is per request
            .app_data(counter.clone())
            .service(index)
            .service(token)
    })
    .bind(addr)?
    .run()
    .await
}
