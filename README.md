# Maticas Backend  

### DB, HTTP Server, and GraphQl layer


### About

Minimal starter for an HTTP actix server with a postgres DB and a graphql layer.

### Up and Running

##### 1) Create the DB 🐘

Create a new postgres DB on your local machine and add the following envs to the .env file to configure the connection:

```yaml
PG.USER=maticas
PG.PASSWORD=maticas
PG.HOST=0.0.0.0
PG.PORT=5432
PG.DBNAME=maticas
PG.POOL.MAX_SIZE=30
```

This example uses `maticas` as the user, password and DB name but you can use whatever you want. 

If you have `psql` installed on your computer you can type:

```shell
$ psql
$ CREATE DATABASE maticas;
$ CREATE USER maticas;
$ ALTER ROLE maticas WITH SUPERUSER 
$ ALTER ROLE maticas WITH PASSWORD 'maticas';
```

This will create the database and user for your DB connection.


##### 2) Connect BD to Diesel ORM ⛓

Create another env called `DATABASE_URL` in the .env file. Set this variable equal to your database's connection string. It should look like this based on the example above:

`DATABASE_URL=postgres://maticas:maticas@localhost:5432/maticas`

Where:

`DATABASE_URL=postgres://<user>:<password>@<host>:<port>/<db_name>`

##### Run migrations & Seed Data 🌱

Once you have diesel connected to your DB, you can run the migrations with: 

```shell
$ cargo install diesel_cli
$ diesel setup
$ diesel migration run
```

This will install Diesel CLI and run the migrations in the `/migrations` directory. 

These migrations will create the schema and poplaute the tables with some seed data that you can query to sanity-check that everything is working as expected.

##### 3) Start the App 🚀 

```shell
$ git clone git@github.com:alexUXUI/maticas.git
$ cd maticas
$ cargo run
```

Once you have the app running, naviagte to http://127.0.0.1:3000/graphiql to make a query. Try typing this into the graphql query builder and pressing play:

```gql
query plants {
  plants {
    id
    name
    species
  }
}
```