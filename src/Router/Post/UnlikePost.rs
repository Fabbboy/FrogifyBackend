use actix_web::{HttpRequest, HttpResponse, post, Responder, web};
use bson::{doc as bson_doc, doc};
use serde::{Deserialize, Serialize};
use serde_json::json;

use crate::Router::FirebaseAccessPoint::deleteImage;
use crate::Router::Intern::Database::MongoClient::Mongo;

#[derive(Deserialize)]
pub struct UnlikePost {
    postId: Option<String>,
    userId: Option<String>,
}

#[derive(Serialize)]
pub struct UnlikePostResponse {
    success: bool,
    message: String,
}

#[post("/unlikepost")]
pub async fn unlikePost(
    req: HttpRequest,
    data: web::Json<UnlikePost>,
) -> Result<impl Responder, actix_web::Error> {
    if data.postId.is_none() || data.userId.is_none() {
        return Ok(HttpResponse::BadRequest().json(json!({
            "success": false,
            "message": "Missing data",
        })));
    }

    let mongo_client = Mongo::new().await.unwrap();
    let post_collection = mongo_client.openCollection("Frogify", "Posts").await.unwrap();
    let post_id = data.postId.clone().unwrap();
    let user_id = data.userId.clone().unwrap();

    let filter = bson_doc! {"postId": post_id.clone()};
    let post = post_collection.find_one(filter.clone(), None).await.unwrap();

    if post.is_none() {
        return Ok(HttpResponse::BadRequest().json(json!({
            "success": false,
            "message": "Post does not exist",
        })));
    }

    let post_doc = post.unwrap();
    let mut likes = post_doc.get_i32("likes").unwrap();
    let liked_by = post_doc.get_array("likedBy")
        .map_or_else(|_| vec![], |array| array.to_owned());

    if let Some(position) = liked_by
        .iter()
        .position(|liked_by_doc| {
            if let Some(user_id_doc) = liked_by_doc.as_document() {
                match user_id_doc.get_str("userId") {
                    Ok(liked_by_user_id) => liked_by_user_id == user_id,
                    Err(_) => false,
                }
            } else {
                false
            }
        })
    {
        likes -= 1;
        let update = doc! {"$set": {"likes": likes}, "$pull": {"likedBy": liked_by[position].clone()}};
        post_collection.update_one(filter, update, None).await.unwrap();

        Ok(HttpResponse::Ok().json(json!({
            "success": true,
            "message": "Post unliked",
        })))
    } else {
        return Ok(HttpResponse::BadRequest().json(json!({
            "success": false,
            "message": "Post not liked",
        })));
    }
}
