use reqwest::Error;

pub(crate) async fn deleteImage(imageUrl: &str) -> Result<(), Error> {
    if imageUrl.is_empty() {
        return Ok(());
    }

    let firebaseUrl = format!("{}", imageUrl);
    let _ = reqwest::Client::new()
        .delete(&firebaseUrl)
        .send()
        .await
        .unwrap();
    Ok(())
}
