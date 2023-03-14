use actix_web::{App, HttpResponse, HttpServer, Responder, post, web, get, middleware};
use serde::{Serialize, Deserialize};
use tokio::sync::Mutex;
use actix_cors::Cors;
use actix_web::web::Data;

mod Router;
use Router::Auth::{Register::register};



struct AppData {

}


#[actix_rt::main]
async fn main() -> std::io::Result<()> {
    std::env::set_var("RUST_LOG", "actix_web=trace");
    env_logger::init();
let test = "test".to_string();
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
            .service(web::scope("").service(register))
            .wrap(cors) // Add CORS middleware to the app
            .wrap(middleware::Logger::default())

    })
        .bind("127.0.0.1:4499")?
        .run()
        .await
}

