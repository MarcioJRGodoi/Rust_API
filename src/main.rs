use actix_web::{web, App, HttpServer, Responder, HttpResponse};
use diesel::prelude::*;
use diesel::r2d2::{self, ConnectionManager};
use std::env;
use dotenv::dotenv;
use serde::Deserialize;

mod db;
mod schema;
mod models;

use models::{User, NewUser};
use schema::users::dsl::*;
use db::{establish_connection, run_migrations};

type DbPool = r2d2::Pool<ConnectionManager<PgConnection>>;

#[derive(Deserialize)]
struct UserInput {
    name: String,
    email: String,
}

#[derive(Deserialize)]
struct PostInput {
    title: String,
    body: String,
    user_id: i32,
}

async fn create_user(pool: web::Data<DbPool>, item: web::Json<UserInput>) -> impl Responder {
    let conn = pool.get().expect("couldn't get db connection from pool");

    let new_user = NewUser {
        name: item.name.clone(),
        email: item.email.clone(),
    };

    let inserted_user: User = diesel::insert_into(users)
        .values(&new_user)
        .get_result(&conn)
        .expect("Error saving new user");

    HttpResponse::Ok().json(inserted_user)
}

async fn get_users(pool: web::Data<DbPool>) -> impl Responder {
    let conn = pool.get().expect("couldn't get db connection from pool");

    let results = users
        .load::<User>(&conn)
        .expect("Error loading users");

    HttpResponse::Ok().json(results)
}


#[actix_web::main]
async fn main() -> std::io::Result<()> {
    dotenv().ok();
    env_logger::init();

    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    let manager = ConnectionManager::<PgConnection>::new(database_url);
    let pool = r2d2::Pool::builder()
        .build(manager)
        .expect("Failed to create pool.");

    // Execute migrations
    let mut conn = establish_connection();
    run_migrations(&mut conn);

    HttpServer::new(move || {
        App::new()
            .app_data(web::Data::new(pool.clone()))
            .route("/users", web::post().to(create_user))
            .route("/users", web::get().to(get_users))
    })
    .bind("127.0.0.1:8080")?
    .run()
    .await
}
