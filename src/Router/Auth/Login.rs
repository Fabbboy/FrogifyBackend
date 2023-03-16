#[allow(non_snake_case)]
use std::time::SystemTime;

use actix_web::{HttpResponse, post, Responder, web};
use serde::{Deserialize, Serialize};
use serde_json::json;

use crate::Router;
use crate::Router::Intern::Database::Checkers::{isMailValid, isPwdValid};
use crate::Router::Intern::Database::MongoClient::Mongo;

#[derive(PartialEq, Clone, Deserialize, Serialize)]
pub(crate) enum LoginMethode {
    Default,
    UserToken,
}

#[derive(Deserialize)]
pub(crate) struct LoginRequest {
    method: LoginMethode,
    usermail: Option<String>,
    password: Option<String>,
    #[allow(non_snake_case)]
    userToken: Option<String>,
}

#[derive(Serialize)]
pub(crate) struct LoginResponse {
    role: String,
    success: bool,
    #[allow(non_snake_case)]
    userId: String,
    #[allow(non_snake_case)]
    userToken: String,
    #[allow(non_snake_case)]
    tokenExpire: SystemTime,
    #[allow(non_snake_case)]
    currentTimestamp: SystemTime,
}

#[post("/login")]
pub(crate) async fn login(
    data: web::Json<LoginRequest>
) -> impl Responder {
    if data.usermail.is_none() || data.password.is_none() && data.method == LoginMethode::Default {
        return HttpResponse::BadRequest().json(json!({
            "success": false,
            "message": "Missing data for default login",
        }));
    }

    if data.usermail.is_none() || data.userToken.is_none() && data.method == LoginMethode::UserToken {
        return HttpResponse::BadRequest().json(json!({
            "success": false,
            "message": "Missing data for userToken login",
        }));
    }



    let client = Mongo::new().await.unwrap();
    let collection = client.openCollection("Frogify", "Users").await.unwrap();

    if data.method == LoginMethode::Default {
        if client.doesMailUserExists(collection.clone(), data.usermail.as_ref().unwrap()).await.unwrap() == false {
            return HttpResponse::BadRequest().json(json!({
        "success": false,
        "message": "User not found",
    }));
        }


        if isPwdValid(data.password.as_ref().unwrap()) == false {
            return HttpResponse::BadRequest().json(json!({
                "success": false,
                "message": "Password is not valid",
            }));
        }

        if isMailValid(data.usermail.as_ref().unwrap()) == false {
            return HttpResponse::BadRequest().json(json!({
                "success": false,
                "message": "Usermail is not valid",
            }));
        }

        let role = client.getRole(collection.clone(), data.usermail.as_ref().unwrap()).await.unwrap();

        if client.checkPwd(collection.clone(), data.usermail.as_ref().unwrap(), data.password.as_ref().unwrap()).await.unwrap() == false {
            return HttpResponse::BadRequest().json(json!({
                "success": false,
                "message": "Password is not correct",
            }));
        }
        #[allow(non_snake_case)]
            let Newtoken;
        //if token is expired generate new token
        if client.isTokenExpired(collection.clone(), data.usermail.as_ref().unwrap()).await.unwrap() {
            Newtoken = Router::Hash::generateHashRandom(data.password.as_ref().unwrap().to_string());
            client.updateToken(collection.clone(), data.usermail.as_ref().unwrap(), &*Newtoken, SystemTime::now() + std::time::Duration::from_secs(60 * 60 * 24 * 7)).await.unwrap();
        }

        let token = client.getToken(collection.clone(), data.usermail.as_ref().unwrap()).await.unwrap();
        #[allow(non_snake_case)]
            let tokenExpire = client.getTokenExpire(collection.clone(), data.usermail.as_ref().unwrap()).await.unwrap();
        #[allow(non_snake_case)]
            let userId = client.getUserId(collection.clone(), data.usermail.as_ref().unwrap()).await.unwrap();

        let response = LoginResponse {
            role,
            success: true,
            userId,
            userToken: token,
            tokenExpire: SystemTime::from(tokenExpire),
            currentTimestamp: SystemTime::now(),
        };

        return HttpResponse::Ok().json(response);
    } else {
        if client.doesMailUserExists(collection.clone(), data.usermail.as_ref().unwrap()).await.unwrap() == false {
            return HttpResponse::BadRequest().json(json!({
        "success": false,
        "message": "User not found",
        }));
        }

        if isMailValid(data.usermail.as_ref().unwrap()) == false {
            return HttpResponse::BadRequest().json(json!({
                "success": false,
                "message": "Usermail is not valid",
            }));
        }

        let role = client.getRole(collection.clone(), data.usermail.as_ref().unwrap()).await.unwrap();
        #[allow(non_snake_case)]
            let tokenFromDb = client.getToken(collection.clone(), data.usermail.as_ref().unwrap()).await.unwrap();
        #[allow(non_snake_case)]
            let tokenExpire = client.getTokenExpire(collection.clone(), data.usermail.as_ref().unwrap()).await.unwrap();
        #[allow(non_snake_case)]
            let userId = client.getUserId(collection.clone(), data.usermail.as_ref().unwrap()).await.unwrap();

        if tokenFromDb != data.userToken.clone().unwrap() {
            return HttpResponse::BadRequest().json(json!({
                "success": false,
                "message": "Token is not correct",
            }));
        }

        if SystemTime::now() > SystemTime::from(tokenExpire) {
            return HttpResponse::BadRequest().json(json!({
                "success": false,
                "message": "Token is expired",
            }));
        }

        let response = LoginResponse {
            role,
            success: true,
            userId,
            userToken: tokenFromDb,
            tokenExpire: SystemTime::from(tokenExpire),
            currentTimestamp: SystemTime::now(),
        };

        return HttpResponse::Ok().json(response);
    }
}

