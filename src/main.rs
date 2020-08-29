use actix_cors::Cors;
use actix_web::{web, App, HttpServer};

mod graphql;
// pub mod models;
// pub mod schema;

fn main() {
  #[cfg(debug_assertions)]
  dotenv::dotenv().ok();

  let port: u16 = std::env::var("PORT")
    .ok()
    .and_then(|p| p.parse().ok())
    .unwrap_or(3000);

  let addr = std::net::SocketAddr::from(([0, 0, 0, 0], port));

  // Start http server
  HttpServer::new(move || {
    App::new()
      .wrap(Cors::new())
      .configure(graphql::register)
      .default_service(web::to(|| "404"))
  })
  .bind(addr)
  .unwrap()
  .run()
  .unwrap();
}
