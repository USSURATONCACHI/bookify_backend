
use crate::{db::DbPool, models::NewSource};
use crate::models::Source;
use crate::schema::sources::dsl::*;
use diesel::prelude::*;
use actix_web::{web, HttpResponse, Responder};
use ::uuid;

pub fn config(cfg: &mut web::ServiceConfig, base_path: &str) {
    cfg.service(
        web::resource(format!("{}{}", base_path, "sources"))
            .route(web::post().to(create))
            .route(web::get().to(get_all)),
    )
    .service(
        web::resource(format!("{}{}", base_path, "sources/{id}"))
            .route(web::get().to(get))
            .route(web::put().to(update))
            .route(web::delete().to(delete)),
    )
    ;
}


async fn get_all(
    pool: web::Data<DbPool>
) -> impl Responder {
    let mut conn = pool.get().expect("couldn't get db connection from pool");

    let list = sources
        .select((uuid, name))
        .load::<Source>(&mut conn)
        .expect("Error loading sources list");

    HttpResponse::Ok().json(list)
}

async fn create(
    pool: web::Data<DbPool>,
    publication_data: web::Json<NewSource>,
) -> impl Responder {
    let mut conn = pool.get().expect("couldn't get db connection from pool");

    let new_publication = NewSource {
        name: publication_data.name.clone(),
    };

    diesel::insert_into(sources)
        .values(&new_publication)
        .execute(&mut conn)
        .expect("Error saving new source");

    HttpResponse::Created().finish()
}

async fn get(
    pool: web::Data<DbPool>,
    pub_id: web::Path<uuid::Uuid>,
) -> impl Responder {
    let mut conn = pool.get().expect("couldn't get db connection from pool");

    let publication = sources
        .filter(uuid.eq(pub_id.into_inner()))
        .first::<Source>(&mut conn)
        .optional()
        .expect("Error loading source");

    match publication {
        Some(publication) => HttpResponse::Ok().json(publication),
        None => HttpResponse::NotFound().finish(),
    }
}

async fn update(
    pool: web::Data<DbPool>,
    pub_id: web::Path<uuid::Uuid>,
    publication_data: web::Json<NewSource>,
) -> impl Responder {
    let mut conn = pool.get().expect("couldn't get db connection from pool");

    diesel::update(sources.filter(uuid.eq(pub_id.into_inner())))
        .set((
            name.eq(&publication_data.name),
        ))
        .execute(&mut conn)
        .expect("Error updating source");

    HttpResponse::Ok().finish()
}

async fn delete(
    pool: web::Data<DbPool>,
    pub_id: web::Path<uuid::Uuid>,
) -> impl Responder {
    let mut conn = pool.get().expect("couldn't get db connection from pool");

    diesel::delete(sources.filter(uuid.eq(pub_id.into_inner())))
        .execute(&mut conn)
        .expect("Error deleting source");

    HttpResponse::NoContent().finish()
}
