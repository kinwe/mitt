mod service;
mod util;
mod domain;
mod controller;
mod errors;
#[macro_use]
extern crate rbatis;
extern crate rbdc;

use clia_ntex_cors::Cors;
use fast_log::Config;
use ntex::{ web};
use crate::service::{CONTEXT};
use serde::{Deserialize, Serialize};

use crate::controller::{add, login, add_friend, get_friends,delete_friend,black_friend};


#[web::get("/")]
async fn index() -> impl web::Responder {
    "Hello, World!"
}

#[ntex::main]
async fn main() -> std::io::Result<()> {

    CONTEXT.init_pool().await;
    fast_log::init(Config::new().file("target/test.log").chan_len(Some(100000))).unwrap();


    // open a connection to RabbitMQ server
//     //
//     let connection = Connection::open(&OpenConnectionArguments::new(
//         "127.0.0.1",
//         5672,
//         "kinwe",
//         "hx19870527",
//     )).await.unwrap();
//     connection
//         .register_callback(DefaultConnectionCallback)
//         .await
//         .unwrap();
//
// // open a channel on the connection
//     let channel = connection.open_channel(None).await.unwrap();
//     channel
//         .register_callback(DefaultChannelCallback)
//         .await
//         .unwrap();
//
// // declare a queue
//     let (queue_name, _, _) = channel
//         .queue_declare(QueueDeclareArguments::default())
//         .await
//         .unwrap()
//         .unwrap();
//
// // bind the queue to exchange
//     let rounting_key = "amqprs.example";
//     let exchange_name = "amq.topic";
//     channel
//         .queue_bind(QueueBindArguments::new(
//             "amq.gen--KNfPtFArNOqGapp_KkVVQ",
//             exchange_name,
//             rounting_key,
//         ))
//         .await
//         .unwrap();
//
// //////////////////////////////////////////////////////////////////
// // start consumer with given name
//     let args = BasicConsumeArguments::new(
//         &queue_name,
//         "basic_consumer"
//     );
//
//     let channel1 = channel.clone();
//     let channel2 = channel.clone();
//
//     let task2 = task::spawn(  async move {
//         let args1  = BasicGetArguments{queue:"amq.gen--KNfPtFArNOqGapp_KkVVQ".to_string(), no_ack:true};
//
//         loop {
//             let r = channel1.basic_get(args1.clone()).await.unwrap();
//             if r.is_none() { break }
//             println!("{}", String::from_utf8_lossy(&r.unwrap().2));
//         }
//     });
//
//
// //////////////////////////////////////////////////////////////////
// // publish message
// //
//     let task1 =  task::spawn( async move {
//         let content = String::from(
//             r#"
//         {
//             "publisher": "example"
//             "data": "Hello, amqprs!"
//         }
//     "#,
//         )
//             .into_bytes();
//
// // create arguments for basic_publish
//         let args = BasicPublishArguments::new(exchange_name, rounting_key);
//
//         for _ in 0..10000 {
//             channel2
//                 .basic_publish(BasicProperties::default(), content.clone(), args.clone())
//                 .await
//                 .unwrap();
//         }
//     });

    web::HttpServer::new(|| web::App::new()
        .wrap(
            Cors::new() // <- Construct CORS middleware builder
                .allowed_origin("http://192.168.50.125:8080")
                .allowed_methods(vec!["GET", "POST","OPTIONS"])
                .max_age(3600)
                .finish())
        .service(index)
        .service(add)
        .service(login)
        .service(add_friend)
        .service(get_friends)
        .service(delete_friend)
        .service(black_friend)
    )
        .bind(("192.168.50.125", 8080))?
        .run()
        .await
}