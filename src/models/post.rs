use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct Post {
    pub id: i32,
    pub user_id: Option<i32>,
    pub title: String,
    pub body: String,
}

impl Post {
    pub fn new(id: i32, user_id: Option<i32>, title: String, body: String) -> Self {
        Post {
            id,
            user_id,
            title,
            body,
        }
    }
}
