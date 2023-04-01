#![allow(non_snake_case)]

use actix_web::{HttpResponse, post, Responder, web};
use bson::{doc as bson_doc};
use serde::{Deserialize, Serialize};
use serde_json::json;

use crate::Router::FirebaseAccessPoint::deleteImage;
use crate::Router::Intern::Database::Checkers::{isPwdValid};
use crate::Router::Intern::Database::MongoClient::Mongo;

#[derive(Deserialize)]
pub(crate) struct DeleteAccountRequest {
    userId: Option<String>,
    password: Option<String>,
}

#[derive(Serialize)]
pub(crate) struct DeleteAccountResponse {
    success: bool,
    message: String,
}

#[post("/delacc")]
pub(crate) async fn deleteAccount(
    data: web::Json<DeleteAccountRequest>,
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


    let postCol = client.openCollection("Frogify", "Posts").await.unwrap();

    let posts = user.get_array("posts").unwrap();


    for post_id in posts.iter().filter_map(|p| p.as_str()) {
        // Delete the post with the given ID
        let post_filter = bson_doc! {
        "postId": post_id,
    };
        let post_doc_opt = postCol.find_one(bson_doc! {
        "postId": post_id,
    }, None).await.unwrap();

        if let Some(post_doc) = post_doc_opt {
            let url = post_doc.get_str("postImageUrl").unwrap();
            deleteImage(url).await.unwrap();
            postCol.delete_one(post_filter, None).await.unwrap();
        }
    }



    collection.delete_one(bson_doc! {
    "userId": userId,
}, None).await.unwrap();

    HttpResponse::Ok().json(json!({
    "success": true,
    "message": "Account deleted",
}))
}