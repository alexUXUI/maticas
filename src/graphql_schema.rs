extern crate dotenv;

use diesel::prelude::*;
use diesel::PgConnection;
use dotenv::dotenv;
use juniper::{EmptyMutation, RootNode};
use std::env;

fn establish_connection() -> PgConnection {
    dotenv().ok();
    let database_url = env::var("DATABASE_URL").expect("DATABASE URL must be set");
    PgConnection::establish(&database_url)
        .expect(&format!("error connecting to DB {}", database_url))
}

#[derive(Queryable)]
struct Plant {
    pub id: i32,
    pub name: String,
    pub species: String,
    pub class_id: i32,
    pub sunlight_id: i32,
    pub water_id: i32,
    pub region_id: i32,
    pub habitat_id: i32,
}

#[juniper::object(description = "Root node of plant object")]
impl Plant {
    pub fn id(&self) -> i32 {
        self.id
    }

    pub fn name(&self) -> String {
        self.name.to_owned()
    }

    pub fn species(&self) -> String {
        self.species.to_owned()
    }
}

pub struct QueryRoot;

#[juniper::object]
impl QueryRoot {
    fn plants() -> Vec<Plant> {
        use crate::schema::plant::dsl::*;
        let connection = establish_connection();

        plant
            .load::<Plant>(&connection)
            .expect("Could not resolve plant data")
    }
}

pub type Schema = RootNode<'static, QueryRoot, EmptyMutation<()>>;

pub fn create_schema() -> Schema {
    Schema::new(QueryRoot {}, EmptyMutation::new())
}
