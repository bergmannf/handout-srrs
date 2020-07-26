use actix_files::Files;
use actix_web::{error, web, App, Error, HttpResponse, HttpRequest, HttpServer, Responder};
use comrak::{markdown_to_html, ComrakOptions};
use tera::Tera;

async fn handout(tmpl: web::Data<tera::Tera>) -> Result<HttpResponse, Error> {
    println!("Rendering handout");
    let mut ctx = tera::Context::new();
    let msg = r#"# Welcome to ShadowNET

Enjoy your stay - do not fuck up ◐ Chaos"#;
    let rendered = markdown_to_html(msg, &ComrakOptions::default());
    ctx.insert("content", &rendered);
    let s = tmpl.render("index.html", &ctx).map_err(|_| error::ErrorInternalServerError("Template Error"))?;
    println!("{}", s);
    Ok(HttpResponse::Ok().body(s))
}

#[actix_rt::main]
async fn main() -> std::io::Result<()> {
    let current_dir = std::env::current_dir().unwrap();
    println!("Current directory: {}", current_dir.to_str().unwrap());
    HttpServer::new(|| {
        let tera = Tera::new(concat!(env!("CARGO_MANIFEST_DIR"), "/static/templates/**/*")).unwrap();
        App::new()
            .data(tera)
            .service(web::resource("/handout/{name}")
                     .route(web::get().to(handout)))
            .service(Files::new("/", "./static/").index_file("index.html"))
    })
        .bind("127.0.0.1:8088")?
        .run()
        .await
}
