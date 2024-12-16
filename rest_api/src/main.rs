#[macro_use]
extern crate lazy_static;

use axum::{
    extract::Query, response::IntoResponse, routing::{get, post, delete}, Json, Router
};
use serde::{Deserialize, Serialize};
use std::net::SocketAddr;
use colored::*;
use std::sync::Mutex;
use std::io;
use std::io::Write;

lazy_static! {
    static ref GLOBAL_USER: Mutex<Vec<UserResponse>> = Mutex::new(Vec::new());
    static ref GLOBAL_ID: Mutex<u32> = Mutex::new(0);
}

fn print(output: &str) {
    print!("{}", output);
    io::stdout().flush().expect("Failed to flush stdout");
}

#[tokio::main]
async fn main() {
    let app = Router::new()
        .route("/", get(root))
        .route("/users", get(get_users))
        .route("/users", post(create_user))
        .route("/users", delete(delete_user));

    let addr = SocketAddr::from(([127, 0, 0, 1], 3000));
    print(&format!("Listening on {}\n", addr.to_string().red().underline()));

    axum::Server::bind(&addr)
        .serve(app.into_make_service())
        .await
        .unwrap();
}

async fn root() -> Json<Response> {
    let message_= String::from("server is running");

    print(&format!("{} {}", "\nGET".blue(), " /"));

    let response: Response = Response {
        message: message_
    };
    
    Json(response)
}

async fn get_users(Query(payload): Query<GetUserRequest>) -> impl IntoResponse {
    let users = GLOBAL_USER.lock().unwrap();
    let mut last_users: Vec<_> = users.iter().rev().take(payload.n_users).cloned().collect();

    print(&format!("{}", "\nGET".blue()));
    print(&format!(
        " /users: {}\n", serde_json::to_string_pretty(&*last_users).unwrap()
    ));

    last_users.reverse();
    Json(last_users.clone())
}

async fn create_user(Json(payload): Json<CreateUserRequest>) -> Json<UserResponse> {
    let mut users = GLOBAL_USER.lock().unwrap();
    let mut user_id = GLOBAL_ID.lock().unwrap();
    let response: UserResponse = UserResponse {
        id: *user_id,
        name: payload.name,
        email: payload.email,
    };

    users.push(response.clone());
    *user_id += 1;

    print(&format!("{}", "\nPOST".green()));
    print(
        &format!(" /users: {}\n", 
        serde_json::to_string_pretty(&response).unwrap()
    ));
    Json(response)
}

async fn delete_user(Query(payload): Query<DeleteUserRequest>) -> impl IntoResponse {
    let mut users = GLOBAL_USER.lock().unwrap();

    users.retain(|users| users.id != payload.id);

    let response: Response = Response {
        message: format!("User from id: {} deleted", payload.id),
    };

    print(&format!("{}", "\nDELETE".red()));
    print(&format!(" /users: User from id: {} deleted\n", payload.id));

    Json(response)
}

#[derive(Serialize, Clone)]
struct Response {
    message: String,
}

#[derive(Deserialize, Clone)]
struct GetUserRequest {
    n_users: usize,
}

#[derive(Deserialize, Clone)]
struct CreateUserRequest {
    name: String,
    email: String,
}

#[derive(Serialize, Clone)]
struct UserResponse {
    id: u32,
    name: String,
    email: String,
}

#[derive(Deserialize, Clone)]
struct DeleteUserRequest {
    id: u32,
}
