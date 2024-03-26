use serde::{Deserialize, Serialize};
use crate::domain::table::Users;

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Friendships {
    pub user_id: String,
    pub friends: Option<Vec<Users>>,
}