use super::schema::links;
use crate::utils::Id;

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
    pub id: Id,
    pub attr: Option<Id>,
    pub src_entity: Id,
    pub dest_entity: Id,
    pub direct: bool,
    pub ref_count: i64,
}
