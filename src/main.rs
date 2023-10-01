use crate::repository::prelude::DadosSensor1Repository;
use handler::prelude::dados_sensor1_handler;
mod entity;
mod handler;
mod repository;
use actix_web::{ web::{self, Data}, App, HttpServer, middleware
};
use sea_orm::DatabaseConnection;

use std::env;

#[derive(Debug, Clone)]

pub struct AppState {
    pub dados_sensor1_repository: DadosSensor1Repository,
}

#[actix_web::main]

async fn main() -> std::io::Result<()> {
    std::env::set_var("RUST_LOG", "debug");
    dotenv::dotenv().ok();
    env_logger::init();
    let db_url = std::env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    let db_conn = sea_orm::Database::connect(&db_url).await.unwrap();
    let host = std::env::var("HOST").expect("HOST musc be set");
    let port = std::env::var("PORT").expect("PORT musc be set");
    let server_url = format!("{}:{}", host, port);

    let dados_sensor1_repository = DadosSensor1Repository {
        db_conn: db_conn.clone(),
    };

    let state: AppState = AppState {
        dados_sensor1_repository: dados_sensor1_repository,
    };

    let server = HttpServer::new(move || {
        App::new()
        .app_data(Data::new(state.clone()))
        .wrap(middleware::Logger::default())
        .configure(init)
    })
    .bind(&server_url)?;
    println!("Server running on {}", server_url);
    server.run().await?;
    Ok(())
}

pub fn init(cfg: &mut  web::ServiceConfig) {
    cfg.service( dados_sensor1_handler());
}