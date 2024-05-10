use serde::{Deserialize, Serialize};

// List
#[derive(Deserialize, Debug, Default)]
pub struct FilterOptions {
    pub page: Option<usize>,
    pub limit: Option<usize>,
}

// Read/Delete
#[derive(Deserialize, Debug)]
pub struct ParamOptions {
    pub id: String,
}

// Create
#[derive(Serialize, Deserialize, Debug)]
pub struct CreateUpdateUserSchema {
    pub username: String,
    pub email: String,
    pub phone: String,
    pub password: String,
}