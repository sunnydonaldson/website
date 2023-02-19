use actix_web::{get, web, App, HttpResponse, HttpServer, Responder};
use actix_files as fs;
use tera::{Tera, Context};

#[get("/")]
async fn hello(data: web::Data<AppData>) -> impl Responder {
    HttpResponse::Ok().body(
        data.tmpl.render("index.html", &Context::new()).unwrap()
    )
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| {
        App::new()
            .service(hello)
            .service(fs::Files::new("/static", "./static").show_files_listing())
            .app_data(
                web::Data::new(AppData {
                    tmpl: Tera::new("templates/**/*.html").unwrap()
                })
            )
    })
    .bind(("127.0.01", 8080))?
    .run()
    .await
}

struct AppData {
    tmpl: Tera
}
