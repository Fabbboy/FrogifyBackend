use actix_web::{HttpRequest, HttpResponse, post, Responder, web};
use bson::{doc as bson_doc};
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use serde_json::json;

use crate::Router::FirebaseAccessPoint::deleteImage;
use crate::Router::Intern::Database::MongoClient::Mongo;

#[derive(Deserialize)]
pub(crate) struct DeletePostRequest {
    userId: Option<String>,
    postId: Option<String>,
}

#[derive(Serialize)]
pub(crate) struct DeletePostResponse {
    success: bool,
    message: String,
}

#[post("/deletepost")]
pub(crate) async fn deletePost(
    req: HttpRequest,
    data: web::Json<DeletePostRequest>,
) -> Result<impl Responder, actix_web::Error> {
    if data.userId.is_none() || data.postId.is_none() {
        return Ok(HttpResponse::BadRequest().json(json!({
            "success": false,
            "message": "Missing data",
        })));
    }

    let userId = data.userId.clone().unwrap();
    let postId = data.postId.clone().unwrap();

    let mongo_client = Mongo::new().await.unwrap();
    let user_collection = mongo_client.openCollection("Frogify", "Users").await.unwrap();
    let post_collection = mongo_client.openCollection("Frogify", "Posts").await.unwrap();

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

    // Check if post exists
    let post_filter = bson_doc! {
        "postId": postId.clone(),
        "userId": userId.clone(),
    };
    let post_doc_opt = post_collection.find_one(post_filter.clone(), None).await.unwrap();

    if let Some(post_doc) = post_doc_opt {
        let post_image_url = post_doc.get_str("postImageUrl").unwrap();
        let del_post_res = deleteImage(post_image_url).await;

        if del_post_res.is_err() {
            return Ok(HttpResponse::BadRequest().json(json!({
                "success": false,
                "message": "Error while deleting post image",
            })));
        }

        let del_post_res = post_collection.delete_one(post_filter, None).await;

        if del_post_res.is_err() {
            return Ok(HttpResponse::BadRequest().json(json!({
                "success": false,
                "message": "Error while deleting post",
            })));
        }

        return Ok(HttpResponse::Ok().json(json!({
            "success": true,
            "message": "Post deleted",
        })));
    }

    return Ok(HttpResponse::BadRequest().json(json!({
        "success": false,
        "message": "Post does not exist",
    })));
}
