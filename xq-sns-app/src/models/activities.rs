pub use super::_entities::activities::{self, ActiveModel, Entity, Model};
use sea_orm::entity::prelude::*;

#[async_trait::async_trait]
impl ActiveModelBehavior for ActiveModel {
    // implement your business logic here
}
