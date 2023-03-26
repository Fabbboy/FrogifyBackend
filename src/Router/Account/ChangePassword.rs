#![allow(non_snake_case)]

use std::time::SystemTime;

use actix_web::{HttpRequest, HttpResponse, post, Responder, web};
use bson::{DateTime, doc as bson_doc};
use serde::{Deserialize, Serialize};
use serde_json::json;

use crate::Router;
use crate::Router::Intern::Database::Checkers::{isMailValid, isPwdValid, isTeacher};
use crate::Router::Intern::Database::MongoClient::Mongo;

#[derive(Deserialize)]
pub(crate) struct ChangePasswordRequest {
    userId: Option<String>,
    oldPassword: Option<String>,
    newPassword: Option<String>,
}

#[derive(Serialize)]
pub(crate) struct ChangePasswordResponse {
    success: bool,
    newToken: String,
    message: String,
}

#[post("/chngpwd")]
pub(crate) async fn changePassword(
    data: web::Json<ChangePasswordRequest>,
    req: HttpRequest,
) -> impl Responder {
    if data.userId.is_none() || data.oldPassword.is_none() || data.newPassword.is_none() {
        return HttpResponse::BadRequest().json(json!({
            "success": false,
            "message": "Missing data",
        }));
    }

    let userId = data.userId.as_ref().unwrap();
    let oldPassword = data.oldPassword.as_ref().unwrap();
    let newPassword = data.newPassword.as_ref().unwrap();

    let client = Mongo::new().await.unwrap();
    let collection = client.openCollection("Frogify", "Users").await.unwrap();

    //check if password is valid
    if !isPwdValid(newPassword) {
        return HttpResponse::BadRequest().json(json!({
            "success": false,
            "message": "Password is not valid",
        }));
    }

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

    let user = user.unwrap();

    //check if old password is correct
    let collection_clone = collection.clone();
    if !client.checkPasswordFromId(collection_clone.clone(), oldPassword, userId).await.unwrap() {
        return HttpResponse::BadRequest().json(json!({
            "success": false,
            "message": "Old password is not correct",
        }));
    }

    //change password
    let newToken = Router::Hash::generateHashRandom(newPassword.to_string());
    let update_result = collection.clone().update_one(bson_doc! {
        "userId": userId,
    }, bson_doc! {
        "$set": {
            "password": newPassword.clone(),
            "userToken": newToken.clone(),
            "updatedAt": DateTime::from(SystemTime::now()),
        }
    }, None).await.unwrap();

    if update_result.modified_count == 0 {
        return HttpResponse::BadRequest().json(json!({
            "success": false,
            "message": "Could not change password",
        }));
    }

    HttpResponse::Ok().json(ChangePasswordResponse {
        success: true,
        newToken,
        message: "Password changed".to_string(),
    })
}
//6.43%