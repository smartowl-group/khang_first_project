use std::net::TcpListener;

use reqwest;

fn spawn_app() -> String {
    // let server = khang_first_project::run("127.0.0.1:0").expect("Failed to bind address");
    // let _ = tokio::spawn(server);

    let listener = TcpListener::bind("127.0.0.1:0").expect("Failed to bind to a random port");
    let port = listener.local_addr().unwrap().port();
    let server = khang_first_project::run(listener).expect("Failed to bind an address");
    let _ = tokio::spawn(server);
    format!("http://127.0.0.1:{}", port)
}
#[actix_rt::test]
async fn health_check_works() {
    let address = spawn_app();

    let client = reqwest::Client::new();

    let response = client
        .get(format!("{}/health_check", address))
        .send()
        .await
        .expect("Failed to execute request.");
    assert!(response.status().is_success());
    assert_eq!(Some(20), response.content_length());
}

// test POST 200 OK
async fn subscribe_returns_200() {}

// test POST 404 Bad Request
async fn subscribe_returns_404() {}
