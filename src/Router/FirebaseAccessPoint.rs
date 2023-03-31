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

pub(crate) async fn doesImageExists(imageUrl: &str) -> Result<bool, Error> {
    let firebaseUrl = format!("{}", imageUrl);
    let response = reqwest::Client::new()
        .get(&firebaseUrl)
        .send()
        .await
        .unwrap();
    Ok(response.status().is_success())
}