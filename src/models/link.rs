use super::schema::links;

#[derive(
    diesel::Associations,
    diesel::Identifiable,
    diesel::Insertable,
    diesel::Queryable,
    serde::Serialize,
    serde::Deserialize,
)]
#[table_name = "links"]
pub struct Link {
    pub id: i64,
    pub attr: Option<i64>,
    pub src_entity: i64,
    pub dest_entity: i64,
    pub direct: bool,
    pub ref_count: i64,
}
