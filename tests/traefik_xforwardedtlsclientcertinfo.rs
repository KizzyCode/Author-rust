//! Tests for the `/traefik/xforwardedtlsclientcertinfo/{regex}` regex

mod utils;

#[test]
pub fn test_401_missingheader() {
    // Get test server instance
    let address = utils::author_test_server();

    // Send request
    #[rustfmt::skip]
    let response = utils::request_response(&address, concat! {
        "GET /traefik/xforwardedtlsclientcertinfo/%5ESubject=%22CN=(Firstname%5C+Lastname)%22.*$ HTTP/1.1\r\n",
        "Connection: close\r\n",
        "Content-Length: 0\r\n",
        "\r\n"
    });

    // Validate response
    #[rustfmt::skip]
    assert_eq!(response, concat! {
        "HTTP/1.1 401 Unauthorized\r\n",
        "Content-Length: 0\r\n",
        "WWW-Authenticate: ClientCertificate\r\n",
        "\r\n"
    });
}

#[test]
pub fn test_400_invalidcertificate_percentcoding() {
    // Get test server instance
    let address = utils::author_test_server();

    // Send request
    #[rustfmt::skip]
    let response = utils::request_response(&address, concat! {
        "GET /traefik/xforwardedtlsclientcertinfo/%5ESubject=%22CN=(Firstname%5C+Lastname)%22.*$ HTTP/1.1\r\n",
        "X-Forwarded-Tls-Client-Cert-Info: Subject%3D%22CN%3DFirstname+Lastname%2G\r\n",
        "Connection: close\r\n",
        "Content-Length: 0\r\n",
        "\r\n"
    });

    // Validate response
    #[rustfmt::skip]
    assert_eq!(response, concat! {
        "HTTP/1.1 400 Bad Request\r\n",
        "Content-Type: text/plain\r\n",
        "Content-Length: 36\r\n",
        "\r\n",
        "Invalid certificate percent encoding"
    });
}

#[test]
pub fn test_400_invalidcertificate_utf8() {
    // Get test server instance
    let address = utils::author_test_server();

    // Send request
    #[rustfmt::skip]
    let response = utils::request_response(&address, concat! {
        "GET /traefik/xforwardedtlsclientcertinfo/%5ESubject=%22CN=(Firstname%5C+Lastname)%22.*$ HTTP/1.1\r\n",
        "X-Forwarded-Tls-Client-Cert-Info: Subject%3D%22CN%3DFirstname+Lastname%80\r\n",
        "Connection: close\r\n",
        "Content-Length: 0\r\n",
        "\r\n"
    });

    // Validate response
    #[rustfmt::skip]
    assert_eq!(response, concat! {
        "HTTP/1.1 400 Bad Request\r\n",
        "Content-Type: text/plain\r\n",
        "Content-Length: 34\r\n",
        "\r\n",
        "Invalid certificate UTF-8 encoding"
    });
}

#[test]
pub fn test_400_invalidregex_percentcoding() {
    // Get test server instance
    let address = utils::author_test_server();

    // Send request
    #[rustfmt::skip]
    let response = utils::request_response(&address, concat! {
        "GET /traefik/xforwardedtlsclientcertinfo/%5ESubject=%22CN=(Firstname%5C+Lastname)%2G.*$ HTTP/1.1\r\n",
        "X-Forwarded-Tls-Client-Cert-Info: Subject%3D%22CN%3DFirstname+Lastname%22\r\n",
        "Connection: close\r\n",
        "Content-Length: 0\r\n",
        "\r\n"
    });

    // Validate response
    #[rustfmt::skip]
    assert_eq!(response, concat! {
        "HTTP/1.1 400 Bad Request\r\n",
        "Content-Type: text/plain\r\n",
        "Content-Length: 30\r\n",
        "\r\n",
        "Invalid regex percent encoding"
    });
}

#[test]
pub fn test_400_invalidregex_utf8() {
    // Get test server instance
    let address = utils::author_test_server();

    // Send request
    #[rustfmt::skip]
    let response = utils::request_response(&address, concat! {
        "GET /traefik/xforwardedtlsclientcertinfo/%5ESubject=%22CN=(Firstname%5C+Lastname)%80.*$ HTTP/1.1\r\n",
        "X-Forwarded-Tls-Client-Cert-Info: Subject%3D%22CN%3DFirstname+Lastname%22\r\n",
        "Connection: close\r\n",
        "Content-Length: 0\r\n",
        "\r\n"
    });

    // Validate response
    #[rustfmt::skip]
    assert_eq!(response, concat! {
        "HTTP/1.1 400 Bad Request\r\n",
        "Content-Type: text/plain\r\n",
        "Content-Length: 28\r\n",
        "\r\n",
        "Invalid regex UTF-8 encoding"
    });
}

