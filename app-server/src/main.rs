mod db;
mod handlers;
mod routes;
mod models;
use warp::filter;
#[tokio::main]
async fn main(){
    db::initialise_db();
    let routes = routes::restaurant_routes();

    println!("Running the server");
    warp::serve(routes.with(warp::trace::request()))
    .run(([127, 0, 0, 1], 3030))
    .await;
}