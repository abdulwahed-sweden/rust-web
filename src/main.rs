use actix_files::Files;
use actix_web::{web, App, HttpResponse, HttpServer, Responder};
use tera::{Tera, Context};

fn tera_engine() -> Tera {
    // يحمل جميع ملفات القوالب داخل templates/
    Tera::new("templates/**/*").expect("init tera")
}

async fn index(tmpl: web::Data<Tera>) -> impl Responder {
    let mut ctx = Context::new();
    ctx.insert("title", "Rust Web AI");
    ctx.insert("active", "home"); // مهم لو بتلوّن رابط Home في النافبار
    let body = tmpl
        .render("index.html.tera", &ctx)
        .unwrap_or_else(|e| format!("Template error: {e}"));
    HttpResponse::Ok()
        .content_type("text/html; charset=utf-8")
        .body(body)
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let tera = tera_engine();

    HttpServer::new(move || {
        App::new()
            .app_data(web::Data::new(tera.clone()))
            // يقدّم ملفات الثيم على /assets  ↴  (تأكد أن مجلد static/assets موجود)
            .service(Files::new("/assets", "static/assets").prefer_utf8(true))
            // الصفحة الرئيسية
            .route("/", web::get().to(index))
    })
    .bind("127.0.0.1:8080")?
    .run()
    .await
}
