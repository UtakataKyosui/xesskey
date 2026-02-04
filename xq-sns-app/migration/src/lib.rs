#![allow(elided_lifetimes_in_paths)]
#![allow(clippy::wildcard_imports)]
pub use sea_orm_migration::prelude::*;
mod m20220101_000001_users;
mod m20240205_000001_create_actors;
mod m20240205_000002_create_activities;
mod m20240205_000003_create_follows;

pub struct Migrator;

#[async_trait::async_trait]
impl MigratorTrait for Migrator {
    fn migrations() -> Vec<Box<dyn MigrationTrait>> {
        vec![
            Box::new(m20220101_000001_users::Migration),
            Box::new(m20240205_000001_create_actors::Migration),
            Box::new(m20240205_000002_create_activities::Migration),
            Box::new(m20240205_000003_create_follows::Migration),
            // inject-above (do not remove this comment)
        ]
    }
}
