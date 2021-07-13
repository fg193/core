use super::schema::attrs;

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
    pub id: i64,
    pub owner_entity: i64,
    pub editor_entity: i64,
    pub viewer_entity: i64,
    pub author_entity: i64,
    pub create_time: crate::data_types::Time,
    pub modify_time: crate::data_types::Time,
}
