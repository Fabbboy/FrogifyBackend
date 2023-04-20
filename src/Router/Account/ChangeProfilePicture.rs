#![allow(non_snake_case)]

use actix_web::{HttpResponse, post, Responder, web};
use bson::{doc as bson_doc};
use serde::{Deserialize, Serialize};
use serde_json::json;

use crate::Router::Intern::Database::MongoClient::Mongo;

#[derive(Deserialize)]
pub(crate) struct ChangePPRequest{
    userId: Option<String>,
    profilePictureUrl: Option<String>,
}

#[derive(Serialize)]
pub(crate) struct ChangePPResponse {
    success: bool,
    message: String,
}

#[post("/changepp")]
pub(crate) async fn changePP(
    data: web::Json<ChangePPRequest>,
) -> impl Responder {
    if data.userId.is_none() || data.profilePictureUrl.is_none() {
        return HttpResponse::BadRequest().json(json!({
            "success": false,
            "message": "Missing data",
        }));
    }

    let userId = data.userId.as_ref().unwrap();
    let profilePictureUrl = data.profilePictureUrl.as_ref().unwrap();

    let client = Mongo::new().await.unwrap();
    let collection = client.openCollection("Frogify", "Users").await.unwrap();

    //check if user exists
    let user = collection.clone().find_one(bson_doc! {
        "userId": userId,
    }, None).await.unwrap();

    if user.is_none() {
        return HttpResponse::BadRequest().json(json!({
            "success": false,
            "message": "User does not exist",
        }));
    }


    let result = collection.clone().update_one(bson_doc! {
        "userId": userId,
    }, bson_doc! {
        "$set": {
            "profilePicture": profilePictureUrl,
        }
    }, None).await.unwrap();

    if result.modified_count == 0 {
        return HttpResponse::BadRequest().json(json!({
            "success": false,
            "message": "Could not change profile picture",
        }));
    }

    HttpResponse::Ok().json(json!({
        "success": true,
        "message": "Profile picture changed",
    }))

}