#[test]
pub fn test_400_invalidregex_syntax() {
    // Get test server instance
    let address = utils::author_test_server();

    // Send request
    #[rustfmt::skip]
    let response = utils::request_response(&address, concat! {
        "GET /traefik/xforwardedtlsclientcertinfo/%5ESubject=%22CN=(Firstname%5C+Lastname))%22.*$ HTTP/1.1\r\n",
        "X-Forwarded-Tls-Client-Cert-Info: Subject%3D%22CN%3DFirstname+Lastname%22\r\n",
        "Connection: close\r\n",
        "Content-Length: 0\r\n",
        "\r\n"
    });

    // Validate response
    #[rustfmt::skip]
    assert_eq!(response, concat! {
        "HTTP/1.1 400 Bad Request\r\n",
        "Content-Type: text/plain\r\n",
        "Content-Length: 28\r\n",
        "\r\n",
        "Invalid regex syntax/grammar"
    });
}

#[test]
pub fn test_400_invalidregex_nogroups() {
    // Get test server instance
    let address = utils::author_test_server();

    // Send request
    #[rustfmt::skip]
    let response = utils::request_response(&address, concat! {
        "GET /traefik/xforwardedtlsclientcertinfo/%5ESubject=%22CN=Firstname%5C+Lastname%22.*$ HTTP/1.1\r\n",
        "X-Forwarded-Tls-Client-Cert-Info: Subject%3D%22CN%3DFirstname+Lastname%22\r\n",
        "Connection: close\r\n",
        "Content-Length: 0\r\n",
        "\r\n"
    });

    // Validate response
    #[rustfmt::skip]
    assert_eq!(response, concat! {
        "HTTP/1.1 400 Bad Request\r\n",
        "Content-Type: text/plain\r\n",
        "Content-Length: 28\r\n",
        "\r\n",
        "Invalid regex capture groups"
    });
}

#[test]
pub fn test_400_invalidregex_manygroups() {
    // Get test server instance
    let address = utils::author_test_server();

    // Send request
    #[rustfmt::skip]
    let response = utils::request_response(&address, concat! {
        "GET /traefik/xforwardedtlsclientcertinfo/%5ESubject=%22CN=(Firstname)%5C+(Lastname)%22.*$ HTTP/1.1\r\n",
        "X-Forwarded-Tls-Client-Cert-Info: Subject%3D%22CN%3DFirstname+Lastname%22\r\n",
        "Connection: close\r\n",
        "Content-Length: 0\r\n",
        "\r\n"
    });

    // Validate response
    #[rustfmt::skip]
    assert_eq!(response, concat! {
        "HTTP/1.1 400 Bad Request\r\n",
        "Content-Type: text/plain\r\n",
        "Content-Length: 28\r\n",
        "\r\n",
        "Invalid regex capture groups"
    });
}

#[test]
pub fn test_403_nomatch() {
    // Get test server instance
    let address = utils::author_test_server();

    // Send request
    #[rustfmt::skip]
    let response = utils::request_response(&address, concat! {
        "GET /traefik/xforwardedtlsclientcertinfo/%5ESubject=%22CN=(Firstname%5C+Lastname)%22.*$ HTTP/1.1\r\n",
        "X-Forwarded-Tls-Client-Cert-Info: Subject%3D%22CN%3DFirstname+Lostname%22\r\n",
        "Connection: close\r\n",
        "Content-Length: 0\r\n",
        "\r\n"
    });

    // Validate response
    #[rustfmt::skip]
    assert_eq!(response, concat! {
        "HTTP/1.1 403 Forbidden\r\n",
        "Content-Length: 0\r\n",
        "\r\n"
    });
}

#[test]
pub fn test_403_emptycapture() {
    // Get test server instance
    let address = utils::author_test_server();

    // Send request
    #[rustfmt::skip]
    let response = utils::request_response(&address, concat! {
        "GET /traefik/xforwardedtlsclientcertinfo/%5ESubject=%22CN=()%22.*$ HTTP/1.1\r\n",
        "X-Forwarded-Tls-Client-Cert-Info: Subject%3D%22CN%3D%22\r\n",
        "Connection: close\r\n",
        "Content-Length: 0\r\n",
        "\r\n"
    });

    // Validate response
    #[rustfmt::skip]
    assert_eq!(response, concat! {
        "HTTP/1.1 403 Forbidden\r\n",
        "Content-Length: 0\r\n",
        "\r\n"
    });
}

#[test]
pub fn test_200() {
    // Get test server instance
    let address = utils::author_test_server();

    // Send request
    #[rustfmt::skip]
    let response = utils::request_response(&address, concat! {
        "GET /traefik/xforwardedtlsclientcertinfo/%5ESubject=%22CN=(Firstname%5C+Lastname)%22.*$ HTTP/1.1\r\n",
        "X-Forwarded-Tls-Client-Cert-Info: Subject%3D%22CN%3DFirstname+Lastname%22\r\n",
        "Connection: close\r\n",
        "Content-Length: 0\r\n",
        "\r\n"
    });

    // Validate response
    #[rustfmt::skip]
    assert_eq!(response, concat! {
        "HTTP/1.1 200 OK\r\n",
        "Content-Length: 0\r\n",
        "X-Author-AuthId: Firstname+Lastname\r\n",
        "\r\n"
    });
}
