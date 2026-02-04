use loco_rs::schema::table_auto_tz;
use sea_orm_migration::{prelude::*, schema::*};

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .create_table(
                table_auto_tz(Actors::Table)
                    .col(pk_uuid(Actors::Id))
                    .col(string(Actors::Uri).unique_key())
                    .col(integer_null(Actors::UserId))
                    .col(string(Actors::Username))
                    .col(string(Actors::Domain))
                    .col(string(Actors::InboxUrl))
                    .col(string(Actors::OutboxUrl))
                    .col(text(Actors::PublicKey))
                    .col(text_null(Actors::PrivateKey))
                    .foreign_key(
                        ForeignKey::create()
                            .name("fk-actors-users")
                            .from(Actors::Table, Actors::UserId)
                            .to(Users::Table, Users::Id)
                            .on_delete(ForeignKeyAction::Cascade)
                            .on_update(ForeignKeyAction::Cascade),
                    )
                    .to_owned(),
            )
            .await
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_table(Table::drop().table(Actors::Table).to_owned())
            .await
    }
}

#[derive(DeriveIden)]
pub enum Actors {
    Table,
    Id,
    Uri,
    UserId,
    Username,
    Domain,
    InboxUrl,
    OutboxUrl,
    PublicKey,
    PrivateKey,
}

#[derive(DeriveIden)]
enum Users {
    Table,
    Id,
}
