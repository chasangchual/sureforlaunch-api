use actix_web::{get, web, App, HttpServer, Responder, HttpResponse};
use actix_web::dev::Server;

#[get("/hello/{name}")]
async fn greet(name: web::Path<String>) -> impl Responder {
    HttpResponse::Ok().body(format!{"Hello {name}!!"})
}

#[get("/ping")]
async fn ping() -> impl Responder {
    HttpResponse::Ok().body("pong")
}

async fn health_check() -> HttpResponse {
    HttpResponse::Ok().body("hello")
}

pub fn run() -> Result<Server, std::io::Error> {
    let server = HttpServer::new(|| {
        App::new()
            .service(
                web::scope("/api/v1")
                    .service(greet)
                    .service(ping)
            )
            .route("/health_check", web::get().to(health_check))
    })
        .bind(("127.0.0.1", 9090))?
        .run();

      Ok(server)
}