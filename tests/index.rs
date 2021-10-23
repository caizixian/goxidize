mod common;
use common::*;
use goxide::models::{Link, LinkFormData};

#[actix_rt::test]
async fn insert_one() {
    let app = spawn_app().await;
    let client = reqwest::Client::new();

    let f = LinkFormData::new(
        "reserve".to_owned(),
        "https://squirrel.anu.edu.au/reserve".to_owned(),
    );

    let response = client
        // Use the returned application address
        .post(&format!("{}/", &app.address))
        .form(&f)
        .send()
        .await
        .expect("Failed to execute request.");

    assert!(response.status().is_success());

    let saved = Link::fetch_by_path("reserve", &app.db_pool)
        .await
        .expect("Failed to fetch a saved row");

    assert_eq!(saved.destination, "https://squirrel.anu.edu.au/reserve");
}

#[actix_rt::test]
async fn upsert() {
    let app = spawn_app().await;
    let client = reqwest::Client::new();

    let f1 = LinkFormData::new(
        "reserve".to_owned(),
        "https://squirrel.anu.edu.au/reserve".to_owned(),
    );

    let response = client
        // Use the returned application address
        .post(&format!("{}/", &app.address))
        .form(&f1)
        .send()
        .await
        .expect("Failed to execute request.");

    assert!(response.status().is_success());

    let f2 = LinkFormData::new(
        "reserve".to_owned(),
        "https://squirrel.anu.edu.au/reserve-check".to_owned(),
    );

    let response = client
        // Use the returned application address
        .post(&format!("{}/", &app.address))
        .form(&f2)
        .send()
        .await
        .expect("Failed to execute request.");

    assert!(response.status().is_success());

    let saved = Link::fetch_by_path("reserve", &app.db_pool)
        .await
        .expect("Failed to fetch a saved row");

    assert_eq!(
        saved.destination,
        "https://squirrel.anu.edu.au/reserve-check"
    );
}
