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
pub(crate) struct deleteAccountRequest {
    userId: Option<String>,
    password: Option<String>,
}

#[derive(Serialize)]
pub(crate) struct deleteAccountResponse {
    success: bool,
    message: String,
}

#[post("/delacc")]
pub(crate) async fn deleteAccount(
    data: web::Json<deleteAccountRequest>,
    req: HttpRequest,
) -> impl Responder {
    if data.userId.is_none() || data.password.is_none() {
        return HttpResponse::BadRequest().json(json!({
            "success": false,
            "message": "Missing data",
        }));
    }

    let userId = data.userId.as_ref().unwrap();
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

    let user = user.unwrap();

    //check if password is correct
    if !isPwdValid(password) {
        return HttpResponse::BadRequest().json(json!({
            "success": false,
            "message": "Password is not valid",
        }));
    }

    let userPassword = user.get_str("password").unwrap();
    if userPassword != password {
        return HttpResponse::BadRequest().json(json!({
            "success": false,
            "message": "Password is not correct",
        }));
    }

    //delete user
    collection.delete_one(bson_doc! {
        "userId": userId,
    }, None).await.unwrap();

    //TODO delete all posts and comments

    HttpResponse::Ok().json(json!({
        "success": true,
        "message": "Account deleted",
    }))
}
//TODO: delete all posts