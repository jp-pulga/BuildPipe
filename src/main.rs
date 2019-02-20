use actix_web::{http, middleware, server, App, HttpRequest, HttpResponse, Result};
use env_logger;
use log;

struct AppState {}

fn index(req: &HttpRequest) -> Result<HttpResponse> {
    Ok(HttpResponse::Ok().content_type("text/html").body("Hello"))
}

fn main() {
    ::std::env::set_var("RUST_LOG", "actix_web=info");
    env_logger::init();

    log::info!("Initialalizng actix web...");

    server::new(move || {
        App::new()
            .middleware(middleware::Logger::default())
            .resource("/", |r| r.method(http::Method::GET).f(index))
    })
    .bind("127.0.0.1:8080")
    .expect("Could not bind to port 8080")
    .run();
}
