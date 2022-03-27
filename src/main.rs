#[macro_use]
extern crate lazy_static;

mod crawl;
mod db;
mod http;
mod item;
mod utils;

use axum::{routing::get, Router};

use crate::crawl::Target;

async fn crawl(target: Target, db_file: &str) {
    let c = crawl::build_crawler(target, db_file).unwrap();
    c.run();
}

#[tokio::main]
async fn main() {
    let _ = tokio::spawn(crawl(Target::Buff, "./data/buff.db"));

    // build our application with a single route
    let app = Router::new().route("/", get(|| async { "Hello, World!" }));

    println!("server start");
    // run it with hyper on localhost:3000
    axum::Server::bind(&"0.0.0.0:3000".parse().unwrap())
        .serve(app.into_make_service())
        .await
        .unwrap();
}
