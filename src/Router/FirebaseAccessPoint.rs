use reqwest::Error;

pub(crate) async fn deleteImage(imageUrl: &str) -> Result<(), Error> {
    let firebaseUrl = format!("{}", imageUrl);
    let response = reqwest::Client::new()
        .delete(&firebaseUrl)
        .send()
        .await
        .unwrap();
    Ok(())
}