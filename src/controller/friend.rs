use log::{error};
use ntex::web;
use ntex::web::Responder;
use rbatis::dark_std::err;
use crate::domain::dto::FormFrends;
use crate::domain::table::UsersInfo;
use crate::domain::vo::{CODE_FAIL, RespVO};
use crate::service;

//添加好友关系
#[web::post("/add_friend")]
pub async fn add_friend(form: web::types::Json<FormFrends>) ->impl Responder {
    if !service::add_friend(&form).await {
        err!("friend add fail, user_id : {}", &form.clone().user_id);
        return  RespVO::<String>::from_error_info(CODE_FAIL, &*format!("friend add fail user_id: {}, friend_id : {}", form.clone().user_id, form.friend_id.unwrap())).resp_json();
    }
    return  RespVO::from_result(&Ok(format!("friend add  success user_id: {}", form.clone().user_id))).resp_json();
}

//获取好友列表信息
#[web::post("/get_friends")]
pub async fn get_friends(form: web::types::Json<FormFrends>) -> impl Responder{
    let data = service::select_friends(form.clone().user_id).await;
    let mut ruselt: Vec<UsersInfo> = Vec::new() ;

    return match data {
        Ok(values)=> {
            if values.is_none() {
                error!("还没有好友!");
                return  RespVO::<String>::from_error_info(CODE_FAIL, format!("select user  empty! username :{}", &form.clone().user_id).as_str()).resp_json()
            }
            if values.clone().unwrap().len() !=0 {
                for i in 0..values.clone().unwrap().len() {
                    let r = service::select_user_by_id(values.clone().unwrap().get(i).unwrap().friend_id.unwrap()).await;
                    if r.is_ok() & r.as_ref().unwrap().is_some() {
                        let user =  r.unwrap().unwrap().clone();
                        let d = UsersInfo{id:user.id, username:user.username,profile_picture:user.profile_picture , status:None};
                        ruselt.push(d);
                    }
                }
                return RespVO::from(&ruselt).resp_json()
            }
            return RespVO::<String>::from_error_info(CODE_FAIL,"还没有好友!").resp_json()
        },
        Err(_)=>{error!("还没有好友!");return RespVO::<String>::from_error_info(CODE_FAIL,"errors").resp_json()}
    }
}

//删除好友
#[web::post("/delete_friend")]
pub async fn delete_friend(form: web::types::Json<FormFrends>) -> impl Responder{
    if form.friend_id.is_none() || form.user_id == 0 {
        return RespVO::<String>::from_error_info(CODE_FAIL,"user_id or friend_id is null").resp_json();
    }
    if !service::delete_friend(form.user_id, form.friend_id.unwrap()).await {
        err!("friend delete fail, user_id : {}", &form.clone().user_id);
    }
    return  RespVO::from_result(&Ok(format!("friend delete  success user_id: {}", form.clone().user_id))).resp_json();
}

//拉黑好友
#[web::post("/black_friend")]
pub async fn black_friend(form: web::types::Json<FormFrends>) -> impl Responder{
    if form.friend_id.is_none() || form.user_id == 0 {
        error!("user_id or friend_id is null");
        return RespVO::<String>::from_error_info(CODE_FAIL,"user_id or friend_id is null").resp_json();
    }
    if !service::black_friend(form.user_id, form.friend_id.unwrap()).await {
        err!("friend black fail, user_id : {},friend_id: {}", &form.clone().user_id,form.clone().friend_id.unwrap());
    }
    return  RespVO::from_result(&Ok(format!("friend black  success user_id: {} , friend_id: {}", form.clone().user_id, form.clone().friend_id.unwrap()))).resp_json();
}