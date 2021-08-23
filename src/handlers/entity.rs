use crate::{models, utils::Id, Pool};
use actix_web::{delete, get, post, web, HttpResponse};
use std::convert::{TryFrom, TryInto};

#[get("/entities/{entity_id}")]
pub async fn get_entity(
    db: web::Data<Pool>,
    entity_id: web::Path<Id>,
) -> Result<HttpResponse, actix_web::Error> {
    Ok(HttpResponse::Ok()
        .json(models::entity::get(Pool::clone(&db), *entity_id).await?))
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
    Id::try_from(token).map_err(|e| actix_web::error::ErrorUnauthorized(e))?;
    Ok(HttpResponse::NoContent()
        .json(models::entity::remove(Pool::clone(&db), *entity_id).await?))
}
