use log::{error, info};
use actix_web::{middleware::Logger, App, HttpRequest, HttpResponse, HttpServer, web};
use actix_files::NamedFile;
use std::{io::Read, path::PathBuf, env};
use dotenvy::dotenv;

const VERSION_STRING: &str = include_str!(concat!(env!("OUT_DIR"), "/version"));

async fn index() -> HttpResponse {
    let mut content = NamedFile::open_async("web/index.html").await
        .unwrap_or_else(|_| panic!("Failed to open index.html"));
    
    let mut buffer = String::new();
    content.read_to_string(&mut buffer).unwrap_or_else(|_| panic!("Failed to read index.html"));

    buffer = buffer.replace("{VERSION_STRING}", VERSION_STRING);

    HttpResponse::Ok().content_type("text/html").body(buffer)
}

async fn file_handler(req: HttpRequest) -> HttpResponse {
    let serve_path = env::var("SERVE_PATH")
        .expect("SERVE_PATH variable must be set.");
    
    let path: PathBuf = PathBuf::from(&serve_path).join(req.path().trim_start_matches('/').replace("%20", " "));
    info!("Received request for path: {}", path.display());
    let canonical = match path.canonicalize() {
        Ok(p) => p,
        Err(_) => return index().await,
    };
    if !canonical.starts_with(&serve_path) {
        error!("Canonical mismatch: {}", path.display());
        return index().await;
    }
    if path.is_dir() {
        return index().await;
    }
    let _file = match NamedFile::open(&path) {
        Ok(file) => return file.into_response(&req),
        Err(_) => return index().await,
    };
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    dotenv().ok();
    env_logger::init_from_env(env_logger::Env::new().default_filter_or("info"));
    info!("{}", VERSION_STRING);
    HttpServer::new(|| {
        App::new()
            .wrap(Logger::default())
            .route("/", web::get().to(index))
            .default_service(web::get().to(file_handler))
    }).workers(1)
    .bind((
        env::var("BIND_ADDR").unwrap_or_else(|_| "0.0.0.0".into()),
        env::var("BIND_PORT").unwrap_or_else(|_| "8080".into()).parse().unwrap_or(8080)
    ))?
    .run()
    .await
}