use super::attr::Attr;
use super::entity::Entity;
use super::schema::links;
use crate::utils::Id;
use diesel::query_dsl::{QueryDsl, RunQueryDsl};
use diesel::{ExpressionMethods, OptionalExtension};
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

pub fn new(
    attr_id: Id,
    src_entity_id: Id,
    dest_entity_id: Id,
    direct: bool,
) -> Link {
    Link {
        id: Id::new(),
        attr: attr_id,
        src_entity: src_entity_id,
        dest_entity: dest_entity_id,
        direct: direct,
        ref_count: 1,
    }
}

pub async fn get_by_src_entity(
    db: crate::Pool,
    attr_id: Id,
    src_entity_id: Id,
) -> Result<Link, Box<dyn Error>> {
    let conn = db.get().await?;
    Ok(links::table
        .filter(links::attr.eq(&attr_id))
        .filter(links::src_entity.eq(&src_entity_id))
        .first(&conn)?)
}

pub async fn create(
    db: crate::Pool,
    attr_id: Id,
    src_entity_id: Id,
    dest_entity_id: Id,
) -> Result<bool, Box<dyn Error>> {
    let conn = db.get().await?;

    // 1. handle direct link

    // dest -> src
    let original_links_dest_to_src: Option<Link> = links::table
        .filter(links::attr.eq(&attr_id))
        .filter(links::src_entity.eq(&dest_entity_id))
        .filter(links::dest_entity.eq(&src_entity_id))
        .for_update()
        .first(&conn)
        .optional()?;

    // keep a direct acyclic graph
    if let Some(_) = original_links_dest_to_src {
        return Err(actix_web::error::ErrorLoopDetected(
            "hierarchy endless loop",
        )
        .into());
    }

    // src -> dest
    let original_link_src_to_dest: Option<Link> = links::table
        .filter(links::attr.eq(&attr_id))
        .filter(links::src_entity.eq(&src_entity_id))
        .filter(links::dest_entity.eq(&dest_entity_id))
        .for_update()
        .first(&conn)
        .optional()?;

    match original_link_src_to_dest {
        Some(link) => {
            // there was already a direct link
            if link.direct {
                return Ok(false);
            }

            // there were indirect links between the nodes
            diesel::update(&link)
                .set((
                    links::direct.eq(true),
                    links::ref_count.eq(links::ref_count + 1),
                ))
                .execute(&conn)?;
        }

        None => {
            // there was no link at all
            diesel::insert_into(links::table)
                .values(new(attr_id, src_entity_id, dest_entity_id, true))
                .execute(&conn)?;
        }
    };

    // 2. handle indirect links

    // * -> src
    let original_links_to_src: Vec<Id> = links::table
        .select(links::src_entity)
        .filter(links::attr.eq(&attr_id))
        .filter(links::dest_entity.eq(&src_entity_id))
        .get_results(&*conn)?;
    let proposed_links_to_dest = original_links_to_src
        .iter()
        .map(|descendant| new(attr_id, *descendant, dest_entity_id, false));

    // dest -> *
    let original_links_from_dest: Vec<Id> = links::table
        .select(links::dest_entity)
        .filter(links::attr.eq(&attr_id))
        .filter(links::src_entity.eq(&dest_entity_id))
        .get_results(&*conn)?;
    let proposed_links_from_src = original_links_from_dest
        .iter()
        .map(|ancestor| new(attr_id, src_entity_id, *ancestor, false));

    diesel::insert_into(links::table)
        .values(
            proposed_links_to_dest
                .chain(proposed_links_from_src)
                .collect::<Vec<Link>>(),
        )
        .on_conflict((links::attr, links::src_entity, links::dest_entity))
        .do_update()
        .set(links::ref_count.eq(links::ref_count + 1))
        .execute(&conn)?;

    Ok(true)
}

pub async fn remove(
    db: crate::Pool,
    attr_id: Id,
    src_entity_id: Id,
    dest_entity_id: Id,
) -> Result<bool, Box<dyn Error>> {
    Ok(false)
}
