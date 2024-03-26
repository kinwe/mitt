use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct FormData {
    pub username: String,
    pub password: String,
    pub profile_picture : String
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct FormFrends {
    pub user_id: i64,
    pub friend_id :Option<i64>,
    pub status: Option<String>
}
