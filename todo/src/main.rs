//use active_webとは、active_web::prelude::*と同じ意味
use actix_web::{get, post, web, App, HttpResponse, HttpServer, Responder};

// https://actix.rs/docs/getting-started/
//　上記の基本

// GET /hello
#[get("/")]
async fn hello() -> impl Responder {
    HttpResponse::Ok().body("Hello world!")
}

// POST /echo
#[post("/echo")]
async fn echo(req_body: String) -> impl Responder {
    HttpResponse::Ok().body(req_body)
}

//
async fn manual_hello() -> impl Responder {
    HttpResponse::Ok().body("Hey there!")
}

//#[]とは、マクロを表す
#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| {
        App::new()
            .service(hello)
            .service(echo)
            .route("/hey", web::get().to(manual_hello))
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}