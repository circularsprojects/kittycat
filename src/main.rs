use kittycat::html::{html_versionstr, replace_files};

use actix_web::{middleware::Logger, App, HttpRequest, HttpResponse, HttpServer, Responder, get, post, web};
use actix_files::NamedFile;
use std::{io::Read, path::PathBuf, format};
use serde::{Deserialize};
use dotenvy::dotenv;
use std::env;

#[get("/")]
async fn index() -> impl Responder {
    HttpResponse::Ok().content_type("text/html").body(html_versionstr("web/index.html").await)
}

#[derive(Deserialize)]
struct ManagementQuery {
    path: Option<String>
}

#[get("/m")]
async fn management(query: web::Query<ManagementQuery>) -> impl Responder {
    let path = query.path.clone().unwrap_or_else(|| "".into());
    if (path.starts_with("/") || path.contains("..")) && !path.is_empty() {
        return HttpResponse::BadRequest().body("Invalid path");
    }
    HttpResponse::Ok().content_type("text/html").body(replace_files(html_versionstr("web/management.html").await, path).await)
}

async fn file_handler(req: HttpRequest) -> impl Responder {
    let serve_path = env::var("SERVE_PATH")
        .expect("SERVE_PATH variable must be set.");
    
    let path: PathBuf = PathBuf::from(serve_path).join(req.path().trim_start_matches('/'));
    println!("Requested path: {:?}", path);
    if path.is_dir() {
        return HttpResponse::NotFound().content_type("text/html").body(html_versionstr("web/index.html").await);
    }
    let _file = match NamedFile::open(&path) {
        Ok(file) => return file.into_response(&req),
        Err(_) => return HttpResponse::NotFound().content_type("text/html").body(html_versionstr("web/index.html").await),
    };
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    dotenv().ok();
    env_logger::init_from_env(env_logger::Env::new().default_filter_or("info"));
    HttpServer::new(|| {
        App::new()
            .wrap(Logger::default())
            .service(index)
            .service(management)
            .default_service(web::get().to(file_handler))
    }).workers(1)
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}