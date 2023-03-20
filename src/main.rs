use sureforlaunch_api::run;

// #[actix_web::main]
#[tokio::main]
async fn main() -> std::io::Result<()> {
    // Bubble up the io::Error if we failed to bind the address
    // Othersise call .awit on our Server
    run()?.await
}
