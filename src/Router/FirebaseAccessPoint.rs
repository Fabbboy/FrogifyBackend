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

pub(crate) async fn generateProfilePicture(username: &str) -> Result<String, Error> {
    let randomRotation = rand::random::<u8>() % 4;
    let baseUrl = "https://api.dicebear.com/6.x/shapes/svg";
    let url = format!("?seed={}&rotate={}&radius=50", username, randomRotation);
    Ok(format!("{}{}", baseUrl, url))
}