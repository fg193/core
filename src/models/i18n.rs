use super::schema::i18ns;

#[derive(
    diesel::Associations,
    diesel::Identifiable,
    diesel::Insertable,
    diesel::Queryable,
    serde::Serialize,
    serde::Deserialize,
)]
#[table_name = "i18ns"]
pub struct I18n {
    pub id: i64,
    pub entity: i64,
    pub attr: i64,
    pub lang: String,
    pub value: String,
}
