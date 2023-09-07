mod routes;
mod models;
mod db;

use actix_web::{App, HttpServer};
use actix_cors::Cors;
use routes::{hello_get, hello_post, trade_post, trade_get, profit_loss_get};

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    // Initialize database
    db::init_db().expect("Failed to initialize database");

    HttpServer::new(|| {
        // Enable CORS *UNSAFE FOR PRODUCTION*
        let cors = Cors::default()
            .allow_any_origin()
            .allow_any_method()
            .allow_any_header()
            .max_age(3600);

        App::new()
            .wrap(cors)
            .service(hello_get)
            .service(hello_post)
            .service(trade_post)
            .service(trade_get)
            .service(profit_loss_get)
    })
    .bind("127.0.0.1:5000")?
    .run()
    .await
}
