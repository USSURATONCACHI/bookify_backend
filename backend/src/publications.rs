
use crate::db::DbPool;
use crate::models::{Publication, NewPublication, PublicationName};
use crate::schema::publications::dsl::*;
use diesel::prelude::*;
use actix_web::{web, HttpResponse, Responder};

pub fn config(cfg: &mut web::ServiceConfig, base_path: &str) {
    cfg.service(
        web::resource(format!("{}{}", base_path, "publications"))
            .route(web::post().to(create_publication))
            .route(web::get().to(get_all_publications)),
    )
    .service(
        web::resource(format!("{}{}", base_path, "publications/{id}"))
            .route(web::get().to(get_publication))
            .route(web::put().to(update_publication))
            .route(web::delete().to(delete_publication)),
    )
    .service(
        web::resource(format!("{}{}", base_path, "publications/brief/{id}"))
            .route(web::get().to(get_brief_publication))
    );
}


async fn get_all_publications(
    pool: web::Data<DbPool>
) -> impl Responder {
    let mut conn = pool.get().expect("couldn't get db connection from pool");

    let pubs =publications
        .select((id, name, filename))
        .load::<PublicationName>(&mut conn)
        .expect("Error loading publication names");

    HttpResponse::Ok().json(pubs)
}

async fn get_brief_publication(
    pool: web::Data<DbPool>,
    pub_id: web::Path<uuid::Uuid>,
) -> impl Responder {
    let mut conn = pool.get().expect("couldn't get db connection from pool");

    let publication = publications
        .filter(id.eq(pub_id.into_inner()))
        .select((id, name, filename))
        .first::<PublicationName>(&mut conn)
        .optional()
        .expect("Error loading publication");

    match publication {
        Some(publication) => HttpResponse::Ok().json(publication),
        None => HttpResponse::NotFound().finish(),
    }
}


async fn create_publication(
    pool: web::Data<DbPool>,
    publication_data: web::Json<NewPublication>,
) -> impl Responder {
    let mut conn = pool.get().expect("couldn't get db connection from pool");

    let new_publication = NewPublication {
        name: publication_data.name.clone(),
        file: publication_data.file.clone(),
        filename: publication_data.filename.clone(),
    };

    diesel::insert_into(publications)
        .values(&new_publication)
        .execute(&mut conn)
        .expect("Error saving new publication");

    HttpResponse::Created().finish()
}

async fn get_publication(
    pool: web::Data<DbPool>,
    pub_id: web::Path<uuid::Uuid>,
) -> impl Responder {
    let mut conn = pool.get().expect("couldn't get db connection from pool");

    let publication = publications
        .filter(id.eq(pub_id.into_inner()))
        .first::<Publication>(&mut conn)
        .optional()
        .expect("Error loading publication");

    match publication {
        Some(publication) => HttpResponse::Ok().json(publication),
        None => HttpResponse::NotFound().finish(),
    }
}

async fn update_publication(
    pool: web::Data<DbPool>,
    pub_id: web::Path<uuid::Uuid>,
    publication_data: web::Json<NewPublication>,
) -> impl Responder {
    let mut conn = pool.get().expect("couldn't get db connection from pool");

    diesel::update(publications.filter(id.eq(pub_id.into_inner())))
        .set((
            name.eq(&publication_data.name),
            file.eq(&publication_data.file),
        ))
        .execute(&mut conn)
        .expect("Error updating publication");

    HttpResponse::Ok().finish()
}

async fn delete_publication(
    pool: web::Data<DbPool>,
    pub_id: web::Path<uuid::Uuid>,
) -> impl Responder {
    let mut conn = pool.get().expect("couldn't get db connection from pool");

    diesel::delete(publications.filter(id.eq(pub_id.into_inner())))
        .execute(&mut conn)
        .expect("Error deleting publication");

    HttpResponse::NoContent().finish()
}
