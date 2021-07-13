mod attr;
mod entity;
mod link;

pub fn api_routes(cfg: &mut actix_web::web::ServiceConfig) {
    cfg.service(
        actix_web::web::scope("/api")
            .service(entity::get_entity)
            .service(entity::create_entity)
            .service(entity::remove_entity),
    );
}
