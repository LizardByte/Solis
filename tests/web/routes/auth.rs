use crate::web::test_route;

use rocket::http::Status;

#[rocket::async_test]
async fn test_login_route() {
    test_route("/login", Status::Ok).await;
}

#[rocket::async_test]
async fn test_logout_route() {
    test_route("/logout", Status::Ok).await;
}
