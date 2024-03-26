use rbdc::datetime::FastDateTime;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Users {
    pub id: Option<i64>,
    pub username: Option<String>,
    pub password: Option<String>,
    pub profile_picture: Option<String>,
    pub reg_date: Option<FastDateTime>,
    pub del_date:Option<FastDateTime>,
    pub status: Option<i64>,
}