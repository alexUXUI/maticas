-- This file should undo anything in `up.sql`
CREATE TABLE class (
    id serial PRIMARY KEY,
    kingdom varchar NOT NULL,
    subkingdom varchar NOT NULL,
    super_division varchar NOT NULL,
    division varchar NOT NULL,
    tax_class varchar NOT NULL,
    subclass varchar NOT NULL,
    tax_order varchar NOT NULL,
    family varchar NOT NULL,
    genus varchar NOT NULL
);

CREATE TABLE sunlight (
    id serial PRIMARY KEY,
    light varchar NOT NULL,
    direct boolean NOT NULL,
    filtered boolean not NUll
);

CREATE TABLE water (
    id serial PRIMARY KEY,
    frequency integer NOT NULL,
    duration varchar not null
);

CREATE TABLE region (
    id serial PRIMARY KEY,
    continent varchar NOT NULL,
    direction varchar NOT NULL
);

CREATE TABLE habitat (
    id serial PRIMARY KEY,
    habitat_type varchar NOT NULL
);

CREATE TABLE plant (
    id serial PRIMARY KEY,
    name varchar NOT NULL,
    species varchar NOT NULL,
    class_id integer NOT NULL,
    sunlight_id integer NOT NULL,
    water_id integer NOT NULL,
    region_id integer NOT NULL,
    habitat_id integer NOT NULL
);

ALTER TABLE plant
    ADD FOREIGN KEY (class_id) REFERENCES class(id);

ALTER TABLE plant
    ADD FOREIGN KEY (region_id) REFERENCES region(id);

ALTER TABLE plant
    ADD FOREIGN KEY (sunlight_id) REFERENCES sunlight(id);

ALTER TABLE plant
    ADD FOREIGN KEY (water_id) REFERENCES water(id);

ALTER TABLE plant
    ADD FOREIGN KEY (habitat_id) REFERENCES plant(id);

INSERT INTO sunlight (light, direct, filtered) VALUES 
    ('low', false, false),
    ('low', true, false),
    ('low', false, true),
    ('low', true, true),
    ('medium', false, false),
    ('medium', true, false),
    ('medium', false, true),
    ('medium', true, true),
    ('bright', false, false),
    ('bright', true, false),
    ('bright', false, true),
    ('bright', true, true);

INSERT INTO water (frequency, duration) VALUES
    (1, 'week'),
    (2, 'week'),
    (3, 'week'),
    (4, 'week'),
    (5, 'week'),
    (1, 'day'),
    (1, 'month'),
    (2, 'month'),
    (3, 'month');

INSERT INTO region (continent, direction) VALUES
    ('Noth America', 'North'),
    ('Noth America', 'South'),
    ('Noth America', 'East'),
    ('Noth America', 'West'),
    ('South America', 'North'),
    ('South America', 'South'),
    ('South America', 'East'),
    ('South America', 'West'),
    ('Australia', 'North'),
    ('Australia', 'South'),
    ('Australia', 'East'),
    ('Australia', 'West'),
    ('Africa', 'North'),
    ('Africa', 'South'),
    ('Africa', 'East'),
    ('Africa', 'West'),
    ('Antartica', 'North'),
    ('Antartica', 'South'),
    ('Antartica', 'East'),
    ('Antartica', 'West'),
    ('Europe', 'North'),
    ('Europe', 'South'),
    ('Europe', 'East'),
    ('Europe', 'West'),
    ('Asia', 'North'),
    ('Asia', 'South'),
    ('Asia', 'East'),
    ('Asia', 'West');

INSERT INTO habitat (habitat_type) VALUES
    ('Tropical Rainforest'),
    ('Rainforest'),
    ('Forest'),
    ('Desert'),
    ('Beach'),
    ('Mountains'),
    ('Humid'),
    ('Dry'),
    ('Rainy'),
    ('Sunny'),
    ('Shady'),
    ('Windy'),
    ('Temperate'),
    ('Extreme');

INSERT INTO class (kingdom, subkingdom, super_division, division, tax_class, subclass, tax_order, family, genus) VALUES 
    ('Plantae', 'Viridiplantae', 'Streptophyta', 'Embryophyta', 'Tracheophyta', 'Magnoliopsida', 'Asparagales', 'Asparagaceae', 'Lomandroideae'),
    ('Plantae', 'Tracheobionta', 'Spermatophyta', 'Magnoliophyta', 'Magnoliopsida', 'Hamamelididae', 'Urticales', 'Moraceae', 'Ficus L.');

INSERT INTO plant (name, species, class_id, sunlight_id, water_id, region_id, habitat_id) VALUES 
    ('Cordyline Terminales', 'Cordyline fruticosa', 1, 3, 3, 26, 1),
    ('Fiddle Leaf Fig', 'Ficus lyrata Warb.', 2, 11, 2, 16, 2);
