#[allow(unused_imports)]
use actix_web::{guard, web, App, HttpResponse, HttpServer, Responder};

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| {
        App::new().service(
            web::scope("/api")
                // .guard(guard::Header("Header", "zix-agent"))
                .route("/hello", web::to(hello))
                .route("/echo", web::to(echo)),
        )
    })
    .workers(5)
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}

async fn hello() -> impl Responder {
    HttpResponse::Ok().body("Hello, World!")
}

async fn echo(req_body: String) -> impl Responder {
    HttpResponse::Ok().body(req_body)
}
