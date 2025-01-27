//! `SeaORM` Entity, @generated by sea-orm-codegen 1.1.3

use sea_orm::entity::prelude::*;

#[derive(Clone, Debug, PartialEq, DeriveEntityModel, Eq)]
#[sea_orm(table_name = "environment_users")]
pub struct Model {
    #[sea_orm(primary_key)]
    pub id: i32,
    pub application_id: Option<Uuid>,
    pub user_id: Option<Uuid>,
    pub role_id: Option<i32>,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {
    #[sea_orm(
        belongs_to = "super::environment_roles::Entity",
        from = "Column::RoleId",
        to = "super::environment_roles::Column::Id",
        on_update = "NoAction",
        on_delete = "NoAction"
    )]
    EnvironmentRoles3,
    #[sea_orm(
        belongs_to = "super::environment_roles::Entity",
        from = "Column::RoleId",
        to = "super::environment_roles::Column::Id",
        on_update = "NoAction",
        on_delete = "NoAction"
    )]
    EnvironmentRoles2,
    #[sea_orm(
        belongs_to = "super::environment_roles::Entity",
        from = "Column::RoleId",
        to = "super::environment_roles::Column::Id",
        on_update = "NoAction",
        on_delete = "NoAction"
    )]
    EnvironmentRoles1,
    #[sea_orm(
        belongs_to = "super::user::Entity",
        from = "Column::UserId",
        to = "super::user::Column::Id",
        on_update = "NoAction",
        on_delete = "NoAction"
    )]
    User3,
    #[sea_orm(
        belongs_to = "super::user::Entity",
        from = "Column::UserId",
        to = "super::user::Column::Id",
        on_update = "NoAction",
        on_delete = "NoAction"
    )]
    User2,
    #[sea_orm(
        belongs_to = "super::user::Entity",
        from = "Column::UserId",
        to = "super::user::Column::Id",
        on_update = "NoAction",
        on_delete = "NoAction"
    )]
    User1,
}

impl ActiveModelBehavior for ActiveModel {}
