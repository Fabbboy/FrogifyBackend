#![allow(non_snake_case)]

use actix_web::{HttpResponse, post, Responder, web};
use bson::{Array, doc as bson_doc};
use serde::{Deserialize, Serialize};
use serde_json::json;

use crate::Router::Intern::Database::MongoClient::Mongo;

#[derive(Deserialize)]
pub(crate) struct GetTotalLikes{
    userId: Option<String>,
}

#[derive(Serialize)]
pub(crate) struct GetTotalLikesResponse {
    success: bool,
    message: String,
    totalLikes: i32,
}

#[post("/totallikes")]
pub(crate) async fn getTotalLikes(
    data: web::Json<GetTotalLikes>,
) -> Result<impl Responder, actix_web::Error> {
    if data.userId.is_none() {
        return Ok(HttpResponse::BadRequest().json(json!({
            "success": false,
            "message": "Missing data",
        })));
    }

    let userId = data.userId.clone().unwrap();

    let mongo_client = Mongo::new().await.unwrap();
    let user_collection = mongo_client.openCollection("Frogify", "Users").await.unwrap();
    let post_collection = mongo_client.openCollection("Frogify", "Posts").await.unwrap();

    //get array psotIds from user
    let user = user_collection
        .find_one(
            bson_doc! {
                "userId": userId.clone(),
            },
            None,
        )
        .await.unwrap();

    if user.is_none() {
        return Ok(HttpResponse::BadRequest().json(json!({
            "success": false,
            "message": "User does not exist",
        })));
    }

    let user = user.unwrap();
    let postIds = user.get_array("posts").unwrap();

    let mut totalLikes = 0;

    for postId in postIds {
        let post = post_collection
            .find_one(
                bson_doc! {
                    "postId": postId.as_str().unwrap(),
                },
                None,
            )
            .await.unwrap();

        if post.is_none() {
            return Ok(HttpResponse::BadRequest().json(json!({
                "success": false,
                "message": "Post does not exist",
            })));
        }

        let post = post.unwrap();
        let likes = match post.get_array("likedBy") {
            Ok(likes_array) => likes_array,
            Err(_) => {
                return Ok(HttpResponse::InternalServerError().json(json!({
            "success": false,
            "message": "An error occurred while retrieving the likes array",
        })));
            }
        };


        totalLikes += likes.len() as i32;
    }

    Ok(HttpResponse::Ok().json(json!({
        "success": true,
        "message": "Total likes",
        "totalLikes": totalLikes,
    })))
}


