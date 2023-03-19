use sea_orm_migration::prelude::*;

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .create_table(
                Table::create()
                    .table(PlayerCharacter::Table)
                    .if_not_exists()
                    .col(
                        ColumnDef::new(PlayerCharacter::Id)
                            .uuid()
                            .not_null()
                            .default(PgFunc::get_random_uuid())
                            .primary_key(),
                    )
                    .col(ColumnDef::new(PlayerCharacter::Name).string().not_null())
                    .to_owned(),
            )
            .await?;

        manager
            .create_table(
                Table::create()
                    .table(CharacterEffect::Table)
                    .if_not_exists()
                    .col(
                        ColumnDef::new(CharacterEffect::Id)
                            .uuid()
                            .not_null()
                            .default(PgFunc::get_random_uuid())
                            .primary_key(),
                    )
                    .col(
                        ColumnDef::new(CharacterEffect::CharacterId)
                            .uuid()
                            .not_null(),
                    )
                    .foreign_key(
                        ForeignKey::create()
                            .from(CharacterEffect::Table, CharacterEffect::CharacterId)
                            .to(PlayerCharacter::Table, PlayerCharacter::Id)
                            .on_delete(ForeignKeyAction::Cascade)
                            .on_update(ForeignKeyAction::Cascade),
                    )
                    .col(ColumnDef::new(CharacterEffect::Key).string().not_null())
                    .col(ColumnDef::new(CharacterEffect::Value).integer().not_null())
                    .to_owned(),
            )
            .await?;

        Ok(())
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_table(Table::drop().table(CharacterEffect::Table).to_owned())
            .await?;

        manager
            .drop_table(Table::drop().table(PlayerCharacter::Table).to_owned())
            .await?;

        Ok(())
    }
}

#[derive(Iden)]
enum PlayerCharacter {
    Table,
    Id,
    Name,
}

#[derive(Iden)]
enum CharacterEffect {
    Table,
    Id,
    CharacterId,
    Key,
    Value,
}
