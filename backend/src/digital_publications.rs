
use crate::db::DbPool;
use crate::models::{DigitalPublication, NewDigitalPublication, Source};
use crate::schema::digital_publications::dsl::*;
use diesel::prelude::*;
use actix_web::{web, HttpResponse, Responder};

use ::uuid::Uuid;

pub fn config(cfg: &mut web::ServiceConfig, base_path: &str) {
    cfg.service(
        web::resource(format!("{}{}", base_path, "digital_publications"))
            .route(web::post().to(create))
            .route(web::get().to(get_all)),
    )
    .service(
        web::resource(format!("{}{}", base_path, "digital_publications/{id}"))
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

    let pubs = digital_publications
        .select((
            uuid,
            source,
            name,
            description, 
            cover_url, 
            links
        ))
        .load::<DigitalPublication>(&mut conn)
        .expect("Error loading digital publications");

    HttpResponse::Ok().json(pubs)
}

async fn create(
    pool: web::Data<DbPool>,
    publication_data: web::Json<NewDigitalPublication>,
) -> impl Responder {
    let mut conn = pool.get().expect("couldn't get db connection from pool");

    let new_publication = NewDigitalPublication {
        name:        publication_data.name.clone(),
        source:      publication_data.source.clone(),
        description: publication_data.description.clone(),
        cover_url:   publication_data.cover_url.clone(),
        links:       publication_data.links.clone(),
    };

    diesel::insert_into(digital_publications)
        .values(&new_publication)
        .execute(&mut conn)
        .expect("Error saving new digital publication");

    HttpResponse::Created().finish()
}

async fn get(
    pool: web::Data<DbPool>,
    pub_id: web::Path<Uuid>,
) -> impl Responder {
    let mut conn = pool.get().expect("couldn't get db connection from pool");

    let publication = digital_publications
        .filter(uuid.eq(pub_id.into_inner()))
        .first::<DigitalPublication>(&mut conn)
        .optional()
        .expect("Error loading digital publication");

    match publication {
        Some(publication) => HttpResponse::Ok().json(publication),
        None => HttpResponse::NotFound().finish(),
    }
}

async fn update(
    pool: web::Data<DbPool>,
    pub_id: web::Path<Uuid>,
    publication_data: web::Json<NewDigitalPublication>,
) -> impl Responder {
    let mut conn = pool.get().expect("couldn't get db connection from pool");

    diesel::update(digital_publications
        .filter(uuid.eq(pub_id.into_inner())))
        .set((
            name.eq(&publication_data.name),
            source.eq(&publication_data.source),
            description.eq(&publication_data.description),
            cover_url.eq(&publication_data.cover_url),
            links.eq(&publication_data.links),
        ))
        .execute(&mut conn)
        .expect("Error updating digital publication");

    HttpResponse::Ok().finish()
}

async fn delete(
    pool: web::Data<DbPool>,
    pub_id: web::Path<Uuid>,
) -> impl Responder {
    let mut conn = pool.get().expect("couldn't get db connection from pool");

    diesel::delete(
        digital_publications
            .filter(uuid.eq(pub_id.into_inner()))
    ).execute(&mut conn)
        .expect("Error deleting digital publication");

    HttpResponse::NoContent().finish()
}
