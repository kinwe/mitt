use rbdc::datetime::FastDateTime;
use crate::domain::table::{Friends, Friendships};
use crate::pool;

pub async fn add_friend(user:&crate::domain::dto::FormFrends) -> bool {
    if user.friend_id.is_none() {
        return false;
    }

    let mainuser = Friendships::select_all_by_id(pool!(), user.user_id.clone(), user.friend_id.unwrap()).await;
    let friend_user = Friendships::select_all_by_id(pool!(), user.friend_id.unwrap().clone(), user.user_id.clone()).await;

    match mainuser {
        Ok(values)=> {
            if friend_user.unwrap().is_some() { return false }
            if values.is_none() {
                let friend = Friendships{
                    friendship_id: None,
                    user_id: Some(user.user_id),
                    friend_id: user.friend_id,
                    status: user.status.clone(),
                    action_user_id: Some(user.user_id),
                    friendship_date: Some(FastDateTime::now()),
                };

                let friend1 = Friendships{
                    friendship_id: None,
                    user_id: user.friend_id,
                    friend_id: Some(user.user_id),
                    status: user.status.clone(),
                    action_user_id: user.friend_id,
                    friendship_date: Some(FastDateTime::now()),
                };
                //事务开始
                let mut tx = pool!().acquire_begin().await.unwrap();

                let _ = Friendships::insert(pool!(), &friend).await;
                let _ = Friendships::insert(pool!(), &friend1).await;

                //提交事务
                let txs = tx.commit().await.unwrap();
                //如有异常，执行回滚
                if !txs {     tx.rollback().await.unwrap(); return false }
                return true
            }
        }
        Err(_)=>{ return false}
    }
    true
}

pub async fn select_friends(user_id :i64)->Result<Option<Vec<Friendships>>, rbdc::Error> {
    let data = Friendships::select_all_by_userid(pool!(), user_id).await.unwrap();
    Ok(Some(data))
}

pub async fn delete_friend(user_id :i64, friend_id: i64)-> bool {
    let f = Friendships{user_id:Some(user_id), friend_id:Some(friend_id), status: Some("-1".to_string()),action_user_id:Some(user_id),friendship_date:Some(FastDateTime::now()), friendship_id:None};
    let data = Friendships::update_by_user_id(pool!(), &f, f.user_id.unwrap(), f.friend_id.unwrap()).await.unwrap();
    if data.rows_affected != 0 {
        return  true
    }
    false
}

pub async fn black_friend(user_id :i64, friend_id: i64)-> bool {
    let u = Friendships{user_id:Some(user_id), friend_id:Some(friend_id), status: Some("-2".to_string()),action_user_id:Some(user_id),friendship_date:Some(FastDateTime::now()), friendship_id:None};
    let f = Friendships{user_id:Some(friend_id), friend_id:Some(user_id), status: Some("-2".to_string()),action_user_id:Some(user_id),friendship_date:Some(FastDateTime::now()), friendship_id:None};
    let mut tx = pool!().acquire_begin().await.unwrap();
    let _ = Friendships::update_by_user_id(pool!(), &u, u.user_id.unwrap(), u.friend_id.unwrap()).await.unwrap();
    let _ = Friendships::update_by_user_id(pool!(), &f, f.user_id.unwrap(), f.friend_id.unwrap()).await.unwrap();
    let txs = tx.commit().await.unwrap();
    //如有异常，执行回滚
    if !txs {
        tx.rollback().await.unwrap();
        return false
    }
    true
}