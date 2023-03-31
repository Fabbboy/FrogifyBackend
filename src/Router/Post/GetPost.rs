use actix_web::{HttpRequest, HttpResponse, post, Responder, web};
use bson::{doc as bson_doc};
use chrono::{DateTime, FixedOffset, Utc};
use serde::{Deserialize, Serialize};
use serde_json::json;

use crate::Router::FirebaseAccessPoint::deleteImage;
use crate::Router::Intern::Database::MongoClient::Mongo;

#[derive(Deserialize)]
pub(crate) struct GetPost {
    postId: Option<String>,
}

#[derive(Serialize)]
pub(crate) struct GetPostResponse {
    success: bool,
    postTitle: String,
    postContent: String,
    postImageUrl: String,
    post_date: String,
    userId: String,
    likes: i32,
}

#[post("/getpost")]
pub(crate) async fn getPost(
    req: HttpRequest,
    data: web::Json<GetPost>,
) -> Result<impl Responder, actix_web::Error> {
    if data.postId.is_none() {
        return Ok(HttpResponse::BadRequest().json(json!({
            "success": false,
            "message": "Missing data",
        })));
    }

    //check if post exists
    let mongo_client = Mongo::new().await.unwrap();
    let post_collection = mongo_client.openCollection("Frogify", "Posts").await.unwrap();
    let postId = data.postId.clone().unwrap();
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

    let post = post.unwrap();
    let postTitle = post.get_str("postTitle").unwrap();
    let postContent = post.get_str("postContent").unwrap();
    let postImageUrl = post.get_str("postImageUrl").unwrap();
    let post_date = post.get_datetime("postDate")
        .ok()
        .ok_or_else(|| {
            actix_web::error::ErrorBadRequest(json!({
            "success": false,
            "message": "Post date not found",
        }))
        })?;
    let post_date = DateTime::<Utc>::from(*post_date);
    let userId = post.get_str("userId").unwrap();
    let likes = post.get_i32("likes").unwrap(); // Updated this line

    let post_date = post_date.with_timezone(&FixedOffset::east(0));
    let post_date = post_date.format("%Y-%m-%d %H:%M:%S").to_string();

    let response = GetPostResponse {
        success: true,
        postTitle: postTitle.to_string(),
        postContent: postContent.to_string(),
        postImageUrl: postImageUrl.to_string(),
        post_date,
        userId: userId.to_string(),
        likes, // Removed .parse::<i32>().unwrap()
    };

    Ok(HttpResponse::Ok().json(json!({
        "success": true,
        "postTitle": response.postTitle,
        "postContent": response.postContent,
        "postImageUrl": response.postImageUrl,
        "post_date": response.post_date,
        "userId": response.userId,
        "likes": response.likes
    })))
}
