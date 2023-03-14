use std::error::Error;

use bson::{DateTime, doc};
use mongoose::mongodb::{Client, Collection, options::ClientOptions};
use serde::{de::DeserializeOwned, Serialize};

pub(crate) struct Mongo {
    pub(crate) client: Client,
}

impl Mongo {
    pub(crate) async fn new() -> Result<Self, Box<dyn std::error::Error>> {
        let mut client_options = ClientOptions::parse("mongodb://localhost:27017").await?;
        client_options.app_name = Some("Rust".to_string());
        let client = Client::with_options(client_options)?;
        Ok(Self { client })
    }

    pub(crate) async fn open_collection<T: Serialize>(&self, database: &str, collection: &str) -> Result<mongoose::mongodb::Collection<T>, Box<dyn std::error::Error>> {
        let db = self.client.database(database);
        let coll = db.collection(collection);
        Ok(coll)
    }

    pub(crate) async fn does_user_exists<T: Serialize + DeserializeOwned + Unpin + Send + Sync>(
        &self,
        collection: mongoose::mongodb::Collection<T>,
        usermail: &str,
        username: &str,
    ) -> Result<bool, Box<dyn std::error::Error>> {
        let result = collection
            .find_one(doc! { "$or": [ { "usermail": usermail }, { "username": username } ] }, None)
            .await?;
        Ok(result.is_some())
    }

}

