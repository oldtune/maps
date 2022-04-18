#[get("/<lat>/<lng>")]
pub async fn get_location_by_latlng(lat: &str, lng: &str) -> Option<String> {
    let api_key = "AIzaSyAPrzJhpA0V2ZtPpwUuwOZCaVDQn2Y1rLs";
    let endpoint = format!(
        "https://maps.googleapis.com/maps/api/geocode/json?latlng={},{}&key={}",
        lat, lng, api_key
    );

    let client = reqwest::Client::new();
    let response = client.get(endpoint).send().await;
    match response {
        Err(_) => None,
        Ok(resp) => Some(resp.text().await.unwrap()),
    }
}
