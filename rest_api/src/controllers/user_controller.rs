use axum::{extract::Query, Json};
use crate::models::user_model::*;
use crate::views::user_view::*;

pub async fn root() -> Json<Response> {
    let message = String::from("server is running");
    log_request("GET", "/", "");
    Json(Response { message })
}

pub async fn get_users(Query(payload): Query<GetUserRequest>) -> Json<Vec<UserResponse>> {
    let users = GLOBAL_USER.lock().unwrap();
    let mut last_users: Vec<_> = users.iter().rev().take(payload.n_users).cloned().collect();
    last_users.reverse();

    log_request("GET", "/users", &format!("Returning {} users", last_users.len()));
    log_json(&last_users);
    Json(last_users)
}

pub async fn create_user(Json(payload): Json<CreateUserRequest>) -> Json<UserResponse> {
    let mut users = GLOBAL_USER.lock().unwrap();
    let mut user_id = GLOBAL_ID.lock().unwrap();

    let user = UserResponse {
        id: *user_id,
        name: payload.name,
        email: payload.email,
    };

    users.push(user.clone());
    *user_id += 1;

    log_request("POST", "/users", &format!("User created with ID: {}", user.id));
    log_json(&user);
    Json(user)
}

pub async fn delete_user(Query(payload): Query<DeleteUserRequest>) -> Json<Response> {
    let mut users = GLOBAL_USER.lock().unwrap();
    users.retain(|user| user.id != payload.id);

    let response = Response {
        message: format!("User with ID: {} deleted", payload.id),
    };

    log_request("DELETE", "/users", &format!("User with ID: {} deleted", payload.id));
    log_json(&response);
    Json(response)
}
