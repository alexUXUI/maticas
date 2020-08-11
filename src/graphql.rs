
use crate::graphql_schema::Schema;
use actix_web::{web, HttpResponse};
use juniper::http::graphiql::graphiql_source;
use juniper::http::GraphQLRequest;
use std::sync::Arc;

pub fn graphql_routes(config: &mut web::ServiceConfig) {
    config
        .service(web::resource("/graphql").route(web::post().to(graphql)))
        .service(web::resource("/graphiql").route(web::get().to(graphiql)));
}

pub fn graphql(schema: web::Data<Arc<Schema>>, data: web::Json<GraphQLRequest>) -> HttpResponse {
    let res = data.execute(&schema, &());
    HttpResponse::Ok().json(res)
}

pub fn graphiql() -> HttpResponse {
    let html = graphiql_source("/graphql"); 
    HttpResponse::Ok()
        .content_type("text/html; charset=utf-8")
        .header("Access-Control-Allow-Origin", "*")
        .body(html)
}