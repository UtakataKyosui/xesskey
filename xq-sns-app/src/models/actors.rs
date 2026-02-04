pub use super::_entities::actors::{self, ActiveModel, Column, Entity, Model};
use sea_orm::entity::prelude::*;

#[async_trait::async_trait]
impl ActiveModelBehavior for ActiveModel {
    // implement your business logic here
}
