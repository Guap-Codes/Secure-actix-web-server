use actix_web::{test, web, App};
use reqwest::Client;
use std::env;
use std::process::Command;
use std::time::Duration;

// Import the necessary modules from your main application
use main::{hello, load_tls_config, not_found};

#[actix_rt::test]
async fn test_server_integration() {
    // Set up environment variables for testing
    env::set_var("CERT_FILE", "path/to/test/cert.pem");
    env::set_var("KEY_FILE", "path/to/test/key.pem");
    env::set_var("SERVER_ADDRESS", "127.0.0.1:3001");
    env::set_var("NUM_WORKERS", "2");

    // Start the server in a separate process
    let mut server = Command::new("cargo")
        .args(&["run", "--bin", "your_binary_name"])
        .spawn()
        .expect("Failed to start server");

    // Give the server some time to start up
    std::thread::sleep(Duration::from_secs(2));

    // Create a test app
    let _ = load_tls_config().expect("Failed to load TLS config");
    let app = test::init_service(
        App::new()
            .route("/hello", web::get().to(hello))
            .default_service(web::route().to(not_found)),
    )
    .await;

    // Use the app for testing
    let req = test::TestRequest::get().uri("/hello").to_request();
    let resp = test::call_service(&app, req).await;
    assert!(resp.status().is_success());

    // Create an HTTPS client
    let client = Client::builder()
        .danger_accept_invalid_certs(true) // For testing purposes only
        .build()
        .expect("Failed to create HTTPS client");

    // Test the /hello route
    let resp = client
        .get("https://127.0.0.1:3001/hello")
        .send()
        .await
        .expect("Failed to execute request");

    assert!(resp.status().is_success());
    assert_eq!(resp.text().await.unwrap(), "Hello world!");

    // Test a non-existent route (should return 404)
    let resp = client
        .get("https://127.0.0.1:3001/non_existent")
        .send()
        .await
        .expect("Failed to execute request");

    assert_eq!(resp.status(), 404);
    assert_eq!(resp.text().await.unwrap(), "Not Found");

    // Test server configuration
    let server_addr = env::var("SERVER_ADDRESS").unwrap();
    assert_eq!(server_addr, "127.0.0.1:3001");

    let num_workers = env::var("NUM_WORKERS").unwrap().parse::<usize>().unwrap();
    assert_eq!(num_workers, 2);

    // Clean up: stop the server
    server.kill().expect("Failed to stop server");
}

#[actix_rt::test]
async fn test_tls_config() {
    // Test TLS configuration loading
    env::set_var("CERT_FILE", "path/to/test/cert.pem");
    env::set_var("KEY_FILE", "path/to/test/key.pem");

    let tls_config = load_tls_config();
    assert!(tls_config.is_ok(), "Failed to load TLS configuration");

    // Test with non-existent files
    env::set_var("CERT_FILE", "non_existent_cert.pem");
    env::set_var("KEY_FILE", "non_existent_key.pem");

    let tls_config = load_tls_config();
    assert!(
        tls_config.is_err(),
        "TLS config should fail with non-existent files"
    );
}

#[actix_rt::test]
async fn test_server_error_handling() {
    // Test server startup with invalid address
    env::set_var("SERVER_ADDRESS", "invalid_address");

    let result = Command::new("cargo")
        .args(&["run", "--bin", "your_binary_name"])
        .output();

    assert!(result.is_err() || !result.unwrap().status.success());

    // Reset SERVER_ADDRESS for other tests
    env::set_var("SERVER_ADDRESS", "127.0.0.1:3001");
}
