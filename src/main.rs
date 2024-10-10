//! A secure web server using Actix-web with TLS support.
//!
//! This module sets up an HTTPS server with a simple "Hello World" route and
//! custom 404 handling. It uses environment variables for configuration and
//! supports multi-threading.

use actix_web::{web, App, HttpResponse, HttpServer, Responder};
use dotenv::dotenv;
use log::{error, info};
use num_cpus;
use rustls::{Certificate, PrivateKey, ServerConfig};
use rustls_pemfile::{certs, pkcs8_private_keys};
use std::env;
use std::fs::File;
use std::io::{BufReader, Error as IoError};

/// Loads TLS configuration from certificate and key files.
///
/// This function reads the TLS certificate and private key from files specified
/// by environment variables or default paths. It then constructs and returns
/// a ServerConfig for use with rustls.
///
/// # Returns
///
/// * `Result<ServerConfig, IoError>` - The TLS configuration on success, or an IoError if loading fails.
///
/// # Errors
///
/// This function will return an error if:
/// * The certificate or key files cannot be read
/// * The certificate or key data is invalid
/// * The ServerConfig cannot be constructed with the provided certificate and key
pub fn load_tls_config() -> Result<ServerConfig, IoError> {
    let cert_path = env::var("CERT_FILE").unwrap_or_else(|_| "cert.pem".to_string());
    let key_path = env::var("KEY_FILE").unwrap_or_else(|_| "key.pem".to_string());

    info!("Loading TLS certificate from: {}", cert_path);
    info!("Loading TLS private key from: {}", key_path);

    let cert_file = match File::open(&cert_path) {
        Ok(file) => file,
        Err(e) => {
            error!("Failed to open certificate file '{}': {}", cert_path, e);
            return Err(e);
        }
    };
    let key_file = match File::open(&key_path) {
        Ok(file) => file,
        Err(e) => {
            error!("Failed to open private key file '{}': {}", key_path, e);
            return Err(e);
        }
    };

    let mut cert_reader = BufReader::new(cert_file);
    let mut key_reader = BufReader::new(key_file);

    let cert_chain = match certs(&mut cert_reader) {
        Ok(certs) => certs.into_iter().map(Certificate).collect(),
        Err(e) => {
            error!("Failed to parse certificate: {}", e);
            return Err(IoError::new(
                std::io::ErrorKind::InvalidData,
                "Invalid certificate",
            ));
        }
    };

    let mut keys: Vec<PrivateKey> = match pkcs8_private_keys(&mut key_reader) {
        Ok(keys) => keys.into_iter().map(PrivateKey).collect(),
        Err(e) => {
            error!("Failed to parse private key: {}", e);
            return Err(IoError::new(
                std::io::ErrorKind::InvalidData,
                "Invalid private key",
            ));
        }
    };

    if keys.is_empty() {
        error!("No private keys found in the key file");
        return Err(IoError::new(
            std::io::ErrorKind::InvalidData,
            "No private keys found",
        ));
    }

    let config = ServerConfig::builder()
        .with_safe_defaults()
        .with_no_client_auth()
        .with_single_cert(cert_chain, keys.remove(0))
        .map_err(|e| {
            error!("Failed to create ServerConfig: {}", e);
            IoError::new(std::io::ErrorKind::InvalidData, e)
        })?;

    info!("TLS configuration loaded successfully");
    Ok(config)
}

/// Handler for the `/hello` route.
///
/// Returns a simple "Hello world!" message.
///
/// # Returns
///
/// * `impl Responder` - An HTTP response with a 200 OK status and "Hello world!" body.
pub async fn hello() -> impl Responder {
    HttpResponse::Ok().body("Hello world!")
}

/// Handler for routes that don't match any defined routes.
///
/// Returns a 404 Not Found response.
///
/// # Returns
///
/// * `impl Responder` - An HTTP response with a 404 Not Found status and "Not Found" body.
pub async fn not_found() -> impl Responder {
    HttpResponse::NotFound().body("Not Found")
}

/// The main function that sets up and runs the web server.
///
/// This function performs the following steps:
/// 1. Loads environment variables
/// 2. Initializes the logger
/// 3. Loads TLS configuration
/// 4. Configures server address and number of workers
/// 5. Sets up and runs the HTTP server with TLS support
///
/// # Returns
///
/// * `std::io::Result<()>` - Ok(()) if the server runs successfully, or an error if it fails to start.
#[actix_web::main]
async fn main() -> std::io::Result<()> {
    // Load environment variables from .env file if present
    dotenv().ok();
    // Initialize the logger
    env_logger::init();

    info!("Starting server initialization");

    // Load TLS configuration
    let tls_config = match load_tls_config() {
        Ok(config) => config,
        Err(e) => {
            error!("Failed to load TLS configuration: {}", e);
            return Err(e);
        }
    };

    // Get server address from environment variable or use default
    let address = env::var("SERVER_ADDRESS").unwrap_or_else(|_| "127.0.0.1:3000".to_string());
    // Get number of workers from environment variable or use number of CPU cores
    let num_workers = env::var("NUM_WORKERS")
        .ok()
        .and_then(|s| s.parse().ok())
        .unwrap_or_else(num_cpus::get);

    info!("Server running on {} with {} workers", address, num_workers);

    HttpServer::new(move || {
        App::new()
            .route("/hello", web::get().to(hello))
            .default_service(web::route().to(not_found))
    })
    .workers(num_workers)
    .bind_rustls(address, tls_config)?
    .run()
    .await
}

// Add these lines to make the functions public and accessible for testing
//pub use crate::hello;
//pub use crate::load_tls_config;
//pub use crate::not_found;
