use sea_orm::{sea_query::extension::postgres::Type, EnumIter};
use sea_orm_migration::prelude::*;

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .create_table(
                Table::create()
                    .table(Feature::Table)
                    .if_not_exists()
                    .col(
                        ColumnDef::new(Feature::Id)
                            .uuid()
                            .not_null()
                            .default(PgFunc::get_random_uuid())
                            .primary_key(),
                    )
                    .col(ColumnDef::new(Feature::Name).text().not_null())
                    .to_owned(),
            )
            .await?;

        manager
            .create_table(
                Table::create()
                    .table(Effect::Table)
                    .if_not_exists()
                    .col(
                        ColumnDef::new(Effect::Id)
                            .uuid()
                            .not_null()
                            .default(PgFunc::get_random_uuid())
                            .primary_key(),
                    )
                    .col(ColumnDef::new(Effect::FeatureId).uuid().not_null())
                    .foreign_key(
                        ForeignKey::create()
                            .from(Effect::Table, Effect::FeatureId)
                            .to(Feature::Table, Feature::Id)
                            .on_delete(ForeignKeyAction::Cascade)
                            .on_update(ForeignKeyAction::Cascade),
                    )
                    .col(ColumnDef::new(Effect::Path).text().not_null())
                    .col(ColumnDef::new(Effect::Value).integer().not_null())
                    .to_owned(),
            )
            .await?;

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
                    .col(ColumnDef::new(PlayerCharacter::Name).text().not_null())
                    .to_owned(),
            )
            .await?;

        manager
            .create_type(
                Type::create()
                    .as_enum(CharacterFeatureEvent::Table)
                    .values([CharacterFeatureEvent::Added, CharacterFeatureEvent::Removed])
                    .to_owned(),
            )
            .await?;

        manager
            .create_table(
                Table::create()
                    .table(CharacterFeature::Table)
                    .if_not_exists()
                    .col(
                        ColumnDef::new(CharacterFeature::CharacterId)
                            .uuid()
                            .not_null(),
                    )
                    .foreign_key(
                        ForeignKey::create()
                            .from(CharacterFeature::Table, CharacterFeature::CharacterId)
                            .to(PlayerCharacter::Table, PlayerCharacter::Id)
                            .on_delete(ForeignKeyAction::Cascade)
                            .on_update(ForeignKeyAction::Cascade),
                    )
                    .col(
                        ColumnDef::new(CharacterFeature::FeatureId)
                            .uuid()
                            .not_null(),
                    )
                    .foreign_key(
                        ForeignKey::create()
                            .from(CharacterFeature::Table, CharacterFeature::FeatureId)
                            .to(Feature::Table, Feature::Id)
                            .on_delete(ForeignKeyAction::Cascade)
                            .on_update(ForeignKeyAction::Cascade),
                    )
                    .primary_key(
                        Index::create()
                            .col(CharacterFeature::CharacterId)
                            .col(CharacterFeature::FeatureId)
                            .primary(),
                    )
                    .col(
                        ColumnDef::new(CharacterFeature::TimeStamp)
                            .timestamp()
                            .not_null()
                            .extra("DEFAULT CURRENT_TIMESTAMP".to_string()),
                    )
                    .col(
                        ColumnDef::new(CharacterFeature::Event)
                            .enumeration(
                                CharacterFeatureEvent::Table,
                                [CharacterFeatureEvent::Added, CharacterFeatureEvent::Removed],
                            )
                            .not_null(),
                    )
                    .to_owned(),
            )
            .await?;

        Ok(())
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_table(Table::drop().table(CharacterFeature::Table).to_owned())
            .await?;

        manager
            .drop_type(Type::drop().name(CharacterFeatureEvent::Table).to_owned())
            .await?;

        manager
            .drop_table(Table::drop().table(PlayerCharacter::Table).to_owned())
            .await?;

        manager
            .drop_table(Table::drop().table(Effect::Table).to_owned())
            .await?;

        manager
            .drop_table(Table::drop().table(Feature::Table).to_owned())
            .await?;

        Ok(())
    }
}

#[derive(Iden)]
enum Feature {
    Table,
    Id,
    Name,
}

#[derive(Iden)]
enum Effect {
    Table,
    Id,
    FeatureId,
    Path,
    Value,
}

#[derive(Iden)]
enum PlayerCharacter {
    Table,
    Id,
    Name,
}

#[derive(Iden)]
enum CharacterFeature {
    Table,
    CharacterId,
    FeatureId,
    TimeStamp,
    Event,
}

#[derive(Iden, EnumIter)]
enum CharacterFeatureEvent {
    Table,
    Added,
    Removed,
}
