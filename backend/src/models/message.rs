use sea_orm::entity::prelude::*;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, DeriveEntityModel, Serialize, Deserialize)]
#[sea_orm(table_name = "messages")]
pub struct Model {
    #[sea_orm(primary_key)]
    pub id: String,
    pub sender_id: String,
    pub receiver_id: String,
    pub content: String,
    pub message_type: String,
    pub timestamp: DateTime,
    pub is_read: bool,
    pub read_at: Option<DateTime>,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {
    #[sea_orm(
        belongs_to = "super::user::Entity",
        from = "Column::SenderId",
        to = "super::user::Column::Id"
    )]
    Sender,
    #[sea_orm(
        belongs_to = "super::user::Entity",
        from = "Column::ReceiverId",
        to = "super::user::Column::Id"
    )]
    Receiver,
}

impl ActiveModelBehavior for ActiveModel {}
