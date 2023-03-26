#![allow(non_snake_case)]
use actix_cors::Cors;
use actix_web::{App, HttpServer, middleware, web};
use actix_web::web::Data;
use tokio::sync::Mutex;

use Router::Auth::{Login::login, Register::register};
use crate::Router::Account::ChangePassword::changePassword;


mod Router;

struct AppData {}

#[actix_rt::main]
async fn main() -> std::io::Result<()> {
    std::env::set_var("RUST_LOG", "actix_web=trace");
    env_logger::init();
    let app_data = Data::new(Mutex::new(AppData {}));


    HttpServer::new(move || {
        let cors = Cors::default()
            .allowed_origin("http://localhost:8000")
            .allowed_methods(vec!["GET", "POST"])
            .allowed_headers(vec!["Content-Type", "X-Requested-With"])
            .allow_any_origin()
            .max_age(3600);

        App::new()
            .app_data(app_data.clone())
            .service(web::scope("/auth")
                .service(register)
                .service(login))
            .service(web::scope("/user")
                .service(changePassword))
            .wrap(cors) // Add CORS middleware to the app
            .wrap(middleware::Logger::default())
    })
        .bind("127.0.0.1:4499")?
        .run()
        .await
}
//3aca304bc2883b704ec32cafe2534ac90e25b5d20c206577d247a3e7ca276cc7
//e8e689e4d18e6d7cba358cabf7176d89086794876674259758fbe1d5a81b7f18