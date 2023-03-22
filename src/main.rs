use std::{env, error::Error};

use entity::{
    character_feature, effect, feature, player_character,
    sea_orm_active_enums::CharacterFeatureEvent,
};
use migration::DbErr;
use sea_orm::{
    ActiveModelTrait, ConnectionTrait, DatabaseBackend, Set, Statement, TransactionTrait,
};
use uuid::Uuid;

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    dotenvy::dotenv()?;

    let db = sea_orm::Database::connect(env::var("DATABASE_URL")?).await?;

    db.execute(Statement::from_string(
        DatabaseBackend::Postgres,
        r#"
            CREATE MATERIALIZED VIEW PLAYER_CHARACTER_AGGREGATE AS
            SELECT PLAYER_CHARACTER.ID,
            	PLAYER_CHARACTER."name",
            	SUM(CASE
            			WHEN EFFECT.PATH = 'strength' THEN EFFECT."value"
            			ELSE 0
            		END) STRENGTH
            FROM PLAYER_CHARACTER
            LEFT JOIN CHARACTER_FEATURE ON CHARACTER_FEATURE.CHARACTER_ID = PLAYER_CHARACTER.ID
            LEFT JOIN FEATURE ON FEATURE.ID = CHARACTER_FEATURE.FEATURE_ID
            LEFT JOIN EFFECT ON EFFECT.FEATURE_ID = FEATURE.ID
            GROUP BY PLAYER_CHARACTER.ID;
        "#
        .to_string(),
    ))
    .await?;

    db.transaction::<_, (), DbErr>(|txn| {
        Box::pin(async move {
            let feat_id = Uuid::new_v4();
            feature::ActiveModel {
                id: Set(feat_id),
                name: Set("Dwarf".to_string()),
                ..Default::default()
            }
            .insert(txn)
            .await?;

            effect::ActiveModel {
                feature_id: Set(feat_id),
                path: Set("strength".to_string()),
                value: Set(2),
                ..Default::default()
            }
            .insert(txn)
            .await?;

            let char_id = Uuid::new_v4();
            player_character::ActiveModel {
                id: Set(char_id),
                name: Set("Foo".to_string()),
                ..Default::default()
            }
            .insert(txn)
            .await?;

            character_feature::ActiveModel {
                character_id: Set(char_id),
                feature_id: Set(feat_id),
                event: Set(CharacterFeatureEvent::Added),
                ..Default::default()
            }
            .insert(txn)
            .await?;

            Ok(())
        })
    })
    .await?;

    Ok(())
}
