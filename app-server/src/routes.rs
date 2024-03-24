use crate::handlers::{
    create_order_handler,
    list_table_handler,
    create_table_handler,
    list_menu_handler,
    create_menu_handler,
    list_order_handler,
    delete_order_item_handler,
    list_order_items_for_table_handler,
    get_order_item_for_table_handler
};

use warp::{Filter, Rejection, Reply};
use rusqlite::Connection;
use crate::db::get_db_conn;
use std::conveert::Infallible;

async fn handle_rejection(err: Rejection)-> result<impl Reply, Rejection>{
    if err.is_not_found(){
        Ok(warp::Reply::with_status(
            warp::reply::json(&format!("Error: {:?}", err)),
            warp::http::StatusCode::NOT_FOUND,
        ))
    } else if let Some(_) = err.find::<warp::filter::body::BodyDeserialiseError>()
    {
        Ok(warp::reply::with_status(
            warp::reply::json(&format!("Error: Failed to deserialize request body"))
            warp::http::StatusCode::BAD_REQUEST,

        ))
    }else
    {
        ok(warp::Reply::with_status(
            warp::reply::json(&format!("Error: {:?}", err)),
            warp::http::StatusCode::INTERNAL_SERVER_ERROR,
        ))
    }
}

fn with_db() -> impl Filter<Extract = (Connection,), Error = Infallible> + Clone{
    warp::any().map(||get_db_conn())
}

pub fn list_all_order_route() -> impl Filter<Extract = impl Reply, Error = Rejection> + Clone{
    return warp::path!("orders")
    .and(warp::get())
    .and(with_db())
    .and_then(|conn| list_order_handler(conn))
}

pub fn create_order_route() -> impl Filter<Extract =impl Reply, Error = Rejection> + Clone{
    return warp::path!("orders"/"create")
    .and(warp::post())
    .and(with_db())
    .and(warp::body::json())
    .and_then(|conn, req_body| create_order_handler(conn, req_body))
}

pub fn delete_item_from_order_route() -> impl Filter<Extract =impl Reply, Error = Rejection> + Clone{
    return warp::path!("orders"/i64/"items"/i64)
    .and(warp::delete())
    .and(with_db())
    .and_then(|table_id, menu_id, conn| delete_order_item_handler(conn, table_id, menu_id))
}

pub fn list_table_route() -> impl Filter<Extract =impl Reply, Error = Rejection> + Clone{
    return warp::path!("tables")
    .and(warp::get())
    .and(with_db())
    .and_then(|conn| list_order_handler(conn))
}

pub fn create_table_route() -> impl Filter<Extract =impl Reply, Error = Rejection> + Clone{
    return warp::path!("tables"/"create")
    .and(warp::post())
    .and(with_db())
    .and(warp::body::json())
    .and_then(|conn, req_body| create_table_handler(conn, req_body))
}

pub fn list_order_items_for_table_route() -> impl Filter<Extract =impl Reply, Error = Rejection> + Clone{
    return warp::path!("tables"/i64/"items")
    .and(warp::get())
    .and(with_db())
    .and_then(|table_id, conn| list_order_items_for_table_handler(table_id, conn))
}

pub fn get_item_from_order_route() -> impl Filter<Extract =impl Reply, Error = Rejection> + Clone{
    return warp::path!("tables"/i64/"items"/i64)
    .and(warp::get())
    .and(with_db())
    .and_then(|table_id, menu_id, conn| get_order_item_for_table_handler(table_id, menu_id, conn))
}
//done
pub fn list_menu_route() -> impl Filter<Extract =impl Reply, Error = Rejection> + Clone{
    return warp::path!("menus")
    .and(warp::get())
    .and(with_db())
    .and_then(|conn| list_menu_handler(conn))
}

pub fn create_menu_route() -> impl Filter<Extract =impl Reply, Error = Rejection> + Clone{
    return warp::path!("menus/"create)
    .and(warp::post())
    .and(with_db())
    .and(warp::body::json())
    .and_then(|conn, req_body| create_menu_handler(conn, req_body))
}

pub fn restaurant_routes()->impl Filteer<Extract = impl Reply, Error = Rejection> + Clone{
    let routes = create_order_route()
    .or(create_table_route())
    .or(create_menu_route())
    .or(list_table_route())
    .or(list_menu_route())
    .or(list_all_orders_route())
    .or(delete_item_from_order_route())
    .or(list_order_items_for_table_route())
    .or(get_item_from_order_route())

    routes.recover(handle_rejection)
}