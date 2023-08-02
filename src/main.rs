use actix_web::{ web, App, HttpResponse, HttpServer, Responder };
use serde::Deserialize;

#[derive(Deserialize)]
struct RegisterBody {
    email: String,
    password: String,
}

struct AppState {
    app_name: String,
}

async fn register(data: web::Data<AppState>, body: web::Json<RegisterBody>) -> impl Responder {
    format!(
        "Welcome to {}! here is your credential {} {}",
        data.app_name,
        body.email,
        body.password
    )
}

async fn authenticate() -> impl Responder {
    HttpResponse::Ok().body("authenticated")
}

fn scoped_auth(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web
            ::resource("/register")
            .route(web::post().to(register))
            .route(web::get().to(authenticate))
    );
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    std::env::set_var("RUST_LOG", "debug");
    env_logger::init();
    HttpServer::new(|| {
        App::new()
            .app_data(
                web::Data::new(AppState {
                    app_name: String::from("Actix Web"),
                })
            )
            .service(web::scope("/auth").configure(scoped_auth))
    })
        .bind(("127.0.0.1", 3715))?
        .run().await
}
