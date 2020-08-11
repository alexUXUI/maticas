use serde::Serialize;
use tokio_pg_mapper_derive::PostgresMapper;

#[derive(Serialize)]
pub struct Status {
    pub status: String
}

#[derive(Serialize, PostgresMapper)]
#[pg_mapper(table="plant")]
pub struct Plant {
    pub id: i32,          
    pub name: String,
    pub species: String,
    pub class_id: i32,
    pub sunlight_id: i32,
    pub water_id: i32,
    pub region_id: i32,
    pub habitat_id: i32,
}

#[derive(Serialize)]
pub struct PlantResponse {
    pub all_plants: Vec<Plant>
}

#[derive(Serialize, PostgresMapper)]
#[pg_mapper(table="plant")]
pub struct PlantWithAllFields {
    id: i32, 
    name: String,         
    species: String,       
    class_id: i32, 
    sunlight_id: i32, 
    water_id: i32, 
    region_id: i32, 
    habitat_id: i32, 
    kingdom: String, 
    subkingdom: String,   
    super_division: String, 
    division: String,    
    tax_class: String,     
    subclass: String,    
    tax_order: String,  
    family: String,    
    genus: String,     
    light: String,  
    direct: bool, 
    filtered: bool,
    continent: String,
    direction: String,
    duration: String,
    frequency: i32,
    habitat_type: String
}

#[derive(Serialize)]
pub struct PlantWithAllFieldsResponse {
    pub plants_with_all_fields:  Vec<PlantWithAllFields>
}