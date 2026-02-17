#![allow(elided_lifetimes_in_paths)]
#![allow(clippy::wildcard_imports)]
pub use sea_orm_migration::prelude::*;
mod m20220101_000001_users;

mod m20260217_095610_categories;
mod m20260217_095755_authors;
mod m20260217_100427_books;
pub struct Migrator;

#[async_trait::async_trait]
impl MigratorTrait for Migrator {
    fn migrations() -> Vec<Box<dyn MigrationTrait>> {
        vec![
            Box::new(m20220101_000001_users::Migration),
            Box::new(m20260217_095610_categories::Migration),
            Box::new(m20260217_095755_authors::Migration),
            Box::new(m20260217_100427_books::Migration),
            // inject-above (do not remove this comment)
        ]
    }
}