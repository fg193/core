use crate::utils::Id;
use crate::{models, DbPool};
use actix_web::{delete, get, post, web, HttpResponse};
use std::error::Error;

#[get("/entities/{entity_id}")]
pub async fn get_entity(
    db: web::Data<DbPool>,
    entity_id: web::Path<Id>,
) -> Result<HttpResponse, Box<dyn Error>> {
    Ok(HttpResponse::Ok()
        .json(models::entity::get(DbPool::clone(&db), *entity_id).await?))
}

#[post("/entities/")]
pub async fn create_entity(
    db: web::Data<DbPool>,
    user_id: actix_identity::Identity,
) -> Result<HttpResponse, Box<dyn Error>> {
    let entity = models::entity::new(user_id.into());
    Ok(HttpResponse::Created()
        .json(models::entity::create(DbPool::clone(&db), &entity).await?))
}

#[delete("/entities/{entity_id}")]
pub async fn remove_entity(
    db: web::Data<DbPool>,
    entity_id: web::Path<Id>,
) -> Result<HttpResponse, Box<dyn Error>> {
    Ok(HttpResponse::NoContent()
        .json(models::entity::remove(DbPool::clone(&db), *entity_id).await?))
}
