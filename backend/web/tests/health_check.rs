use web::run;

#[tokio::test]
async fn health_check_test() {
    spawn_app();

    let client = reqwest::Client::new();

    let response = client
        .get("http://127.0.0.1:8000/health_check")
        .send()
        .await
        .expect("Failed to execute requsest");

    assert!(response.status().is_success());
    assert_eq!(Some(19), response.content_length());
}

fn spawn_app() {
    let server = web::run("127.0.0.1:8000").expect("Failed to bind address");
    let _ = tokio::spawn(server);
}
