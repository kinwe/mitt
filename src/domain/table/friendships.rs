use rbdc::datetime::FastDateTime;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Friendships {
    pub friendship_id: Option<i64>,
    pub user_id: Option<i64>,
    pub friend_id: Option<i64>,
    pub status: Option<String>,
    pub action_user_id:Option<i64>,
    pub friendship_date: Option<FastDateTime>,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct UsersInfo {
    pub id: Option<i64>,
    pub username: Option<String>,
    pub profile_picture: Option<String>,
    pub status: Option<String>,
}


#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Friends {
    pub user_id: String,
    pub friends: Option<Vec<UsersInfo>>,
}