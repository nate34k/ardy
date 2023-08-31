use actix_web::{post, get, web, HttpResponse, Responder, Error};
use rusqlite::{Connection, Result};

use crate::models::{ItemData, Hello};

// Handle GET request
#[get("/api/hello")]
pub async fn hello_get() -> impl Responder {
    HttpResponse::Ok().body("Hello, World!")
}

// Handle POST request
#[post("/api/hello")]
pub async fn hello_post(web::Json(hello_data): web::Json<Hello>) -> impl Responder {
    if hello_data.hello == "actix" {
        println!("Hello, actix!");
        HttpResponse::Ok().body("Hello, actix!")
    } else {
        println!("Invalid JSON body");
        HttpResponse::BadRequest().body("Invalid JSON body")
    }
}

// ... (rest of the code)

#[post("/api/item")]
pub async fn item_post(web::Json(item_data): web::Json<ItemData>) -> Result<impl Responder, Error> {
    let conn = Connection::open("merchant.db").map_err(|e| {
        println!("Failed to open database: {}", e);
        HttpResponse::InternalServerError().body("Failed to open database")
    });

    // Insert item_name into items table and get its id
    conn.as_ref().unwrap().execute(
        "INSERT OR IGNORE INTO items (name) VALUES (?1)",
        &[&item_data.item_name],
    ).map_err(|e| {
        println!("Failed to insert item name into items table: {}", e);
        HttpResponse::InternalServerError().body("Failed to insert item name into items table")
    }).unwrap();

    let item_id: i64 = conn.as_ref().unwrap().query_row(
        "SELECT id FROM items WHERE name = ?1",
        &[&item_data.item_name],
        |row| row.get(0),
    ).map_err(|e| {
        println!("Failed to get item id from items table: {}", e);
        HttpResponse::InternalServerError().body("Failed to get item id from items table")
    }).unwrap();

    // Convert boolean to i64 (0 or 1)
    let is_purchase_i64 = if item_data.is_purchase { 1 } else { 0 };

    let naive_datetime = &item_data.timestamp.timestamp();

    // Insert the trade data into trades table
    conn.unwrap().execute(
        "INSERT INTO trades (item_id, quantity, total_price, is_purchase, timestamp) VALUES (?1, ?2, ?3, ?4, ?5)",
        &[&item_id, &item_data.quantity, &item_data.total_price, &is_purchase_i64, &naive_datetime],
    ).map_err(|e| {
        println!("Failed to insert trade data into trades table: {}", e);
        HttpResponse::InternalServerError().body("Failed to insert trade data into trades table")
    }).unwrap();

    Ok(HttpResponse::Ok().body("Trade data successfully saved"))
}
