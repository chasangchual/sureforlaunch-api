//! tests/health_check.rs

use sureforlaunch_api::run;

#[tokio::test]
async fn health_check_works() {
    spawn_app();

    let client = reqwest::Client::new();
    let response = client
        .get("http://127.0.0.1:9090/health_check")
        .send()
        .await
        .expect("Failed to execute request");

    assert!(response.status().is_success());

    // expect the health check endpoint returns 'hello'
    assert_eq!(Some(5), response.content_length());
}

fn spawn_app() {
    let server = run().expect("Failed to bind address");

    // Launch the server as a background task
    // tokio::spawn returns a handle to the spawned future,
    // but we have no use for it here, hence the non-binding let
    let _ = tokio::spawn(server);
}