use actix_web::{HttpRequest, HttpResponse, post, Responder, web};
use bson::{doc as bson_doc};
use chrono::{DateTime, FixedOffset, Utc};
use serde::{Deserialize, Serialize};
use serde_json::json;
use futures_util::stream::StreamExt;

use crate::Router::FirebaseAccessPoint::deleteImage;
use crate::Router::Intern::Database::MongoClient::Mongo;
use crate::Router::Post::GetPost::{GetPostResponse, GetPost};

#[derive(Deserialize)]
pub(crate) struct GetAllPostsRequest {
    amount: Option<i32>,
}

#[derive(Serialize)]
pub(crate) struct GetAllPostsResponse {
    success: bool,
    posts: Vec<GetPostResponse>,
}

#[post("/getallposts")]
pub(crate) async fn getAllPosts(
    req: HttpRequest,
    data: web::Json<GetAllPostsRequest>,
) -> Result<impl Responder, actix_web::Error> {
    let mongo_client = Mongo::new().await.unwrap();
    let post_collection = mongo_client.openCollection("Frogify", "Posts").await.unwrap();
    let amount = data.amount.unwrap_or(10);

    let mut cursor = post_collection
        .find(None, None)
        .await.unwrap();

    let mut posts = Vec::new();

    while let Some(post) = cursor.next().await {
        let post = post.unwrap();
        let postTitle = post.get_str("postTitle").unwrap();
        let postContent = post.get_str("postContent").unwrap();
        let postImageUrl = post.get_str("postImageUrl").unwrap();
        let postId = post.get_str("postId").unwrap();
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
            postId: postId.to_string(),
            postTitle: postTitle.to_string(),
            postContent: postContent.to_string(),
            postImageUrl: postImageUrl.to_string(),
            post_date,
            userId: userId.to_string(),
            likes, // Removed .parse::<i32>().unwrap()
        };

        posts.push(response);

        if posts.len() >= amount as usize {
            break;
        }
    }

    Ok(HttpResponse::Ok().json(json!({
        "success": true,
        "posts": posts,
    })))
}
