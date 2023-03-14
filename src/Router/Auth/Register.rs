use std::{io::{self, BufRead, Write}, sync::Mutex, time::SystemTime};
use std::net::TcpStream;

use actix_web::{post, web, HttpRequest, HttpResponse, Responder};
use bson::{doc as bson_doc, Bson, DateTime};
use serde::{Deserialize, Serialize};
use serde_json::json;

use crate::{AppData, Router};
use crate::Router::Database::MongoClient::Mongo;
use crate::Router::Database::Checkers::{isMailValid, isPwdValid};

#[derive(Deserialize)]
pub(crate) struct RegisterRequest {
    username: Option<String>,
    usermail: Option<String>,
    password: Option<String>,
}

#[derive(Serialize)]
struct Response {
    userId: String,
    userToken: String,
    tokenExpire: SystemTime,
    currentTimestamp: SystemTime,
    success: bool,
}

#[post("/auth/register")]
pub(crate) async fn register(
    data: web::Json<RegisterRequest>,
    req: HttpRequest,
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
    let collection = client.open_collection("Frogify", "Users").await.unwrap();

    let doc = bson_doc! {
        "userId": userId.clone(),
        "userToken": userToken.clone(),
        "tokenExpire": DateTime::from(tokenExpire),
        "username": username.clone(),
        "usermail": usermail.clone(),
        "password": password.clone(),
        "currentTimestamp": DateTime::from(currentDateTime),
    };
    let result;
    //let result = collection.insert_one(doc, None).await;
    if client.does_user_exists(collection.clone(), usermail, username).await.unwrap() {
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
    }else{
        result = collection.insert_one(doc, None).await
    }
    let errorMsg = match result {
        Ok(_) => {
            return HttpResponse::Ok().json(Response {
                userId: userId,
                userToken: userToken,
                tokenExpire: tokenExpire,
                currentTimestamp: currentDateTime,
                success: true,
            })
        }
        Err(e) => {
            return HttpResponse::InternalServerError().json(json!({
                "success": false,
                "message": e.to_string(),
            }))
        }
    };
}
