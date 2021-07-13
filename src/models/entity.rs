use super::schema::entities;
use super::schema::entities::dsl::*;
use diesel::prelude::*;
use std::error::Error;

#[derive(
    diesel::Associations,
    diesel::Identifiable,
    diesel::Insertable,
    diesel::Queryable,
    serde::Serialize,
    serde::Deserialize,
)]
#[table_name = "entities"]
pub struct Entity {
    pub id: i64,
    pub avatar_entity: Option<i64>,
    pub owner_entity: i64,
    pub editor_entity: i64,
    pub viewer_entity: i64,
    pub author_entity: i64,
    pub create_time: chrono::DateTime<chrono::Utc>,
    pub modify_time: chrono::DateTime<chrono::Utc>,
}

pub async fn get(
    db: crate::DbPool,
    entity_id: i64,
) -> Result<Entity, Box<dyn Error>> {
    let conn = db.get().await?;
    Ok(entities.find(entity_id).first(&*conn)?)
}

pub async fn create(
    db: crate::DbPool,
    entity: &Entity,
) -> Result<i64, Box<dyn Error>> {
    let conn = db.get().await?;

    Ok(diesel::insert_into(entities)
        .values(entity)
        .returning(id)
        .get_result(&*conn)?)
}

pub async fn remove(
    db: crate::DbPool,
    entity_id: i64,
) -> Result<Entity, Box<dyn Error>> {
    let conn = db.get().await?;
    Ok(entities.find(entity_id).first(&*conn)?)
}

#[cfg(test)]
mod test {
    use std::error::Error;

    #[test]
    fn test_entity() -> Result<(), Box<dyn Error>> {
        let time: crate::data_types::Time =
            chrono::DateTime::parse_from_rfc2822(
                "Mon, 02 Jan 2006 15:04:05 -0700",
            )?;

        let ret = serde_json::to_string(&time)?;
        assert_eq!(ret, r#""2006-01-02T15:04:05-07:00""#);
        Ok(())
    }
}
