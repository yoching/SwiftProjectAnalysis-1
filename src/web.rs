use actix_files as fs;
use actix_web::{App, HttpServer};
use std::time::Duration;

#[actix_web::main]
pub async fn start_http_server() -> std::io::Result<()> {
    let server = HttpServer::new(|| {
        let app = App::new().service(fs::Files::new("/*", "./web").index_file("index.html"));
        app
    })
    .bind(("127.0.0.1", 8080))?
    .run();

    _ = tokio::join!(server, open_browser());
    Ok(())
}

async fn open_browser() {
    tokio::time::sleep(Duration::from_secs(5)).await;
    if let Err(err) = open::that("http://127.0.0.1:8080/") {
        println!("faile to open browser: {}", err);
    }
}
