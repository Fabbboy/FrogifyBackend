#![allow(non_snake_case)]
use std::time::SystemTime;

use actix_web::{HttpResponse, post, Responder, web};
use bson::{DateTime, doc as bson_doc};
use serde::{Deserialize, Serialize};
use serde_json::json;

use crate::Router;
use crate::Router::Intern::Database::Checkers::{isMailValid, isPwdValid, isTeacher};
use crate::Router::Intern::Database::MongoClient::Mongo;

#[derive(Deserialize)]
pub(crate) struct RegisterRequest {
    username: Option<String>,
    usermail: Option<String>,
    password: Option<String>,
}

#[derive(Serialize)]
struct Response {
    #[allow(non_snake_case)]
    userId: String,
    #[allow(non_snake_case)]
    userToken: String,
    #[allow(non_snake_case)]
    tokenExpire: SystemTime,
    #[allow(non_snake_case)]
    currentTimestamp: SystemTime,
    success: bool,
}

#[post("/register")]
pub(crate) async fn register(
    data: web::Json<RegisterRequest>,
) -> impl Responder {
    if data.username.is_none() || data.usermail.is_none() || data.password.is_none() {
        return HttpResponse::BadRequest().json(json!({
            "success": false,
            "message": "Missing data",
        }));
    }

    let username = data.username.as_ref().unwrap();
    let usermail = data.usermail.as_ref().unwrap();
    let password = data.password.as_ref().unwrap();

    let currentDateTime = SystemTime::now();
    let tokenExpire = currentDateTime + std::time::Duration::from_secs(2592000);
    let userId = Router::Hash::generateHashAdv(usermail.to_string(), currentDateTime);
    let userToken = Router::Hash::generateHashRandom(password.to_string());

    let client = Mongo::new().await.unwrap();
    let collection = client.openCollection("Frogify", "Users").await.unwrap();

    let role = if isTeacher(usermail) { "teacher" } else { "student" };

    let doc = bson_doc! {
        "userId": userId.clone(),
        "userToken": userToken.clone(),
        "tokenExpire": DateTime::from(tokenExpire),
        "username": username.clone(),
        "usermail": usermail.clone(),
        "password": password.clone(),
        "currentTimestamp": DateTime::from(currentDateTime),
        "role": role,
    };
    let result;
    if client.doesUserExists(collection.clone(), usermail, username).await.unwrap() {
        return HttpResponse::BadRequest().json(json!({
            "success": false,
            "message": "User already exists (username or usermail)",
        }));
    }else if isMailValid(usermail) == false {
        return HttpResponse::BadRequest().json(json!({
            "success": false,
            "message": "Invalid usermail or no sbl email was provided",
        }));
    }else if username.len() < 3 {
        return HttpResponse::BadRequest().json(json!({
            "success": false,
            "message": "Username is too short",
        }));
    }else if username.len() > 20 {
        return HttpResponse::BadRequest().json(json!({
            "success": false,
            "message": "Username is too long",
        }));
    }else if isPwdValid(password) == false {
        return HttpResponse::BadRequest().json(json!({
            "success": false,
            "message": "Password is too weak (min 8 chars, max 32, no spaces)",
        }));
    } else if client.doesMailUserExists(collection.clone(), usermail).await.unwrap() {
        return HttpResponse::BadRequest().json(json!({
            "success": false,
            "message": "User already exist (usermail)",
        }));
    }else{
        result = collection.insert_one(doc, None).await
    }
    return match result {
        Ok(_) => {
            HttpResponse::Ok().json(Response {
                userId,
                userToken,
                tokenExpire,
                currentTimestamp: currentDateTime,
                success: true,
            })
        }
        Err(e) => {
            HttpResponse::InternalServerError().json(json!({
                "success": false,
                "message": e.to_string(),
            }))
        }
    };
}
