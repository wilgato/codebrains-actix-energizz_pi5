use crate::entity::dados_sensor1::Model as DadosSensor1;
use crate::AppState;
use actix_web::{
    get, post, web, web::Json, Error as ActixError, Responder, Result as ActixResult, Scope, put, delete,
};
use log::{debug};
use sea_orm::DeleteResult;
use serde_json::json;

#[get("")]
async fn get_dados_sensor1(state: web::Data<AppState>) -> ActixResult<impl Responder, ActixError> {
    let dados_sensor1 = state.dados_sensor1_repository.get_dados_sensor1().await;
    Ok(web::Json(dados_sensor1))
}

#[get("/{id}")]
async fn get_dados_sensor1_by_id(state: web::Data<AppState>, id: web::Path<i32>) -> ActixResult<impl Responder, ActixError> {
    let dados_sensor1 = state.dados_sensor1_repository.get_dados_sensor1_by_id(id.into_inner()).await;
    Ok(web::Json(dados_sensor1))
}

#[post("")]
async fn create_dados_sensor1(
    state: web::Data<AppState>,
    new_dados_sensor1: Json<crate::repository::dados_sensor1::DadosSensor1Request>,
) -> ActixResult<impl Responder, ActixError> {
    let dados_sensor1: Option<DadosSensor1> = state.dados_sensor1_repository.create_dados_sensor1(new_dados_sensor1).await;
    Ok(web::Json(dados_sensor1))
}

#[put("/{id}")]
async fn update_dados_sensor1(
    state: web::Data<AppState>,
    id: web::Path<i32>,
    new_dados_sensor1: Json<crate::repository::dados_sensor1::DadosSensor1Request>,
) -> ActixResult<impl Responder, ActixError> {
    let dados_sensor1: Option<DadosSensor1> = state.dados_sensor1_repository.update_dados_sensor1(id.into_inner(), new_dados_sensor1).await;
    Ok(web::Json(dados_sensor1))
}

#[delete("/{id}")]
async fn delete_dados_sensor1(
    state: web::Data<AppState>, 
    id: web::Path<i32> 
) -> ActixResult<impl Responder, ActixError> {
    let dados_sensor1: DeleteResult = state.dados_sensor1_repository.delete_dados_sensor1_by_id(id.into_inner()).await;
    Ok(web::Json(json!({
        "message": "Todo deleted successfully",
        "deleted": dados_sensor1.rows_affected
    })))
}

pub fn dados_sensor1_handler() -> Scope {
    web::scope("/dados_sensor1")
        .service(get_dados_sensor1)
        .service(get_dados_sensor1_by_id)
        .service(create_dados_sensor1)
        .service(update_dados_sensor1)
        .service(delete_dados_sensor1)
}