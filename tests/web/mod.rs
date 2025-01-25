mod routes;

use koko::web;
use rocket::http::{Header, Status};
use rocket::local::asynchronous::Client;

pub struct TestResponse {
    pub status: Status,
    pub body: String,
    pub headers: Vec<Header<'static>>,
}

pub async fn test_route(
    path: &'static str,
    expected_status: Status,
) -> TestResponse {
    let rocket = web::rocket();
    let client = Client::tracked(rocket)
        .await
        .expect("Failed to launch web server");

    let response = client.get(path).dispatch().await;

    assert_eq!(response.status(), expected_status);

    let status = response.status();

    // Extract headers before moving `response`
    let headers: Vec<Header<'static>> = response
        .headers()
        .iter()
        .map(|h| Header::new(h.name().to_string(), h.value().to_string())) // Convert to owned Header
        .collect();

    let body = response.into_string().await.unwrap_or_default(); // Move response

    TestResponse {
        status,
        body,
        headers,
    }
}

#[rocket::async_test]
async fn test_swagger_ui_route() {
    test_route("/swagger-ui/", Status::SeeOther).await;
}

#[rocket::async_test]
async fn test_rapidoc_route() {
    test_route("/rapidoc/", Status::SeeOther).await;
}

#[rocket::async_test]
async fn test_non_existent_route() {
    test_route("/non-existent", Status::NotFound).await;
}
