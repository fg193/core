use super::schema::entities;
use super::schema::entities::dsl::*;
use crate::utils::Id;
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
    pub id: Id,
    pub avatar_entity: Option<Id>,
    pub owner_entity: Id,
    pub editor_entity: Id,
    pub viewer_entity: Id,
    pub author_entity: Id,
    pub create_time: chrono::DateTime<chrono::Utc>,
    pub modify_time: chrono::DateTime<chrono::Utc>,
}

pub fn new(user_id: Id) -> Entity {
    let now = chrono::Utc::now();
    Entity {
        id: Id::new(),
        avatar_entity: None,
        owner_entity: user_id,
        editor_entity: user_id,
        viewer_entity: user_id,
        author_entity: user_id,
        create_time: now,
        modify_time: now,
    }
}

pub async fn get(
    db: crate::DbPool,
    entity_id: Id,
) -> Result<Entity, Box<dyn Error>> {
    let conn = db.get().await?;
    Ok(entities.find(entity_id).first(&*conn)?)
}

pub async fn create(
    db: crate::DbPool,
    entity: &Entity,
) -> Result<Id, Box<dyn Error>> {
    let conn = db.get().await?;

    Ok(diesel::insert_into(entities)
        .values(entity)
        .returning(id)
        .get_result(&*conn)?)
}

pub async fn remove(
    db: crate::DbPool,
    entity_id: Id,
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
