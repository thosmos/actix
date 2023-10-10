use actix_web::{get, post, web, App, HttpResponse, HttpServer, Responder, HttpRequest, Error};
use std::env;
use actix_files::NamedFile;
use actix_files as fs;
use std::path::PathBuf;

async fn index(req: HttpRequest) -> Result<NamedFile, Error> {
    let path: PathBuf = req.match_info().query("filename").parse().unwrap();
    Ok(NamedFile::open(path)?)
}
#[get("/healthz")]
async fn healthz() -> impl Responder {
    HttpResponse::Ok().body("Alive")
}

#[get("/")]
async fn hello() -> impl Responder {
    HttpResponse::Ok().body("OK")
}

#[post("/echo")]
async fn echo(req_body: String) -> impl Responder {
    HttpResponse::Ok().body(req_body)
}

async fn manual_hello() -> impl Responder {
    HttpResponse::Ok().body("Hey there!")
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let port = env::var("PORT").expect("Missing port number");
    let port = port.parse::<u16>().expect("Invalid port given");

    HttpServer::new(|| {
        App::new()
            // .handler("/static", fs::Files::new(".", ()).unwrap())
            .service(healthz)
            //.service(hello)
            .service(echo)
            //.route("/{filename:index.*}", web::get().to(index))
            .route("/hey", web::get().to(manual_hello))
            .service(fs::Files::new("/", "../static").show_files_listing().index_file("index.html"))
    })
        .bind(("0.0.0.0", port))?
        .run()
        .await
}
