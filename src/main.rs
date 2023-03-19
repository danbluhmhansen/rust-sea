use std::{env, error::Error};

use entity::{character_effect, player_character};
use migration::DbErr;
use sea_orm::{ActiveModelTrait, Set, TransactionTrait};
use uuid::Uuid;

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    dotenvy::dotenv()?;

    let db = sea_orm::Database::connect(env::var("DATABASE_URL")?).await?;

    let (_char, _eff_str, _eff_dexx) = db
        .transaction::<_, (
            player_character::Model,
            character_effect::Model,
            character_effect::Model,
        ), DbErr>(|txn| {
            Box::pin(async move {
                let char_id = Uuid::new_v4();
                let char = player_character::ActiveModel {
                    id: Set(char_id),
                    name: Set("Foo".to_string()),
                    ..Default::default()
                };

                let eff_str = character_effect::ActiveModel {
                    character_id: Set(char_id),
                    key: Set("strength".to_string()),
                    value: Set(16),
                    ..Default::default()
                };
                let eff_dex = character_effect::ActiveModel {
                    character_id: Set(char_id),
                    key: Set("dexterity".to_string()),
                    value: Set(14),
                    ..Default::default()
                };

                let fut_char = char.insert(txn);
                let fut_eff_str = eff_str.insert(txn);
                let fut_eff_dex = eff_dex.insert(txn);

                Ok((fut_char.await?, fut_eff_str.await?, fut_eff_dex.await?))
            })
        })
        .await?;

    Ok(())
}
