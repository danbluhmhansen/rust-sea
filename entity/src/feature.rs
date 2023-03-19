//! `SeaORM` Entity. Generated by sea-orm-codegen 0.11.1

use sea_orm::entity::prelude::*;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, DeriveEntityModel, Eq, Serialize, Deserialize)]
#[sea_orm(table_name = "feature")]
pub struct Model {
    #[sea_orm(primary_key, auto_increment = false)]
    pub id: Uuid,
    pub name: String,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {
    #[sea_orm(has_many = "super::effect::Entity")]
    Effect,
}

impl Related<super::effect::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::Effect.def()
    }
}

impl Related<super::player_character::Entity> for Entity {
    fn to() -> RelationDef {
        super::character_feature::Relation::PlayerCharacter.def()
    }
    fn via() -> Option<RelationDef> {
        Some(super::character_feature::Relation::Feature.def().rev())
    }
}

impl ActiveModelBehavior for ActiveModel {}