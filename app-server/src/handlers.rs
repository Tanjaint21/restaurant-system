use crate::models::{OrderResponse, OrderItem, OrderRequestBody, Table, Menu, MenuResponse, TableResponse, OrderItemResponse};
use rusqlite::Connection;
use warp;
use rand::Rng;
use rusqlite::params;
use serde_json::json;


// Table Handlers

/// List All Tables
pub async fn list_table_handler(conn: Connection)-> Result<impl warp::Reply, warp::Rejection>{
    match Table::list(&conn) {
        Ok(tables) => {
            Ok(warp::reply::with_status(
                warp::reply::json(&tables),
                warp::http::StatusCode::OK
            ))
        }
        Err(_err) => {
            Ok(warp::reply::with_status(
                warp::reply::json::<Vec<TableResponse>>(&vec![]),
                warp::http::StatusCode::INTERNAL_SERVER_ERROR
            ))
        }
    }
}
/// Create a new Table
pub async fn create_table_handler(conn: Connection, data: Table) -> Result<impl warp::Reply, warp::Rejection> {
    match Table::get_existing_table_id(&conn, &data) {
    Ok(Some(table_id))=>{
        Ok(warp::reply::with_status(
            warp::reply::json(&json!({ "id": table_id })),
            warp::http::StatusCode::CREATED,
        ))
    }
    Ok(None)=>{
        match Table::create(&conn, &data) {
            Ok(table_id) => {
                Ok(warp::reply::with_status(
                    warp::reply::json(&json!({ "id": table_id })),
                    warp::http::StatusCode::CREATED,
                ))
            }
            Err(_err) => {
                // Respond with an error
                Ok(warp::reply::with_status(
                    warp::reply::json(&json!({"error":"Error creating table"})),
                    warp::http::StatusCode::INTERNAL_SERVER_ERROR,
                ))
            }
        }
    }
    Err(_err) => {
        // Respond with an error
        Ok(warp::reply::with_status(
            warp::reply::json(&json!({"error":"Error creating table"})),
            warp::http::StatusCode::INTERNAL_SERVER_ERROR,
        ))
    }
}
    
}

// Menu Handler

/// List All Menus
pub async fn list_menu_handler(conn: Connection)-> Result<impl warp::Reply, warp::Rejection>{
    match Menu::list(&conn) {
        Ok(menus) => {
            Ok(warp::reply::with_status(
                warp::reply::json(&menus),
                warp::http::StatusCode::OK,
            ))
        }
        Err(_err) => {
            Ok(warp::reply::with_status(
                warp::reply::json::<Vec<MenuResponse>>(&vec![]),
                warp::http::StatusCode::INTERNAL_SERVER_ERROR,
            )
            )
        }
    }
}
// Create a new Menu
pub async fn create_menu_handler(conn: Connection, data: Menu) -> Result<impl warp::Reply, warp::Rejection> {
    match Menu::get_existing_menu_id(&conn, &data) {
        Ok(Some(menu_id))=>{
            Ok(warp::reply::with_status(
                warp::reply::json(&json!({ "id": menu_id })),
                warp::http::StatusCode::CREATED,
            ))
        }
        Ok(None)=>{
            match Menu::create(&conn, &data) {
                Ok(menu_id) => {
                    Ok(warp::reply::with_status(
                        warp::reply::json(&json!({ "id": menu_id })),
                        warp::http::StatusCode::CREATED,
                    ))
                }
                Err(_err) => {
                    // Respond with an error
                    Ok(warp::reply::with_status(
                        warp::reply::json(&json!({ "error": "Error creating Menu" })),
                        warp::http::StatusCode::INTERNAL_SERVER_ERROR,
                    ))
                }
            }
        }
        Err(_err) => {
            // Respond with an error
            Ok(warp::reply::with_status(
                warp::reply::json(&json!({ "error": "Error creating Menu" })),
                warp::http::StatusCode::INTERNAL_SERVER_ERROR,
            ))
        }
}
}



// Order Handlers

/// Create a new order
pub async fn create_order_handler(conn: Connection, req_body: OrderRequestBody) -> Result<impl warp::Reply, warp::Rejection> {
    let table_id = req_body.table_id;
    let menu_ids = req_body.menu_ids;
    if menu_ids.len() == 0{
        return Ok(warp::reply::with_status(
            warp::reply::json(&json!({"error":"Please Add Items"})),
            warp::http::StatusCode::BAD_REQUEST,
        ));
    }
    // Check if there is an existing order with status 0 (running order) for the given table_id
    match OrderResponse::get_existing_order_id(&conn, table_id) {
        Ok(Some(order_id)) => {
            // Order exists for the given table_id, update the order items
            for menu_id in menu_ids {
                // Generate a random cooking time
                let cooking_time = rand::thread_rng().gen_range(5..=15);
                match OrderItem::get_existing_order_item_id(&conn, order_id, menu_id) {
                    Ok(Some(order_item_id)) => {
                         // Order item does exist, update quantity
                         match OrderItem::add_quantity_of_existing_order_item(&conn, order_item_id){
                            Ok(_)=>{
                                continue;
                            },
                            Err(_)=>{
                                return Ok(warp::reply::with_status(
                                    warp::reply::json(&json!({"error":"Error updating order Item"})),
                                    warp::http::StatusCode::INTERNAL_SERVER_ERROR,
                                ));
                            }
                         }
                    }
                    Ok(None) => {
                        // Order item does not exist, create a new order item
                        match OrderItem::create(&conn, order_id, menu_id, cooking_time) {
                            Ok(_) => {
                                // Continue to the next menu_id
                                continue;
                            }
                            Err(_err) => {
                                // Return an error response
                                eprintln!("{}",_err);
                                return Ok(warp::reply::with_status(
                                    warp::reply::json(&json!({"error":"Error creating order Item"})),
                                    warp::http::StatusCode::INTERNAL_SERVER_ERROR,
                                ));
                            }
                        }
                    }
                    Err(_err) => {
                        // Return an error response
                        return Ok(warp::reply::with_status(
                            warp::reply::json(&json!({"error":"Error creating for existing order Item"})),
                            warp::http::StatusCode::INTERNAL_SERVER_ERROR,
                        ));
                    }
                }
            }

            // If you reach this point, it means all order items were successfully handled
            Ok(warp::reply::with_status(
                warp::reply::json(&json!({"success":"All order items updated successfully"})),
                warp::http::StatusCode::OK,
            ))
        }
        Ok(None) => {
            // No running order exists for the given table_id, create a new order and order items
            match OrderResponse::create(&conn, table_id) {
                Ok(last_inserted_id) => {
                    for menu_id in menu_ids {
                        // Generate a random cooking time
                        let cooking_time = rand::thread_rng().gen_range(5..=15);
                        match OrderItem::create(&conn, last_inserted_id, menu_id, cooking_time) {
                            Ok(_) => {
                                // Continue to the next menu_id
                                continue;
                            }
                            Err(_err) => {
                                // Return an error response
                                eprintln!("{}",_err);
                                return Ok(warp::reply::with_status(
                                    warp::reply::json(&json!({"error":"Error creating order Item"})),
                                    warp::http::StatusCode::INTERNAL_SERVER_ERROR,
                                ));
                            }
                        }
                    }

                    