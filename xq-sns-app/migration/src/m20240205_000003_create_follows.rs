use loco_rs::schema::table_auto_tz;
use sea_orm_migration::{prelude::*, schema::*};

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .create_table(
                table_auto_tz(Follows::Table)
                    .col(pk_uuid(Follows::Id))
                    .col(uuid(Follows::FollowerId))
                    .col(uuid(Follows::FollowingId))
                    .col(string(Follows::State)) // Pending, Accepted, Rejected
                    .foreign_key(
                        ForeignKey::create()
                            .name("fk-follows-follower")
                            .from(Follows::Table, Follows::FollowerId)
                            .to(Actors::Table, Actors::Id)
                            .on_delete(ForeignKeyAction::Cascade)
                            .on_update(ForeignKeyAction::Cascade),
                    )
                    .foreign_key(
                        ForeignKey::create()
                            .name("fk-follows-following")
                            .from(Follows::Table, Follows::FollowingId)
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
            .drop_table(Table::drop().table(Follows::Table).to_owned())
            .await
    }
}

#[derive(DeriveIden)]
enum Follows {
    Table,
    Id,
    FollowerId,
    FollowingId,
    State,
}

#[derive(DeriveIden)]
enum Actors {
    Table,
    Id,
}
