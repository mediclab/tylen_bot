use sea_orm_migration::{prelude::*, schema::*};

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .create_table(
                Table::create()
                    .table(Ban::Table)
                    .if_not_exists()
                    .col(pk_uuid(Ban::Uuid).default(Expr::cust("gen_random_uuid()")))
                    .col(big_integer(Ban::Uuid).not_null())
                    .col(string(Ban::Reason).null())
                    .col(timestamp(Ban::BannedAt).default(Expr::current_timestamp()))
                    .to_owned(),
            )
            .await?;

        manager
            .create_foreign_key(
                ForeignKey::create()
                    .name("ban_user_id_users_user_id_fkey")
                    .from(Ban::Table, Ban::UserId)
                    .to(Users::Table, Users::UserId)
                    .on_delete(ForeignKeyAction::Cascade)
                    .to_owned(),
            )
            .await
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager.drop_table(Table::drop().table(Ban::Table).to_owned()).await
    }
}

#[derive(DeriveIden)]
enum Ban {
    Table,
    Uuid,
    UserId,
    Reason,
    BannedAt,
}

#[derive(DeriveIden)]
enum Users {
    Table,
    UserId,
}
