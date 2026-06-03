use actix_files::NamedFile;
use std::{env, fs, io::Read, path::PathBuf};

const VERSION_STRING: &str = include_str!(concat!(env!("OUT_DIR"), "/version"));

pub async fn html_versionstr(path: &str) -> String {
    let mut content = NamedFile::open_async(path).await
        .unwrap_or_else(|_| panic!("Failed to open {}", path));
    
    let mut buffer = String::new();
    content.read_to_string(&mut buffer).unwrap_or_else(|_| panic!("Failed to read {}", path));

    buffer = buffer.replace("{VERSION_STRING}", VERSION_STRING);

    buffer
}

pub async fn replace_version(html: String) -> String {
    html.replace("{VERSION_STRING}", VERSION_STRING)
}

pub async fn replace_files(html: String, path: String) -> String {
    let serve_path = env::var("SERVE_PATH")
        .expect("SERVE_PATH variable must be set.");

    let full_path = PathBuf::from(serve_path).join(&path);
    println!("Files for path: {:?}", full_path);
    let mut files = fs::read_dir(&full_path)
        .unwrap_or_else(|err| panic!("Failed to read directory ({full_path}): {err}", full_path = &full_path.display()));
    
    let mut files_list = String::new();
    while let Some(Ok(entry)) = files.next() {
        if let Ok(file_type) = entry.file_type() {
            if file_type.is_file() {
                files_list.push_str(&format!("<li><a href=\"{path}/{name}\">{name}</a> <button onclick=\"copy('{name}')\">Copy Link</button> <button onclick=\"deleteFile('{path}/{name}')\">Delete</button></li>", path = &path, name = entry.file_name().to_string_lossy()))
            } else if file_type.is_dir() {
                files_list.push_str(&format!("<li><a href=\"?path={path}{name}\">{name}</a></li>", path = if !&path.is_empty() { path.clone() + "/" } else { "".to_string() }, name = entry.file_name().to_string_lossy()))
            }
        } else {
            files_list.push_str(&format!("<li>{} (indeterminate type)</li>", entry.file_name().to_string_lossy()))
        }
    }

    html.replace("{FILES_LIST}", &files_list)
}