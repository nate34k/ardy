use actix_web::{post, get, web, HttpResponse, Responder, Error, delete};
use rusqlite::{Connection, Result};

use crate::models::{ItemData, Hello};

// Handle GET request
#[get("/api/v1/hello")]
pub async fn hello_get() -> impl Responder {
    HttpResponse::Ok().body("Hello, World!")
}

// Handle POST request
#[post("/api/v1/hello")]
pub async fn hello_post(web::Json(hello_data): web::Json<Hello>) -> impl Responder {
    if hello_data.hello == "actix" {
        println!("Hello, actix!");
        HttpResponse::Ok().body("Hello, actix!")
    } else {
        println!("Invalid JSON body");
        HttpResponse::BadRequest().body("Invalid JSON body")
    }
}

// Handle Post request for adding trade data to database
#[post("/api/v1/trade")]
pub async fn trade_post(web::Json(item_data): web::Json<ItemData>) -> Result<impl Responder, Error> {
    println!("POST request received");

    let conn = Connection::open("db/ardy.db").map_err(|e| {
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

    println!("item_data: {:?}", item_data);

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

#[derive(serde::Deserialize)]
pub struct QueryParams {
    item_name: Option<String>,
}

// Handle GET request for getting trade data from database
// Takes optional query parameters: item_name
#[get("/api/v1/trade")]
pub async fn trade_get(query_params: web::Query<QueryParams>) -> Result<impl Responder, Error> {
    println!("GET request received");

    let conn = Connection::open("db/ardy.db").map_err(|e| {
        println!("Failed to open database: {}", e);
        HttpResponse::InternalServerError().body("Failed to open database")
    });

    let sql_query = if query_params.item_name.is_some() {
        format!("SELECT trades.id, items.name, trades.quantity, trades.total_price, trades.is_purchase, trades.timestamp FROM trades INNER JOIN items ON trades.item_id = items.id WHERE items.name = '{}'", query_params.item_name.as_ref().unwrap())
    } else {
        "SELECT trades.id, items.name, trades.quantity, trades.total_price, trades.is_purchase, trades.timestamp FROM trades INNER JOIN items ON trades.item_id = items.id".to_string()
    };

    let mut stmt = conn.as_ref().unwrap().prepare(
        &sql_query,
    ).map_err(|e| {
        println!("Failed to prepare statement: {}", e);
        HttpResponse::InternalServerError().body("Failed to prepare statement")
    }).unwrap();

    let rows = stmt.query_map(
        [],
        |row| {
            Ok(ItemData {
                id: row.get(0)?,
                item_name: row.get(1)?,
                quantity: row.get(2)?,
                total_price: row.get(3)?,
                is_purchase: row.get(4)?,
                timestamp: {
                    let timestamp: String = row.get(5)?;
                    let timestamp: i64 = timestamp.parse().unwrap();
                    let naive_datetime = chrono::NaiveDateTime::from_timestamp_opt(timestamp, 0).unwrap();
                    naive_datetime
                },
            })
        },
    ).map_err(|e| {
        println!("Failed to query map: {}", e);
        HttpResponse::InternalServerError().body("Failed to query map")
    }).unwrap();

    let mut item_data_vec = Vec::new();

    for row in rows {
        item_data_vec.push(row.unwrap());
    }

    Ok(HttpResponse::Ok().json(item_data_vec))
}

// Handle DELETE request for deleting trade data from database
#[derive(serde::Deserialize)]
pub struct DeleteQueryParams {
    id: i64,
}

#[delete("/api/v1/trade")]
pub async fn trade_delete(query_params: web::Query<DeleteQueryParams>) -> Result<impl Responder, Error> {
    println!("DELETE request received");

    let conn = Connection::open("db/ardy.db").map_err(|e| {
        println!("Failed to open database: {}", e);
        HttpResponse::InternalServerError().body("Failed to open database")
    });

    conn.unwrap().execute(
        "DELETE FROM trades WHERE id = ?1",
        &[&query_params.id],
    ).map_err(|e| {
        println!("Failed to delete trade data from trades table: {}", e);
        HttpResponse::InternalServerError().body("Failed to delete trade data from trades table")
    }).unwrap();

    Ok(HttpResponse::Ok().body("Trade data successfully deleted"))
}

struct ProfitLossData {
    profit_loss: i64,
}

// Handle GET request for profit/loss calculation
#[get("/api/v1/profit_loss")]
pub async fn profit_loss_get() -> Result<impl Responder, Error> {
    // Open database conn
    let conn = Connection::open("db/ardy.db").map_err(|e| {
        println!("Failed to open database: {}", e);
        HttpResponse::InternalServerError().body("Failed to open database")
    });

    // Get a list of all prices for each trade (quantity * total_price)
    // If is_purchase is 0, multiply by -1 to get the correct sign
    let mut stmt = conn.as_ref().unwrap().prepare(
        "SELECT trades.quantity * trades.total_price * (CASE WHEN trades.is_purchase = 0 THEN 1 ELSE -1 END) FROM trades",
    ).map_err(|e| {
        println!("Failed to prepare statement: {}", e);
        HttpResponse::InternalServerError().body("Failed to prepare statement")
    }).unwrap();

    let rows = stmt.query_map(
        [],
        |row| {
            Ok(
                ProfitLossData {
                    profit_loss: row.get(0)?,
                }
            )
        },
    ).map_err(|e| {
        println!("Failed to query map: {}", e);
        HttpResponse::InternalServerError().body("Failed to query map")
    }).unwrap();

    let mut profit_loss_vec = Vec::new();

    for row in rows {
        profit_loss_vec.push(row.unwrap().profit_loss);
    }

    // Return Sum of all prices
    Ok(HttpResponse::Ok().json(profit_loss_vec.iter().sum::<i64>()))
}
