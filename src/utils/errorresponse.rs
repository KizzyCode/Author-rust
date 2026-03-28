//! Provides convenience constructors for informative error responses

use ehttpd::bytes::Data;
use ehttpd::http::Response;

/// Convenience constructors for informative error responses
pub trait ErrorResponse
where
    Self: Sized,
{
    /// Creates a new `400 Bad Request` HTTP response with the given message as `text/plain` body
    fn new_400_badrequest<Message>(message: Message) -> Self
    where
        Message: Into<Data>;
    /// Creates a new `404 Not Found` HTTP response with the given message as `text/plain` body
    fn new_404_notfound<Message>(message: Message) -> Self
    where
        Message: Into<Data>;
}
impl ErrorResponse for Response {
    fn new_400_badrequest<Message>(message: Message) -> Self
    where
        Message: Into<Data>,
    {
        let mut response = Response::new_400_badrequest();
        response.set_content_type(b"text/plain");
        response.set_body_data(message);
        response
    }

    fn new_404_notfound<Message>(message: Message) -> Self
    where
        Message: Into<Data>,
    {
        let mut response = Response::new_404_notfound();
        response.set_content_type(b"text/plain");
        response.set_body_data(message);
        response
    }
}
