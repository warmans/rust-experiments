use actix_web::{App, HttpServer, web};

use cache::storage;
use std::io;

//todo: is it not possible encapsulate ethe cache logic?
async fn put(data: web::Data<storage::Memory>) -> String {
    data.data.write().unwrap().insert(String::from("foo"), String::from("bar"));
    String::from("OK")
}

async fn fetch(data: web::Data<storage::Memory>) -> String {
    data.data.read().unwrap().get(&String::from("foo")).cloned().ok_or(io::ErrorKind::NotFound).unwrap()
}

#[actix_rt::main]
async fn main() -> std::io::Result<()> {
    let cache = web::Data::new(storage::new());

    HttpServer::new(
        move || {
            App::new()
                .app_data(cache.clone())
                .route("/put", web::get().to(put))
                .route("/fetch", web::get().to(fetch))
        })
        .bind("127.0.0.1:8888")?.run().await
}
