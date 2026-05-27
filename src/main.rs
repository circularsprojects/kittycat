use actix_web::{middleware::Logger, App, HttpRequest, HttpResponse, HttpServer, Responder, get, post, web};
use actix_files::NamedFile;
use std::path::PathBuf;

#[get("/")]
async fn index() -> actix_web::Result<NamedFile> {
    Ok(NamedFile::open("index.html").expect("Failed to open index.html"))
}

async fn file_handler(req: HttpRequest) -> actix_web::Result<NamedFile> {
    let path: PathBuf = req.path().trim_start_matches('/').parse().unwrap_or_else(|_| PathBuf::from("index.html"));
    if path.is_dir() {
        return Ok(NamedFile::open("index.html")?);
    }
    let _file = match NamedFile::open(&path) {
        Ok(file) => return Ok(file),
        Err(_) => return Ok(NamedFile::open("index.html")?),
    };
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    env_logger::init_from_env(env_logger::Env::new().default_filter_or("info"));
    HttpServer::new(|| {
        App::new()
            .wrap(Logger::default())
            .service(index)
            .default_service(web::get().to(file_handler))
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}