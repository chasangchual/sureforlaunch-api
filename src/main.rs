use actix_web::{get, web, App, HttpServer, Responder, HttpResponse};

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

#[actix_web::main] // or #[tokio::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| {
        App::new()
            .service(
                web::scope("/api/v1")
                    .service(greet)
                    .service(ping)
            )
            .route("/health_check", web::get().to(health_check))
    })
    .bind(("127.0.0.1", 9090))?
    .run()
    .await
}

#[cfg(test)]
mod tests {
    use crate::health_check;

    #[tokio::test]
    async fn health_check_succeeds() {
        let response = health_check().await;
        assert!(response.status().is_success())
    }
}