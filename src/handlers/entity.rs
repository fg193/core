use crate::{models, DbPool};
use actix_web::{delete, get, post, web, HttpResponse};
use std::error::Error;

#[get("/entities/{entity_id}")]
pub async fn get_entity(
    db: web::Data<DbPool>,
    entity_id: web::Path<i64>,
) -> Result<HttpResponse, Box<dyn Error>> {
    Ok(HttpResponse::Ok()
        .json(models::entity::get(DbPool::clone(&db), *entity_id).await?))
}

#[post("/entities/")]
pub async fn create_entity(
    db: web::Data<DbPool>,
    mut entity: web::Json<models::entity::Entity>,
) -> Result<HttpResponse, Box<dyn Error>> {
    models::entity::set_create_default_values(&mut entity);
    Ok(HttpResponse::Created()
        .json(models::entity::create(DbPool::clone(&db), &entity).await?))
}

#[delete("/entities/{entity_id}")]
pub async fn remove_entity(
    db: web::Data<DbPool>,
    entity_id: web::Path<i64>,
) -> Result<HttpResponse, Box<dyn Error>> {
    Ok(HttpResponse::NoContent()
        .json(models::entity::remove(DbPool::clone(&db), *entity_id).await?))
}
