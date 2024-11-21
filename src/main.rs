use actix_web::{get, web::Data, App, HttpResponse, HttpServer};
use dotenv::dotenv;
use sqlx::postgres::PgPoolOptions;
use zero_api::{
    db::app_state::AppState,
    services::{device_service::create_device, sensor_reading_service::create_reading},
};

#[get("/hello")]
async fn hello() -> HttpResponse {
    HttpResponse::Ok().body("Hello World!")
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    dotenv().ok();
    let database_url = std::env::var("DATABASE_URL").expect("DATABASE_URL must to be set");
    let pool = PgPoolOptions::new()
        .max_connections(10)
        .connect(database_url.as_str())
        .await
        .expect("Failed to connect to the database");

    println!("Connection to DB resolved");

    // Iniciando o servidor Actix
    HttpServer::new(move || {
        App::new()
            .app_data(Data::new(AppState { db: pool.clone() })) // Passando o pool de conexões para AppState
            .service(create_device)
            .service(create_reading)
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}
