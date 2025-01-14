use sea_orm_migration::{prelude::*, schema::*};

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .create_table(
                Table::create()
                    .table(Memes::Table)
                    .if_not_exists()
                    .col(pk_uuid(Memes::Uuid).default(Expr::cust("gen_random_uuid()")))
                    .col(big_integer(Memes::UserId).not_null())
                    .col(big_integer(Memes::MsgId).null())
                    .col(string(Memes::PhotoId).not_null())
                    .col(boolean(Memes::IsApproved).not_null().default(false))
                    .col(big_integer(Memes::ChannelMsgId).null())
                    .col(timestamp(Memes::CreatedAt).default(Expr::current_timestamp()))
                    .col(timestamp(Memes::PostedAt).null())
                    .to_owned(),
            )
            .await?;

        manager
            .create_foreign_key(
                ForeignKey::create()
                    .name("memes_user_id_users_user_id_fkey")
                    .from(Memes::Table, Memes::UserId)
                    .to(Users::Table, Users::UserId)
                    .on_delete(ForeignKeyAction::Cascade)
                    .to_owned(),
            )
            .await
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager.drop_table(Table::drop().table(Memes::Table).to_owned()).await
    }
}

#[derive(DeriveIden)]
enum Memes {
    Table,
    Uuid,
    UserId,
    MsgId,
    PhotoId,
    IsApproved,
    ChannelMsgId,
    CreatedAt,
    PostedAt,
}

#[derive(DeriveIden)]
enum Users {
    Table,
    UserId,
}
