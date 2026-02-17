use loco_rs::schema::*;
use sea_orm_migration::prelude::*;

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, m: &SchemaManager) -> Result<(), DbErr> {
        create_table(m, "books",
            &[
            
            ("id", ColType::PkAuto),
            
            ("book_name", ColType::StringNull),
            ("book_image", ColType::StringNull),
            ("book_title", ColType::StringNull),
            ("book_price", ColType::FloatNull),
            ],
            &[
            ("category_id", ""),
            ("author_id", ""),
            ]
        ).await
    }

    async fn down(&self, m: &SchemaManager) -> Result<(), DbErr> {
        drop_table(m, "books").await
    }
}
