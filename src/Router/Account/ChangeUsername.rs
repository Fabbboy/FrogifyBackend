#![allow(non_snake_case)]


use actix_web::{HttpResponse, post, Responder, web};
use bson::{doc as bson_doc};
use serde::{Deserialize, Serialize};
use serde_json::json;

use crate::Router::Intern::Database::MongoClient::Mongo;

#[derive(Deserialize)]
pub(crate) struct ChangeUsernameRequest {
    userId: Option<String>,
    newUsername: Option<String>,
    password: Option<String>,
}

#[derive(Serialize)]
pub(crate) struct ChangeUsernameResponse {
    success: bool,
    message: String,
}

#[post("/chngusrn")]
pub(crate) async fn changeUsername(
    data: web::Json<ChangeUsernameRequest>,
) -> impl Responder {
    if data.userId.is_none() || data.newUsername.is_none() || data.password.is_none() {
        return HttpResponse::BadRequest().json(json!({
            "success": false,
            "message": "Missing data",
        }));
    }

    let userId = data.userId.as_ref().unwrap();
    let newUsername = data.newUsername.as_ref().unwrap();
    let password = data.password.as_ref().unwrap();

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


    let collection_clone = collection.clone();
    if !client.checkPasswordFromId(collection_clone, password, userId).await.unwrap() {
        return HttpResponse::BadRequest().json(json!({
        "success": false,
        "message": "Wrong password",
    }));
    }

    //check if username is already taken
    let user = collection.clone().find_one(bson_doc! {
        "username": newUsername,
    }, None).await.unwrap();

    if user.is_some() {
        return HttpResponse::BadRequest().json(json!({
            "success": false,
            "message": "Username is already taken",
        }));
    }

    //update username
    collection.clone().update_one(bson_doc! {
        "userId": userId,
    }, bson_doc! {
        "$set": {
            "username": newUsername,
        }
    }, None).await.unwrap();

    HttpResponse::Ok().json(ChangeUsernameResponse {
        success: true,
        message: "Username changed".to_string(),
    })
}
