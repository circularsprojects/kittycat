use actix_web::{middleware::Logger, App, HttpRequest, HttpResponse, HttpServer, Responder, get, post, web};
use actix_files::NamedFile;
use std::{io::Read, path::PathBuf, format};

const VERSION_STRING: &str = include_str!(concat!(env!("OUT_DIR"), "/version"));

#[get("/")]
async fn index() -> impl Responder {
    HttpResponse::Ok().content_type("text/html").body(html_versionstr("web/index.html").await)
}

#[get("/m")]
async fn management() -> impl Responder {
    HttpResponse::Ok().content_type("text/html").body(html_versionstr("web/management.html").await)
}

async fn file_handler(req: HttpRequest) -> impl Responder {
    let path: PathBuf = req.path().trim_start_matches('/').parse().unwrap_or_else(|_| PathBuf::from("web/index.html"));
    if path.is_dir() {
        return HttpResponse::NotFound().content_type("text/html").body(html_versionstr("web/index.html").await);
    }
    let _file = match NamedFile::open(&path) {
        Ok(file) => return file.into_response(&req),
        Err(_) => return HttpResponse::NotFound().content_type("text/html").body(html_versionstr("web/index.html").await),
    };
}

async fn html_versionstr(path: &str) -> String {
    let mut content = NamedFile::open_async(path).await
        .unwrap_or_else(|_| panic!("Failed to open {}", path));
    
    let mut buffer = String::new();
    content.read_to_string(&mut buffer).unwrap_or_else(|_| panic!("Failed to read {}", path));

    buffer = buffer.replace("{VERSION_STRING}", VERSION_STRING);

    buffer
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    env_logger::init_from_env(env_logger::Env::new().default_filter_or("info"));
    HttpServer::new(|| {
        App::new()
            .wrap(Logger::default())
            .service(index)
            .service(management)
            .default_service(web::get().to(file_handler))
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}