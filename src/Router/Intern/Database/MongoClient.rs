#![allow(non_snake_case)]
#![allow(dead_code)]
use std::error::Error;
use std::time::SystemTime;

use bson::{DateTime, doc, Document};
use mongoose::mongodb::{Client, Collection, options::ClientOptions};

pub(crate) struct Mongo {
    pub(crate) client: Client,
}

impl Mongo {
    pub(crate) async fn new() -> Result<Self, Box<dyn Error>> {
        let mut client_options = ClientOptions::parse("mongodb://localhost:27017").await?;
        client_options.app_name = Some("Rust".to_string());
        let client = Client::with_options(client_options)?;
        Ok(Self { client })
    }


    pub(crate) async fn openCollection(
        &self,
        database: &str,
        collection: &str
    ) -> Result<Collection<Document>, Box<dyn Error>> {
        let db = self.client.database(database);
        let coll = db.collection(collection);
        Ok(coll)
    }

    pub(crate) async fn doesUserExists(
        &self,
        collection: Collection<Document>,
        usermail: &str,
        username: &str,
    ) -> Result<bool, Box<dyn Error>> {
        let result = collection
            .find_one(doc! { "$or": [ { "usermail": usermail }, { "username": username } ] }, None)
            .await?;
        Ok(result.is_some())
    }

    pub(crate) async fn doesMailUserExists(
        &self,
        collection: Collection<Document>,
        usermail: &str,
    ) -> Result<bool, Box<dyn Error>> {
        let result = collection.find_one(doc! { "usermail": usermail }, None).await?;
        Ok(result.is_some())
    }

    pub(crate) async fn getRole(
        &self,
        collection: Collection<Document>,
        usermail: &str,
    ) -> Result<String, Box<dyn Error>> {
        let result = collection.find_one(doc! { "usermail": usermail }, None).await?;
        let doc = result.unwrap();
        let role = doc.get("role").unwrap().as_str().unwrap();
        let role_string = role.to_string();
        Ok(role_string)
    }

    pub(crate) async fn checkPwd(
        &self,
        collection: Collection<Document>,
        usermail: &str,
        password: &str,
    ) -> Result<bool, Box<dyn Error>> {
        let result = collection.find_one(doc! { "usermail": usermail }, None).await?;
        let doc = result.unwrap();
        let pwd = doc.get("password").unwrap().as_str().unwrap();
        Ok(pwd == password)
    }

    pub(crate) async fn isTokenExpired(
        &self,
        collection: Collection<Document>,
        usermail: &str,
    ) -> Result<bool, Box<dyn Error>> {
        let result = collection.find_one(doc! { "usermail": usermail }, None).await?;
        let doc = result.unwrap();
        let tokenExpire = doc.get("tokenExpire").unwrap().as_datetime().unwrap();
        let now = SystemTime::now();
        let now = DateTime::from(now);
        Ok(tokenExpire < &now)
    }

    pub(crate) async fn updateToken(
        &self,
        collection: Collection<Document>,
        usermail: &str,
        userToken: &str,
        tokenExpire: SystemTime,
    ) -> Result<(), Box<dyn Error>> {
        collection
            .update_one(
                doc! { "usermail": usermail },
                doc! { "$set": { "userToken": userToken, "tokenExpire": DateTime::from(tokenExpire) } },
                None,
            )
            .await?;
        Ok(())
    }

    pub(crate) async fn updatePwd(
        &self,
        collection: Collection<Document>,
        usermail: &str,
        password: &str,
    ) -> Result<(), Box<dyn Error>> {
        collection
            .update_one(
                doc! { "usermail": usermail },
                doc! { "$set": { "password": password } },
                None,
            )
            .await?;
        Ok(())
    }

    pub(crate) async fn getTokenExpire(
        &self,
        collection: Collection<Document>,
        usermail: &str,
    ) -> Result<DateTime, Box<dyn Error>> {
        let result = collection.find_one(doc! { "usermail": usermail }, None).await?;
        let tokenExpireUnwraped = result.unwrap();
        let valval =  tokenExpireUnwraped.get("tokenExpire").unwrap();
        let tokenExpire = valval.as_datetime().unwrap();
        Ok(tokenExpire.clone())
    }

    pub(crate) async fn getUserId(
        &self,
        collection: Collection<Document>,
        usermail: &str,
    ) -> Result<String, Box<dyn Error>> {
        let result = collection.find_one(doc! { "usermail": usermail }, None).await?;
        let userIdUn = result.unwrap();
        let datVal = userIdUn.get("userId").unwrap();
        let userId = datVal.as_str().unwrap();
        Ok(userId.to_string())
    }

    pub(crate) async fn getToken(
        &self,
        collection: Collection<Document>,
        usermail: &str,
    ) -> Result<String, Box<dyn Error>> {
        let result = collection.find_one(doc! { "usermail": usermail }, None).await?;

        let document = result.unwrap();
        let user_token_value = document.get("userToken").unwrap();
        let user_token = user_token_value.as_str().unwrap();

        Ok(user_token.to_string())
    }

    pub(crate) async fn checkPasswordFromId(
        &self,
        collection: Collection<Document>,
        password: &str,
        user_id: &str,
    ) -> Result<bool, Box<dyn Error>> {
        // Try to get the user and check if the password of THIS user is like the one provided
        let result = collection.find_one(doc! { "userId": user_id }, None).await?;
        if let Some(document) = result {
            if let Some(pwd) = document.get("password") {
                return Ok(pwd.as_str().unwrap_or("") == password);
            }
        }
        Ok(false)
    }

}

