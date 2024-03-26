use crate::domain::table::Users;
use crate::errors::Error;
use crate::pool;

pub async fn add(user:Users) -> bool {
    let data = Users::select_all_by_name(pool!(), &user.username.clone().unwrap()).await;

    match data {
        Ok(values)=> {
            if values.is_none() {
                let _ = Users::insert(pool!(), &user).await;
                return true
            }
        }
        Err(_)=>{ return false}
    }
   true
}

pub async fn select_user(username :String)->Result<Option<Users>, rbdc::Error> {
    let data = Users::select_all_by_name(pool!(), &username).await;
    data
}

pub async fn select_user_by_id(id :i64)->Result<Option<Users>, rbdc::Error> {
    let data = Users::select_all_by_id(pool!(), id).await;
    data
}