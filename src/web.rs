use actix_files as fs;
use actix_web::{get, post, web, App, HttpResponse, HttpServer, Responder};

// #[get("/")]
// async fn hello() -> impl Responder {
//     HttpResponse::Ok().body("Hello world!")
// }

// #[post("/echo")]
// async fn echo(req_body: String) -> impl Responder {
//     HttpResponse::Ok().body(req_body)
// }

// async fn manual_hello() -> impl Responder {
//     HttpResponse::Ok().body("Hey there!")
// }

#[actix_web::main]
pub async fn start_http_server() -> std::io::Result<()> {
    // App::new().service(fs::Files::new("/*", "./web").index_file("index.html"))
    // App::new().service(spa().index_file("./web/index.html").finish())

    HttpServer::new(|| App::new().service(fs::Files::new("/*", "./web").index_file("index.html")))
        .bind(("127.0.0.1", 8080))?
        .run()
        .await
}

// // .service(hello)
// // .service(echo)
// // .route("/hey", web::get().to(manual_hello))
