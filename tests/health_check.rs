use std::net::TcpListener;

fn spawn_app() -> String {
    // Create a new listener on port 0 which automatically chooses a random port
    let listener = TcpListener::bind("127.0.0.1:0").expect("Failed to bind to a random port");
    // Retrieve the port from the listener
    let port = listener.local_addr().unwrap().port();

    let server = zero2prod::startup::run(listener).expect("Failed to spawn the server.");
    let _ = tokio::spawn(server);

    format!("http:127.0.0.1:{}", port)
}

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

#[tokio::test]
async fn subscribe_returns_200_for_valid_form_data() {
    // Start the application
    let address = spawn_app();
    // Create a reqwest client
    let client = reqwest::Client::new();

    // Run the request
    let body = "name=le%20guin&email=ursula_le_guin%40gmail.com";
    let response = client
        .post(&format!("{}/subscribe", &address))
        .header("Content-Type", "application/x-www-form-urlencoded")
        .body(body)
        .send()
        .await
        .expect("Failed to execute the request.");

    // Make assertions
    assert_eq!(response.status().as_u16(), 200);
}

#[tokio::test]
async fn subscribe_returns_400_for_invalid_form_data() {
    // Start the application
    let address = spawn_app();
    // Create a reqwest client
    let client = reqwest::Client::new();
    // Create a mapping of invalid data mapped to error message
    let test_cases = vec![
        ("name=le%20guin", "missing the email"),
        ("email=ursula_le_guin%40gmail.com", "missing the name"),
        ("", "missing both name and email"),
        ("fork=knife", "irrelevant data submitted"),
    ];

    for (invalid_body, error_message) in test_cases {
        // Run the request
        let response = client
            .post(&format!("{}/subscribe", &address))
            .header("Content-Type", "application/x-www-form-urlencoded")
            .body(invalid_body)
            .send()
            .await
            .expect("Failed to execute the request.");

        // Make assertions
        assert_eq!(
            response.status().as_u16(),
            400,
            "The API did not fail with 400 Bad Request when the payload was {}.",
            error_message
        );
    }
}
