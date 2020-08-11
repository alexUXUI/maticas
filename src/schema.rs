table! {
    class (id) {
        id -> Int4,
        kingdom -> Varchar,
        subkingdom -> Varchar,
        super_division -> Varchar,
        division -> Varchar,
        tax_class -> Varchar,
        subclass -> Varchar,
        tax_order -> Varchar,
        family -> Varchar,
        genus -> Varchar,
    }
}

table! {
    habitat (id) {
        id -> Int4,
        habitat_type -> Varchar,
    }
}

table! {
    plant (id) {
        id -> Int4,
        name -> Varchar,
        species -> Varchar,
        class_id -> Int4,
        sunlight_id -> Int4,
        water_id -> Int4,
        region_id -> Int4,
        habitat_id -> Int4,
    }
}

table! {
    region (id) {
        id -> Int4,
        continent -> Varchar,
        direction -> Varchar,
    }
}

table! {
    sunlight (id) {
        id -> Int4,
        light -> Varchar,
        direct -> Bool,
        filtered -> Bool,
    }
}

table! {
    water (id) {
        id -> Int4,
        frequency -> Int4,
        duration -> Varchar,
    }
}

joinable!(plant -> class (class_id));
joinable!(plant -> region (region_id));
joinable!(plant -> sunlight (sunlight_id));
joinable!(plant -> water (water_id));

allow_tables_to_appear_in_same_query!(
    class,
    habitat,
    plant,
    region,
    sunlight,
    water,
);
