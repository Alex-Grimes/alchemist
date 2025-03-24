use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct CreatePost {
    pub title: String,
    pub body: String,
    pub user_id: Option<i32>,
}
