mod routes;
mod models;
mod db;

use actix_web::{App, HttpServer};
use routes::{hello_get, hello_post, item_post};

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    // Initialize database
    db::init_db().expect("Failed to initialize database");

    HttpServer::new(|| {
        App::new()
            .service(hello_get)
            .service(hello_post)
            .service(item_post)
    })
    .bind("127.0.0.1:8080")?
    .run()
    .await
}
