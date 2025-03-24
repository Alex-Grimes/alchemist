use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct UpdatePost {
    pub title: String,
    pub body: String,
    pub user_id: Option<i32>,
}
