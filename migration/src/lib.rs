pub use sea_orm_migration::prelude::*;

mod m20230917_175056_create_energizz_pi5_table;

pub struct Migrator;

#[async_trait::async_trait]
impl MigratorTrait for Migrator {
    fn migrations() -> Vec<Box<dyn MigrationTrait>> {
        vec![
            Box::new(m20230917_175056_create_energizz_pi5_table::Migration),
        ]
    }
}
