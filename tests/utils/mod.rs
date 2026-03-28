//! Testing utils

use author::Config;
use std::io::{Read, Write};
use std::net::{Shutdown, SocketAddr, TcpStream};
use std::sync::LazyLock;
use std::time::Duration;
use std::{env, thread};

/// Starts an author test server if necessary and returns the address
pub fn author_test_server() -> SocketAddr {
    /// The server address
    static ADDRESS: LazyLock<SocketAddr> = LazyLock::new(|| {
        // Allow the user to overwrite the test server address
        let listen_var = env::var("AUTHOR_LISTEN");
        let listen_str = listen_var.as_deref().unwrap_or("127.0.0.1:60174");
        let listen = listen_str.parse().expect("failed to parse AUTHOR_LISTEN");

        // Create config and start server in an async context
        let config =
            Config { AUTHOR_LISTEN: listen, AUTHOR_CONNMAX: 1024, AUTHOR_HEADER: "X-Forward-AuthorAuthId".into() };
        thread::spawn(|| {
            // Start the server **and** panic if the server crashes
            eprintln!("Starting author test server on {}...", config.AUTHOR_LISTEN);
            author::serve(config).expect("author server panicked");
        });

        // Give the server some time to start
        thread::sleep(Duration::from_secs(3));
        listen
    });

    // Start server and get address
    *LazyLock::force(&ADDRESS)
}

/// Performs a request to the server and reads the response
pub fn request_response<Request>(address: &SocketAddr, request: Request) -> String
where
    Request: AsRef<str>,
{
    // We do not read HTTP content length, so we require the server to close the connection
    let request = request.as_ref();
    assert!(request.contains("Connection: close"), "request is keep alive");

    // Open connection and send all data
    let mut connection = TcpStream::connect(address).expect("failed to connect to server");
    connection.write_all(request.as_bytes()).expect("failed to sent request to server");
    connection.shutdown(Shutdown::Write).expect("failed to close tx connection");
    connection.flush().expect("failed to flush tx connection");

    // Receive the entire response
    let mut response = Vec::new();
    connection.read_to_end(&mut response).expect("failed to receive response");
    String::from_utf8(response).expect("response is not valid UTF-8")
}
