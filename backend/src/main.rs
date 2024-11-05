pub mod schema;
pub mod models;
pub mod db;

pub mod digital_publications;
pub mod sources;

use actix_web::middleware::Logger;
use actix_web::HttpServer;
use actix_web::App;
use actix_web::web;

use dotenv::dotenv;
use log::info;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    dotenv().ok();
    env_logger::init();
    info!("Program starts.");

    let pool = db::establish_connection();
    HttpServer::new(move || {
        App::new()
            .app_data(web::Data::new(pool.clone()))
            .configure(|cfg| digital_publications::config(cfg, "/api/v1/"))
            .configure(|cfg| sources::config(cfg, "/api/v1/"))
            .service(actix_files::Files::new("/", "./static").show_files_listing())
            .wrap(Logger::default())
    })
    .bind(("0.0.0.0", 8081))?
        .run()
        .await
}
