mod home;
mod events;

use actix_web::{web, App, HttpServer};
use home::home;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| {
        App::new()
            .route("/", web::get().to(home))
    })
    .bind("127.0.0.1:8080")?
    .run()
    .await
}