use crate::sql_queries;
use crate::models::{PlantResponse, Status};
use actix_web::{web, HttpResponse, Responder};
use deadpool_postgres::{Client, Pool};

pub async fn status() -> impl Responder {
    web::HttpResponse::Ok().json(Status {
        status: "Hello 🌎! Checkout /graphiql".to_string()
    })
}

pub async fn get_all_plants(db_pool: web::Data<Pool>) -> impl Responder {
    let client: Client = db_pool
        .get()
        .await
        .expect("Could not connect to DB from get all plants");

    let result = sql_queries::get_all_plants(&client).await;
    match result {
        Ok(all_plants) => HttpResponse::Ok().json(PlantResponse { all_plants }),
        Err(_) => HttpResponse::InternalServerError().into(),
    }
}
