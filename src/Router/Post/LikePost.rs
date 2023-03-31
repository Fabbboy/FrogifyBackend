use actix_web::{HttpRequest, HttpResponse, post, Responder, web};
use bson::{doc as bson_doc};
use chrono::{DateTime, FixedOffset, Utc};
use serde::{Deserialize, Serialize};
use serde_json::json;

use crate::Router::FirebaseAccessPoint::deleteImage;
use crate::Router::Intern::Database::MongoClient::Mongo;

#[derive(Deserialize)]
pub(crate) struct LikePost {
    postId: Option<String>,
    userId: Option<String>,
}

#[derive(Serialize)]
pub(crate) struct LikePostResponse {
    success: bool,
    message: String,
}

#[post("/likepost")]
pub(crate) async fn likePost(
    req: HttpRequest,
    data: web::Json<LikePost>,
) -> Result<impl Responder, actix_web::Error> {
    if data.postId.is_none() || data.userId.is_none() {
        return Ok(HttpResponse::BadRequest().json(json!({
            "success": false,
            "message": "Missing data",
        })));
    }

    let postId = data.postId.clone().unwrap();
    let userId = data.userId.clone().unwrap();

    let mongo_client = Mongo::new().await.unwrap();
    let post_collection = mongo_client.openCollection("Frogify", "Posts").await.unwrap();
    let user_collection = mongo_client.openCollection("Frogify", "Users").await.unwrap();

    // Check if post exists
    let post = post_collection
        .find_one(
            bson_doc! {
                "postId": postId.clone(),
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

    // Check if user exists
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

    // Check if user already liked the post
    let post = post.unwrap();
    let likes = post.get_array("likes").unwrap();
    let mut already_liked = false;
    for like in likes {
        if like.as_str().unwrap() == userId.clone() {
            already_liked = true;
            break;
        }
    }

    if already_liked {
        return Ok(HttpResponse::BadRequest().json(json!({
            "success": false,
            "message": "User already liked the post",
        })));
    }

    // Update post
    let update_result = post_collection
        .update_one(
            bson_doc! {
                "postId": postId.clone(),
            },
            bson_doc! {
                "$push": {
                    "likes": userId.clone(),
                },
            },
            None,
        )
        .await.unwrap();

    if update_result.modified_count == 0 {
        return Ok(HttpResponse::BadRequest().json(json!({
            "success": false,
            "message": "Failed to update post",
        })));
    }

    // Update user
    let update_result = user_collection
        .update_one(
            bson_doc! {
                "userId": userId.clone(),
            },
            bson_doc! {
                "$push": {
                    "likedPosts": postId.clone(),
                },
            },
            None,
        )
        .await.unwrap();

    if update_result.modified_count == 0 {
        return Ok(HttpResponse::BadRequest().json(json!({
            "success": false,
            "message": "Failed to update user",
        })));
    }

    Ok(HttpResponse::Ok().json(json!({
        "success": true,
        "message": "Post liked",
    })))

}