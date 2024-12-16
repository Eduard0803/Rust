use serde::{Deserialize, Serialize};
use std::sync::Mutex;

lazy_static! {
    pub static ref GLOBAL_USER: Mutex<Vec<UserResponse>> = Mutex::new(Vec::new());
    pub static ref GLOBAL_ID: Mutex<u32> = Mutex::new(1);
}

#[derive(Serialize, Clone)]
pub struct UserResponse {
    pub id: u32,
    pub name: String,
    pub email: String,
}

#[derive(Deserialize, Clone)]
pub struct GetUserRequest {
    pub n_users: usize,
}

#[derive(Deserialize, Clone)]
pub struct CreateUserRequest {
    pub name: String,
    pub email: String,
}

#[derive(Deserialize, Clone)]
pub struct DeleteUserRequest {
    pub id: u32,
}

#[derive(Serialize)]
pub struct Response {
    pub message: String,
}
