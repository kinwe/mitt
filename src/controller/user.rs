use log::{error, info};
use ntex::web;
use ntex::web::{ Responder};
use rbatis::dark_std::err;
use rbdc::datetime::{DateTime, FastDateTime};
use crate::domain::table::{Users};
use crate::{ pool};
use crate::domain::dto::{FormData, FormFrends};
use crate::domain::vo::{CODE_FAIL, JWTToken, RespVO};
use crate::errors::Error;
use crate::util::PasswordEncoder;
use crate::service;
pub type Result<T> = std::result::Result<T, Error>;

//注册账号
#[web::post("/add")]
pub async fn add(form: web::types::Json<FormData>) ->impl Responder {
    let data = Users::select_all_by_name(pool!(), &form.username).await;
    match data {
        Ok(data) => {if data.is_some() { return  RespVO::<String>::from_error_info(CODE_FAIL, &*format!("用户已存在 username: {}", data.unwrap().username.unwrap())).resp_json();}}
        Err(_) => {}
    }
    let user = Users{
        id: None,
        username:Some( form.clone().username),
        profile_picture:Some(form.clone().profile_picture),
        password: Some(PasswordEncoder::encode(&form.password)),
        reg_date: Some(FastDateTime::now()),
        del_date: None,
        status: Some(1),
    };

    if !service::add(user.clone()).await {
        err!("user add fail, username : {}", &user.clone().username.unwrap());
        return  RespVO::<String>::from_error_info(CODE_FAIL, &*format!("insert fail username: {}", user.clone().username.unwrap())).resp_json();
    }
    return  RespVO::from_result(&Ok(format!("insert success username: {}", user.clone().username.unwrap()))).resp_json();
}

//登陆
#[web::post("/api/login")]
pub async fn login(form: web::types::Json<FormData>) -> impl Responder{
    let data = service::select_user(form.clone().username).await;

    return match data {
        Ok(values)=> {
            if values.is_none() {
                error!("登陆失败!");
                return  RespVO::<String>::from_error_info(CODE_FAIL, format!("select user  empty! username :{}", &form.clone().username).as_str()).resp_json()
            }
            if  PasswordEncoder::verify(&values.clone().unwrap().password.unwrap(), &form.0.password) {
                let j = JWTToken {
                    id: values.clone().unwrap().id.unwrap().to_string(),
                    username: values.clone().unwrap().username.unwrap(),
                    exp: DateTime::now().set_micro(30).unix_timestamp_millis() as usize,
                };

                let token = j.create_token("sss").unwrap();
                info!("登陆成功!");
                RespVO::from_result(&Ok(token)).resp_json()
            } else {
                error!("登陆失败!");
                return  RespVO::<String>::from_error_info(CODE_FAIL, format!("login fail! username :{}", &values.clone().unwrap().username.unwrap()).as_str()).resp_json()
            }
        },
        Err(_)=>{error!("登陆失败!");return RespVO::<String>::from_error_info(CODE_FAIL,"errors").resp_json()}
    }
}