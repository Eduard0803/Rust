#[macro_use]
extern crate lazy_static;

use axum::{routing::{get, post, delete}, Router};
use std::net::SocketAddr;
use colored::*;
use crate::controllers::user_controller::*;

mod controllers{pub mod user_controller;}
mod models{pub mod user_model;}
mod views{pub mod user_view;}


#[tokio::main]
async fn main() {
    let app = Router::new()
        .route("/", get(root))
        .route("/users", get(get_users))
        .route("/users", post(create_user))
        .route("/users", delete(delete_user));

    let addr = SocketAddr::from(([127, 0, 0, 1], 3000));
    println!("Listening on {}\n", addr.to_string().red().underline());

    axum::Server::bind(&addr)
        .serve(app.into_make_service())
        .await
        .unwrap();
}
