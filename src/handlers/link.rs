use crate::{models, utils::Id, Pool};
use actix_web::{delete, get, post, web, HttpResponse};
use std::convert::TryInto;

#[get("/entities/{src_entity_id}/links")]
pub async fn get_links_by_src_entity(
    db: web::Data<Pool>,
    src_entity_id: web::Path<Id>,
    token: actix_identity::Identity,
) -> Result<HttpResponse, actix_web::Error> {
    let user_id = token
        .try_into()
        .map_err(|e| actix_web::error::ErrorUnauthorized(e))?;

    let src_entity =
        models::entity::get(Pool::clone(&db), *src_entity_id).await?;
    if src_entity.viewer_entity != user_id {
        // todo: permission check
    }

    // todo
    Ok(HttpResponse::Ok().json(0))
}

#[post("/entities/{src_entity_id}/links/{attr_id}/{dest_entity_id}")]
pub async fn create_link(
    db: web::Data<Pool>,
    attr_id: web::Path<Id>,
    src_entity_id: web::Path<Id>,
    dest_entity_id: web::Path<Id>,
    token: actix_identity::Identity,
) -> Result<HttpResponse, actix_web::Error> {
    let user_id = token
        .try_into()
        .map_err(|e| actix_web::error::ErrorUnauthorized(e))?;

    let src_entity =
        models::entity::get(Pool::clone(&db), *src_entity_id).await?;
    if src_entity.editor_entity != user_id {
        // todo: permission check
    }

    let dest_entity =
        models::entity::get(Pool::clone(&db), *dest_entity_id).await?;
    if dest_entity.viewer_entity != user_id {
        // todo: permission check
    }

    Ok(HttpResponse::Created().json(
        models::link::create(
            Pool::clone(&db),
            *attr_id,
            *src_entity_id,
            *dest_entity_id,
        )
        .await?,
    ))
}

#[delete("/entities/{src_entity_id}/links/{attr_id}/{dest_entity_id}")]
pub async fn remove_link(
    db: web::Data<Pool>,
    attr_id: web::Path<Id>,
    src_entity_id: web::Path<Id>,
    dest_entity_id: web::Path<Id>,
    token: actix_identity::Identity,
) -> Result<HttpResponse, actix_web::Error> {
    let user_id = token
        .try_into()
        .map_err(|e| actix_web::error::ErrorUnauthorized(e))?;

    let src_entity =
        models::entity::get(Pool::clone(&db), *src_entity_id).await?;
    if src_entity.editor_entity != user_id {
        // todo: permission check
    }

    let dest_entity =
        models::entity::get(Pool::clone(&db), *dest_entity_id).await?;
    if dest_entity.viewer_entity != user_id {
        // todo: permission check
    }

    Ok(HttpResponse::NoContent().json(
        models::link::remove(
            Pool::clone(&db),
            *attr_id,
            *src_entity_id,
            *dest_entity_id,
        )
        .await?,
    ))
}
