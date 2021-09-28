use super::schema::entities;
use super::schema::i18ns;
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
#[belongs_to(Entity, foreign_key=avatar_entity)]
#[belongs_to(Entity, foreign_key=owner_entity)]
#[belongs_to(Entity, foreign_key=editor_entity)]
#[belongs_to(Entity, foreign_key=viewer_entity)]
#[belongs_to(Entity, foreign_key=author_entity)]
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
    db: crate::Pool,
    entity_id: Id,
) -> Result<Entity, Box<dyn Error>> {
    let conn = db.get().await?;
    Ok(entities::table.find(entity_id).first(&*conn)?)
}

pub async fn get_by_label(
    db: crate::Pool,
    label: &str,
    user_id: Id,
) -> Result<Vec<Id>, Box<dyn Error>> {
    let conn = db.get().await?;

    Ok(entities::table
        .select(entities::id)
        .inner_join(i18ns::table)
        .filter(entities::viewer_entity.eq(user_id))
        .filter(i18ns::value.ilike(format!("%{}%", label)))
        .load(&*conn)?)
}

pub async fn create(
    db: crate::Pool,
    entity: &Entity,
) -> Result<Id, Box<dyn Error>> {
    let conn = db.get().await?;

    Ok(diesel::insert_into(entities::table)
        .values(entity)
        .returning(entities::id)
        .get_result(&*conn)?)
}

pub async fn remove(
    db: crate::Pool,
    entity_id: Id,
) -> Result<bool, Box<dyn Error>> {
    let conn = db.get().await?;
    let entity: Option<Entity> =
        entities::table.find(entity_id).first(&*conn).optional()?;
    match entity {
        None => Ok(false),
        Some(entity) => {
            // todo: remove link ref
            diesel::delete(&entity).execute(&*conn)?;
            Ok(true)
        }
    }
}

#[cfg(test)]
mod test {
    use std::error::Error;

    #[test]
    fn test_entity() -> Result<(), Box<dyn Error>> {
        Ok(())
    }
}
