#![allow(non_snake_case)]

use actix_web::{HttpResponse, post, Responder, web};
use bson::{doc as bson_doc};
use serde::{Deserialize, Serialize};
use serde_json::json;

use crate::Router::Intern::Database::MongoClient::Mongo;

#[derive(Deserialize)]
pub(crate) struct GetAccountRequest {
    userId: Option<String>,
}

#[derive(Serialize)]
pub(crate) struct GetAccountResponse {
    success: bool,
    username: String,
    usermail: String,
    role: String,
    profilePictureUrl: String,
    postIds: Vec<String>,
    postStreak: i32,
}
use chrono::{DateTime};

async fn calculatePostStreak(postIds: Vec<String>) -> i32 {
    let mut streak = 0;
    let mut lastPostDate = None;

    for postId in postIds {
        let client = Mongo::new().await.unwrap();
        let collection = client.openCollection("Frogify", "Posts").await.unwrap();

        let post = collection.clone().find_one(bson_doc! {
            "postId": postId,
        }, None).await.unwrap();

        if let Some(post) = post {
            if let Some(post_date_doc) = post.get_document("postDate").ok() {
                if let Ok(post_date_str) = post_date_doc.get_str("$date") {
                    if let Ok(post_date) = DateTime::parse_from_rfc3339(post_date_str) {
                        let post_timestamp = post_date.timestamp_millis();
                        if let Some(last_post_date) = lastPostDate {
                            if post_timestamp - last_post_date == 86400000 { //86400000
                                streak += 1;
                            } else {
                                break;
                            }
                        } else {
                            lastPostDate = Some(post_timestamp);
                            streak += 1;
                        }
                    }
                }
            }
        }
    }

    streak
}


#[post("/getacc")]
pub(crate) async fn getAccount(
    data: web::Json<GetAccountRequest>,
) -> impl Responder {
    if data.userId.is_none() {
        return HttpResponse::BadRequest().json(json!({
            "success": false,
            "message": "Missing data",
        }));
    }

    let client = Mongo::new().await.unwrap();
    let collection = client.openCollection("Frogify", "Users").await.unwrap();

    let user = collection.clone().find_one(bson_doc! {
        "userId": data.userId.as_ref().unwrap(),
    }, None).await.unwrap();

    if user.is_none() {
        return HttpResponse::BadRequest().json(json!({
            "success": false,
            "message": "User does not exist",
        }));
    }

    let user = user.unwrap();

    let username = user.get_str("username").unwrap();
    let usermail = user.get_str("usermail").unwrap();
    let teacher = user.get_str("role").unwrap();
    let tempVec = vec![];
    let postIds = user.get_array("posts").unwrap_or(&tempVec);
    let profilePictureUrl = user.get_str("profilePicture").unwrap();

    HttpResponse::Ok().json(json!({
        "success": true,
        "username": username,
        "usermail": usermail,
        "role": teacher,
        "profilePictureUrl": profilePictureUrl,
        "postIds": postIds,
        "postStreak": calculatePostStreak(postIds.iter().map(|x| x.as_str().unwrap().to_string()).collect()).await,
    }))

}

