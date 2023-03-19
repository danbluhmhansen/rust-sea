use std::{env, error::Error};

use entity::{
    character_feature, effect, feature, player_character,
    sea_orm_active_enums::CharacterFeatureEvent,
};
use migration::DbErr;
use sea_orm::{ActiveModelTrait, Set, TransactionTrait};
use uuid::Uuid;

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    dotenvy::dotenv()?;

    let db = sea_orm::Database::connect(env::var("DATABASE_URL")?).await?;

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
