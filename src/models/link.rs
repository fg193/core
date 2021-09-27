use super::attr::Attr;
use super::entity::Entity;
use super::schema::links;
use crate::utils::Id;
use diesel::query_dsl::{QueryDsl, RunQueryDsl};
use diesel::ExpressionMethods;
use std::error::Error;

#[derive(
    diesel::Associations,
    diesel::Identifiable,
    diesel::Insertable,
    diesel::Queryable,
    serde::Serialize,
    serde::Deserialize,
)]
#[table_name = "links"]
#[belongs_to(Attr, foreign_key = "attr")]
#[belongs_to(Entity, foreign_key = "src_entity")]
pub struct Link {
    pub id: Id,
    pub attr: Id,
    pub src_entity: Id,
    pub dest_entity: Id,
    pub direct: bool,
    pub ref_count: i64,
}

pub async fn get(
    db: crate::Pool,
    attr_id: Id,
    src_entity_id: Id,
    dest_entity_id: Id,
) -> Result<Link, Box<dyn Error>> {
    let conn = db.get().await?;
    Ok(links::table
        .filter(links::attr.eq(&attr_id))
        .filter(links::src_entity.eq(&src_entity_id))
        .filter(links::dest_entity.eq(&dest_entity_id))
        .first(&*conn)?)
}

pub async fn create(
    db: crate::Pool,
    attr_id: Id,
    src_entity_id: Id,
    dest_entity_id: Id,
) -> Result<Id, Box<dyn Error>> {
    let conn = db.get().await?;

    // src -> dest
    let current = links::table
        .filter(links::attr.eq(&attr_id))
        .filter(links::src_entity.eq(&src_entity_id))
        .filter(links::dest_entity.eq(&dest_entity_id))
        .for_update()
        .first(&conn);

    Ok(diesel::insert_into(links::table)
        .values(current)
        .returning(links::id)
        .get_result(&*conn)?)
}
