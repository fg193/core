use crate::{models, utils::Id, Pool};
use actix_web::{delete, get, post, web, HttpResponse};
use std::convert::TryInto;

#[get("/entities/{entity_id}")]
pub async fn get_entity(
    db: web::Data<Pool>,
    token: actix_identity::Identity,
    entity_id: web::Path<Id>,
) -> Result<HttpResponse, actix_web::Error> {
    let user_id = token
        .try_into()
        .map_err(|e| actix_web::error::ErrorUnauthorized(e))?;

    let entity = models::entity::get(Pool::clone(&db), *entity_id).await?;
    if entity.viewer_entity != user_id {
        // todo: permission check
    }
    Ok(HttpResponse::Ok().json(entity))
}

#[get("/entities/match/{keyword}")]
pub async fn search_entites(
    db: web::Data<Pool>,
    token: actix_identity::Identity,
    keyword: web::Path<String>,
) -> Result<HttpResponse, actix_web::Error> {
    let user_id = token
        .try_into()
        .map_err(|e| actix_web::error::ErrorUnauthorized(e))?;

    Ok(HttpResponse::Ok().json(
        models::entity::get_by_label(Pool::clone(&db), &keyword, user_id)
            .await?,
    ))
}

#[post("/entities")]
pub async fn create_entity(
    db: web::Data<Pool>,
    token: actix_identity::Identity,
) -> Result<HttpResponse, actix_web::Error> {
    let user_id = token
        .try_into()
        .map_err(|e| actix_web::error::ErrorUnauthorized(e))?;

    let entity = models::entity::new(user_id);
    Ok(HttpResponse::Created()
        .json(models::entity::create(Pool::clone(&db), &entity).await?))
}

#[delete("/entities/{entity_id}")]
pub async fn remove_entity(
    db: web::Data<Pool>,
    token: actix_identity::Identity,
    entity_id: web::Path<Id>,
) -> Result<HttpResponse, actix_web::Error> {
    let user_id = token
        .try_into()
        .map_err(|e| actix_web::error::ErrorUnauthorized(e))?;

    let entity = models::entity::get(Pool::clone(&db), *entity_id).await?;
    if entity.owner_entity != user_id {
        // todo: permission check
    }

    Ok(HttpResponse::NoContent()
        .json(models::entity::remove(Pool::clone(&db), *entity_id).await?))
}
