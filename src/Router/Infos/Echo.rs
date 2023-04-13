
use actix_web::{HttpResponse, get, Responder};
use serde_json::json;




#[get("/echo")]
pub(crate) async fn respEcho(

) -> Result<impl Responder, actix_web::Error> {
    Ok(HttpResponse::Ok().json(json!({
        "success": true,
        "message": "Successfully",
    })))
}