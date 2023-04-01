use actix_web::{HttpResponse, post, Responder, web};
use bson::{doc as bson_doc};
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

        // get the user that posted the post
        let user_filter = bson_doc! {
            "userId": userId.clone(),
        };

        let user_doc_opt = user_collection.find_one(user_filter, None).await.unwrap();

        if let Some(user_doc) = user_doc_opt {
            let user_posts = user_doc.get_array("posts").unwrap();
            let mut user_posts_vec = Vec::new();

            for post in user_posts {
                user_posts_vec.push(post.as_str().unwrap().to_string());
            }

            user_posts_vec.retain(|x| x != &postId);

            let update_res = user_collection.update_one(
                bson_doc! {
                    "userId": userId.clone(),
                },
                bson_doc! {
                    "$set": {
                        "posts": user_posts_vec,
                    }
                },
                None,
            ).await;

            if update_res.is_err() {
                return Ok(HttpResponse::BadRequest().json(json!({
                    "success": false,
                    "message": "Error while updating user posts",
                })));
            }
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
