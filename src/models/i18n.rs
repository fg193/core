use super::schema::i18ns;
use crate::utils::Id;

#[derive(
    diesel::Associations,
    diesel::Identifiable,
    diesel::Insertable,
    diesel::Queryable,
    serde::Serialize,
    serde::Deserialize,
)]
#[table_name = "i18ns"]
#[belongs_to(Entity, foreign_key=entity)]
#[belongs_to(Attr, foreign_key=attr)]
pub struct I18n {
    pub id: Id,
    pub entity: Id,
    pub attr: Id,
    pub lang: String,
    pub value: String,
}
