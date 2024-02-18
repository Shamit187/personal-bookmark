use actix_files as fs;
use actix_web::web;
use std::collections::HashMap;

pub fn frontend_routes(cfg: &mut web::ServiceConfig) {
    let mut directories = HashMap::new();
    directories.insert("/", "./static");
    directories.insert("/login", "./static/login");

    for (path, dir) in &directories {
        cfg.service(fs::Files::new(path, dir).index_file("index.html"));
    }
}