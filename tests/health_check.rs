use std::net::TcpListener;

#[tokio::test]
async fn health_check_works() {
    // Start the application
    let address = spawn_app();
    // Create a reqwest client
    let client = reqwest::Client::new();

    // Run the request
    let response = client
        .get(&format!("{}/healthcheck", &address))
        .send()
        .await
        .expect("Failed to execute the request.");

    // Make assertions
    assert!(response.status().is_success());
    assert_eq!(Some(0), response.content_length());
}

fn spawn_app() -> String {
    // Create a new listener on port 0 which automatically chooses a random port
    let listener = TcpListener::bind("127.0.0.1:0").expect("Failed to bind to a random port");
    // Retrieve the port from the listener
    let port = listener.local_addr().unwrap().port();

    let server = zero2prod::run(listener).expect("Failed to spawn the server.");
    let _ = tokio::spawn(server);

    format!("http:127.0.0.1:{}", port)
}
