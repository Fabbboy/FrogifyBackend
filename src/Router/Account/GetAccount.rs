#![allow(non_snake_case)]

use actix_web::{HttpResponse, post, Responder, web};
use bson::{doc as bson_doc};
use serde::{Deserialize, Serialize};
use serde_json::json;

use crate::Router::Intern::Database::MongoClient::Mongo;

#[derive(Deserialize)]
pub(crate) struct GetAccountRequest {
    userId: Option<String>,
}

#[derive(Serialize)]
pub(crate) struct GetAccountResponse {
    success: bool,
    username: String,
    usermail: String,
    role: String,
    //postIds: Vec<String>,
}

#[post("/getacc")]
pub(crate) async fn getAccount(
    data: web::Json<GetAccountRequest>,
) -> impl Responder {
    if data.userId.is_none() {
        return HttpResponse::BadRequest().json(json!({
            "success": false,
            "message": "Missing data",
        }));
    }

    let client = Mongo::new().await.unwrap();
    let collection = client.openCollection("Frogify", "Users").await.unwrap();

    let user = collection.clone().find_one(bson_doc! {
        "userId": data.userId.as_ref().unwrap(),
    }, None).await.unwrap();

    if user.is_none() {
        return HttpResponse::BadRequest().json(json!({
            "success": false,
            "message": "User does not exist",
        }));
    }

    let user = user.unwrap();

    let username = user.get_str("username").unwrap();
    let usermail = user.get_str("usermail").unwrap();
    let teacher = user.get_str("role").unwrap();
    //let postIds = user.get_array("postIds").unwrap();

    HttpResponse::Ok().json(json!({
        "success": true,
        "username": username,
        "usermail": usermail,
        "role": teacher,
        //"postIds": postIds,
    }))

}