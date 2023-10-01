use crate::entity::{prelude::*, dados_sensor1};
use actix_web::web::Json;
use log::debug;
use sea_orm::{DatabaseConnection, EntityTrait, Set, ActiveValue::NotSet};
use sea_orm::{entity::*, query::*, DeriveEntityModel, DeleteResult};
use serde::{Serialize, Deserialize};
use sea_orm::prelude::Date as SeaOrmDate;
use sea_orm::entity::prelude::*;

use sea_orm::{EnumIter, Iterable};

#[derive(DeriveIden)]
enum Post {
    Table,
    #[sea_orm(iden = "Integer")] // Renaming the identifier
    id,
    #[sea_orm(iden = "Float")] // Renaming the identifier
    temperatura,
    #[sea_orm(iden = "Float")] // Renaming the identifier
    umidade,
    #[sea_orm(iden = "Float")] // Renaming the identifier
    pressao,
    #[sea_orm(iden = "Text")] // Renaming the identifier
    paciente_id,
    #[sea_orm(iden = "Timestamp")] // Renaming the identifier
    posting_time,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct DadosSensor1Request {
    //#[sea_orm(primary_key)]
    pub id: i32,
    //#[sea_orm(iden = "Float")] // Renaming the identifier
    pub temperatura: f32,
    //#[sea_orm(column_type = "Float")]
    pub umidade: f32,
   // #[sea_orm(column_type = "Float")]
    pub pressao: f32,
   // #[sea_orm(column_type = "Text")]
    pub paciente_id: String,
    ///pub posting_time: Date, // Alterado o tipo para SeaOrmDate<Utc>
    pub posting_time: chrono::NaiveDateTime,
}

#[derive(Clone, Debug)]
pub struct DadosSensor1Repository {
    pub db_conn: DatabaseConnection,
}

impl DadosSensor1Repository {
    pub async fn get_dados_sensor1(&self) -> Vec<dados_sensor1::Model> {
        DadosSensor1::find()
            .all(&self.db_conn)
            .await
            .expect("Error while fetching all dados_sensor1")
    }

    pub async fn get_dados_sensor1_by_id(&self, id: i32) -> Option<dados_sensor1::Model> {
        DadosSensor1::find_by_id(id)
            .one(&self.db_conn)
            .await
            .expect("Error while fetching todo by id")
    }

    pub async fn create_dados_sensor1(&self, new_dados_sensor1: Json<DadosSensor1Request>) -> Option<dados_sensor1::Model> {
        let dados_sensor1 = dados_sensor1::ActiveModel {
            id: NotSet,
            temperatura: ActiveValue::Set(new_dados_sensor1.temperatura.to_owned()),
            umidade: ActiveValue::Set(new_dados_sensor1.umidade.to_owned()),
            pressao: ActiveValue::Set(new_dados_sensor1.pressao.to_owned()),
            paciente_id: ActiveValue::Set(new_dados_sensor1.paciente_id.to_owned()),
            posting_time: ActiveValue::Set(new_dados_sensor1.posting_time), // Use diretamente o campo posting_time
            //posting_time: ActiveValue::Set(new_dados_sensor1.posting_time.to_owned()),
        };

        let dados_sensor1: dados_sensor1::Model = dados_sensor1.insert(&self.db_conn).await.unwrap();
        debug!("Created dados_sensor1: dados_sensor1{}", dados_sensor1.temperatura);
        return dados_sensor1.into();
    }

    pub async fn update_dados_sensor1(
        &self,
        id: i32,
        new_dados_sensor1: Json<DadosSensor1Request>,
    ) -> Option<dados_sensor1::Model> {
        let dados_sensor1 = DadosSensor1::find_by_id(id)
            .one(&self.db_conn)
            .await
            .expect("Failed to get dados_sensor1");
        let mut dados_sensor1: dados_sensor1::ActiveModel = dados_sensor1.unwrap().into();
        
        dados_sensor1.temperatura = ActiveValue::Set(new_dados_sensor1.temperatura.to_owned());
        dados_sensor1.umidade = ActiveValue::Set(new_dados_sensor1.umidade.to_owned());
        dados_sensor1.pressao = ActiveValue::Set(new_dados_sensor1.pressao.to_owned());
        dados_sensor1.paciente_id = ActiveValue::Set(new_dados_sensor1.paciente_id.to_owned());
        dados_sensor1.posting_time = ActiveValue::Set(new_dados_sensor1.posting_time); // Use diretamente o campo posting_time
        //dados_sensor1.posting_time = ActiveValue::Set(new_dados_sensor1.posting_time.to_owned());

        let dados_sensor1: dados_sensor1::Model = dados_sensor1.update(&self.db_conn).await.unwrap();
        debug!("Updated dados_sensor1: dados_sensor1{}", dados_sensor1.temperatura);
        return dados_sensor1.into();
    }

    pub async fn delete_dados_sensor1_by_id(&self, id: i32) -> DeleteResult {
        let res: DeleteResult = DadosSensor1::delete_by_id(id).exec(&self.db_conn).await.unwrap();

        return res.into();
    }
}

