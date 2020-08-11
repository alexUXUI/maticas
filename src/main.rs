mod config;
mod schema;
mod models;
mod sql_queries;
mod graphql;
mod graphql_schema;
mod handlers;

// DB
use tokio_postgres::NoTls;

// Server Framework
use actix_web::{HttpServer, App, web};
use std::{io, env};
use dotenv::dotenv;

// GraphQl
extern crate juniper;
use crate::graphql::graphql_routes;
use crate::graphql_schema::{create_schema};

// ORM
#[macro_use]
extern crate diesel;

// Route Handlers
use crate::handlers::*;

#[actix_rt::main]
async fn main() -> io::Result<()> {
    dotenv().ok();

    let config = crate::config::Config::from_env().unwrap();

    let pool = config.pg.create_pool(NoTls).unwrap();

    let schema = std::sync::Arc::new(create_schema());

    println!("Start server {:#?}", config);

    let port = env::var("PORT")
        .unwrap_or_else(|_| "3000".to_string())
        .parse()
        .expect("PORT must be a number");

    HttpServer::new(move || {
        App::new()
            .data(pool.clone())
            .data(schema.clone())
            .configure(graphql_routes)
            .route("/", web::get().to(status))
            .route("/plants", web::get().to(get_all_plants))
    })
    .bind(("0.0.0.0", port))
    .expect("Can not bind to port 8000")
    .run()
    .await
}
