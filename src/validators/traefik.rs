//! Traefik auth validators

use crate::config::Config;
use crate::utils::errorresponse::ErrorResponse;
use crate::utils::percentcoding;
use ehttpd::http::{Request, Response};
use regex::Regex;

/// Validates the `X-Forwarded-Tls-Client-Cert-Info` against the given regex, and sets the captured regex match as
/// authentication response header
pub fn xforwardedtlsclientcertinfo(config: &Config, request: Request, regex: &[u8]) -> Response {
    // Get the certificate info from the `X-Forwarded-Tls-Client-Cert-Info` header
    let Some(cert_info) = request.field("X-Forwarded-Tls-Client-Cert-Info") else {
        return Response::new_401_unauthorized("ClientCertificate");
    };
    let Ok(cert_info) = percentcoding::decode(cert_info.as_ref()) else {
        return ErrorResponse::new_400_badrequest("Invalid certificate percent encoding");
    };
    let Ok(cert_info) = str::from_utf8(&cert_info) else {
        return ErrorResponse::new_400_badrequest("Invalid certificate UTF-8 encoding");
    };

    // Get the regex from the URL fragment
    let Ok(regex) = percentcoding::decode(regex) else {
        return ErrorResponse::new_400_badrequest("Invalid regex percent encoding");
    };
    let Ok(regex) = str::from_utf8(&regex) else {
        return ErrorResponse::new_400_badrequest("Invalid regex UTF-8 encoding");
    };
    let Ok(regex) = Regex::new(regex) else {
        return ErrorResponse::new_400_badrequest("Invalid regex syntax/grammar");
    };
    let 2 = regex.captures_len() else {
        return ErrorResponse::new_400_badrequest("Invalid regex capture groups");
    };

    // Validate the certificate info against the required regex
    let Some(matches) = regex.captures(cert_info) else {
        return Response::new_403_forbidden();
    };
    let false = matches[1].is_empty() else {
        return Response::new_403_forbidden();
    };

    // We need to pass take ownership here to move data into the response
    let header = config.AUTHOR_HEADER.to_string();
    let match_ = matches[1].to_string();

    // Build response
    let mut response = Response::new_200_ok();
    response.set_field(header, match_);
    response
}
