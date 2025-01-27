//! `SeaORM` Entity, @generated by sea-orm-codegen 1.1.3

use sea_orm::entity::prelude::*;

#[derive(Clone, Debug, PartialEq, DeriveEntityModel, Eq)]
#[sea_orm(table_name = "environment_channels")]
pub struct Model {
    #[sea_orm(primary_key)]
    pub id: i32,
    pub environment_id: Option<Uuid>,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {
    #[sea_orm(
        belongs_to = "super::environment::Entity",
        from = "Column::EnvironmentId",
        to = "super::environment::Column::Id",
        on_update = "NoAction",
        on_delete = "NoAction"
    )]
    Environment3,
    #[sea_orm(
        belongs_to = "super::environment::Entity",
        from = "Column::EnvironmentId",
        to = "super::environment::Column::Id",
        on_update = "NoAction",
        on_delete = "NoAction"
    )]
    Environment2,
    #[sea_orm(
        belongs_to = "super::environment::Entity",
        from = "Column::EnvironmentId",
        to = "super::environment::Column::Id",
        on_update = "NoAction",
        on_delete = "NoAction"
    )]
    Environment1,
}

impl ActiveModelBehavior for ActiveModel {}
