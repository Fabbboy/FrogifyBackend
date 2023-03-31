use actix_web::{HttpRequest, HttpResponse, post, Responder, web};
use bson::{doc as bson_doc, doc};
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

    //check if user exists
    let user_collection = mongo_client.openCollection("Frogify", "Users").await.unwrap();
    let user_filter = bson_doc! {"userId": user_id.clone()};
    let user = user_collection.find_one(user_filter, None).await.unwrap();

    if user.is_none() {
        return Ok(HttpResponse::BadRequest().json(json!({
            "success": false,
            "message": "User does not exist",
        })));
    }

    let post_doc = post.unwrap();
    let mut likes = post_doc.get_i32("likes").unwrap();
    let liked_by = post_doc.get_array("likedBy")
        .map_or_else(|_| vec![], |array| array.to_owned());

    if liked_by
        .iter()
        .any(|liked_by_doc| {
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
        return Ok(HttpResponse::BadRequest().json(json!({
            "success": false,
            "message": "Post already liked",
        })));
    }

    likes += 1;
    let update = doc! {"$set": {"likes": likes}, "$push": {"likedBy": {"userId": user_id.clone()}}};
    post_collection.update_one(filter, update, None).await.unwrap();

    Ok(HttpResponse::Ok().json(json!({
        "success": true,
        "message": "Post liked",
    })))
}
