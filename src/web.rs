use actix_files as fs;
use actix_web::{App, HttpServer};

#[actix_web::main]
pub async fn start_http_server() -> std::io::Result<()> {
    // App::new().service(fs::Files::new("/*", "./web").index_file("index.html"))
    // App::new().service(spa().index_file("./web/index.html").finish())

    HttpServer::new(|| App::new().service(fs::Files::new("/*", "./web").index_file("index.html")))
        .bind(("127.0.0.1", 8080))?
        .run()
        .await
}
