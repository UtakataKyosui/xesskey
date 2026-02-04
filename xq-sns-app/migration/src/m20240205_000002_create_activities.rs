use loco_rs::schema::table_auto_tz;
use sea_orm_migration::{prelude::*, schema::*};

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .create_table(
                table_auto_tz(Activities::Table)
                    .col(pk_uuid(Activities::Id))
                    .col(string(Activities::Uri).unique_key())
                    .col(uuid(Activities::ActorId))
                    .col(string(Activities::Type))
                    .col(string(Activities::ObjectId))
                    .col(json(Activities::Payload))
                    .foreign_key(
                        ForeignKey::create()
                            .name("fk-activities-actors")
                            .from(Activities::Table, Activities::ActorId)
                            .to(Actors::Table, Actors::Id)
                            .on_delete(ForeignKeyAction::Cascade)
                            .on_update(ForeignKeyAction::Cascade),
                    )
                    .to_owned(),
            )
            .await
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_table(Table::drop().table(Activities::Table).to_owned())
            .await
    }
}

#[derive(DeriveIden)]
enum Activities {
    Table,
    Id,
    Uri,
    ActorId,
    Type,
    ObjectId,
    Payload,
}

#[derive(DeriveIden)]
enum Actors {
    Table,
    Id,
}
