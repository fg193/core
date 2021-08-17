use super::schema::attrs;
use crate::utils::Id;

#[derive(
    diesel::Associations,
    diesel::Identifiable,
    diesel::Insertable,
    diesel::Queryable,
    serde::Serialize,
    serde::Deserialize,
)]
#[table_name = "attrs"]
pub struct Attr {
    pub id: Id,
    pub owner_entity: Id,
    pub editor_entity: Id,
    pub viewer_entity: Id,
    pub author_entity: Id,
    pub create_time: crate::data_types::Time,
    pub modify_time: crate::data_types::Time,
}
