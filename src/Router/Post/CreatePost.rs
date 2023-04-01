use std::time::SystemTime;

use actix_web::{HttpResponse, post, Responder, web};
use bson::{doc as bson_doc};
use serde::{Deserialize};
use serde_json::json;
use chrono::{DateTime, Utc};


use crate::Router;
use crate::Router::Intern::Database::MongoClient::Mongo;

#[derive(Deserialize)]
pub(crate) struct CreatePostRequest {
    userId: String,
    postTitle: String,
    postContent: String,
    postImageUrl: Option<String>,
}

#[post("/createpost")]
pub(crate) async fn createPost(
    data: web::Json<CreatePostRequest>,
) -> Result<impl Responder, actix_web::Error> {
    let userId = data.userId.clone();
    let postTitle = data.postTitle.clone();
    let postContent = data.postContent.clone();
    let postImageUrl = data.postImageUrl.clone().unwrap_or_else(|| "".to_string());

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

    // Check if post already exists
    let post = post_collection
        .find_one(
            bson_doc! {
                "userId": userId.clone(),
                "posts.postTitle": postTitle.clone(),
            },
            None,
        )
        .await.unwrap();

    if post.is_some() {
        return Ok(HttpResponse::BadRequest().json(json!({
            "success": false,
            "message": "Post already exists",
        })));
    }

    // Generate post ID
    let current_date_time = SystemTime::now();
    let datetime: DateTime<Utc> = current_date_time.into();
    let post_id = Router::Hash::generateHashAdv(postTitle.clone().to_string(), current_date_time);

    // Update user's posts array with the new post ID
    let result = user_collection.update_one(
        bson_doc! {
            "userId": userId.clone(),
        },
        bson_doc! {
            "$push": {
                "posts": post_id.clone(),
            }
        },
        None,
    ).await.unwrap();

    if result.modified_count == 0 {
        return Ok(HttpResponse::BadRequest().json(json!({
            "success": false,
            "message": "Failed to create post",
        })));
    }

    // Create post document
    let post_doc = bson_doc! {
        "postId": post_id,
        "postTitle": postTitle.clone(),
        "postContent": postContent,
        "postImageUrl": postImageUrl,
        "postDate": bson_doc! {"$date": datetime.to_rfc3339()},
        "userId": userId.clone(),
        "likes": 0,
    };

    // Insert post document into Posts collection
    post_collection.insert_one(post_doc, None).await.unwrap();

    Ok(HttpResponse::Ok().json(json!({
        "success": true,
        "message": "Post created",
    })))
}
