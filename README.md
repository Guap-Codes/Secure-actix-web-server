# Secure Actix Web Server

This project implements a secure web server using Actix-web with TLS support. It provides a simple "Hello World" route and custom 404 handling, with support for environment variable configuration and multi-threading.

## Features

- HTTPS support using TLS
- Simple "Hello World" route
- Custom 404 handling
- Environment variable configuration
- Multi-threading support

## Prerequisites

- Rust (latest stable version)
- OpenSSL development libraries

## Setup

1. Clone the repository:
   ```
   git clone https://github.com/Guap-Codes/secure-actix-web-server.git
   cd secure-actix-web-server
   ```

2. Create a `.env` file in the project root and add the following variables (adjust as needed):
   ```
   CERT_FILE=path/to/your/cert.pem
   KEY_FILE=path/to/your/key.pem
   SERVER_ADDRESS=127.0.0.1:3000
   NUM_WORKERS=4
   RUST_LOG=info
   ```

3. Generate a self-signed certificate for development (for production, use a properly signed certificate):
   ```
   openssl req -x509 -newkey rsa:4096 -nodes -keyout key.pem -out cert.pem -days 365 -subj '/CN=localhost'
   ```

## Running the Server

1. Build and run the server:
   ```
   cargo run
   ```

2. The server will start and display the address it's running on (e.g., `https://127.0.0.1:3000`).

## Usage

- Access the hello route: `https://127.0.0.1:3000/hello`
- Any other route will return a 404 Not Found response

## Configuration

The following environment variables can be used to configure the server:

- `CERT_FILE`: Path to the TLS certificate file (default: "cert.pem")
- `KEY_FILE`: Path to the TLS private key file (default: "key.pem")
- `SERVER_ADDRESS`: Address and port for the server to listen on (default: "127.0.0.1:3000")
- `NUM_WORKERS`: Number of worker threads (default: number of CPU cores)
- `RUST_LOG`: Log level (e.g., "info", "debug", "warn")

## Development

To run the server in development mode with auto-reloading:

1. Install `cargo-watch`:
   ```
   cargo install cargo-watch
   ```

2. Run the server with auto-reloading:
   ```
   cargo watch -x run
   ```


## Tests

This project includes integration tests to ensure the functionality of the HTTPS server.

### Running Tests

To run all tests, use the following command:
```cargo test```

This will run the integration test.

### Integration Tests

Integration tests are located in the `tests` directory. They test the server as a whole, including its TLS functionality and route handling.

To run only the integration test, use:
```cargo test --test integration_test```

#### Integration Test Details

The integration tests cover:

1. The `/hello` route
2. The 404 handler for non-existent routes
3. TLS functionality with a self-signed certificate

Note: The integration tests generate temporary self-signed certificates for testing HTTPS. Ensure you have `openssl` installed on your system to run these tests.

### Test Dependencies

The tests require additional dependencies, which are specified in the `dev-dependencies` section of `Cargo.toml`:
```
[dev-dependencies]
actix-rt="2.7"
reqwest={version="0.11", features=["rustls-tls"]}
```

Make sure these dependencies are present in your `Cargo.toml` file to run the tests successfully.



## License

This project is licensed under the MIT License - see the [LICENSE](LICENSE) file for details.