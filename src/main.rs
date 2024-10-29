pub mod schema;
pub mod models;
pub mod db;

use actix_web::{web, App, HttpServer, HttpResponse, Responder};
use diesel::prelude::*;
use dotenv::dotenv;

use db::DbPool;
use models::{Publication, NewPublication};
use schema::publications::dsl::*;


#[actix_web::main]
async fn main() -> std::io::Result<()> {
    
    dotenv().ok();

    let pool = db::establish_connection();
    HttpServer::new(move || {
        App::new()
            .app_data(web::Data::new(pool.clone()))
            .route("/publications", web::get().to(get_all_publications))
            .route("/publications", web::post().to(create_publication))
            .route("/publications/{id}", web::get().to(get_publication))
            .route("/publications/{id}", web::put().to(update_publication))
            .route("/publications/{id}", web::delete().to(delete_publication))
    })
    .bind(("0.0.0.0", 8080))?
        .run()
        .await
}

async fn get_all_publications(
    pool: web::Data<DbPool>
) -> impl Responder {
    let mut conn = pool.get().expect("couldn't get db connection from pool");

    let pubs = publications
        .select(name)
        .load::<String>(&mut conn)
        .expect("Error loading publication names");

    HttpResponse::Ok().json(pubs)
}


async fn create_publication(
    pool: web::Data<DbPool>,
    publication_data: web::Json<NewPublication>,
) -> impl Responder {
    let mut conn = pool.get().expect("couldn't get db connection from pool");

    let new_publication = NewPublication {
        name: publication_data.name.clone(),
        file: publication_data.file.clone(),
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
