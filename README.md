[![License BSD-2-Clause](https://img.shields.io/badge/License-BSD--2--Clause-blue.svg)](https://opensource.org/licenses/BSD-2-Clause)
[![License MIT](https://img.shields.io/badge/License-MIT-blue.svg)](https://opensource.org/licenses/MIT)
[![AppVeyor CI](https://ci.appveyor.com/api/projects/status/github/KizzyCode/Author-rust?svg=true)](https://ci.appveyor.com/project/KizzyCode/Author-rust)
[![docs.rs](https://docs.rs/author/badge.svg)](https://docs.rs/author)
[![crates.io](https://img.shields.io/crates/v/author.svg)](https://crates.io/crates/author)
[![Download numbers](https://img.shields.io/crates/d/author.svg)](https://crates.io/crates/author)
[![dependency status](https://deps.rs/crate/author/latest/status.svg)](https://deps.rs/crate/author)


# `author`
Welcome to `author` 🎉

`author` is an ultra minimalistic forward-auth server that can be used to verify and extract login conditions.


## Supported Pattern URLs
- `GET /traefik/xforwardedtlsclientcertinfo/{regex}`: This pattern validates the `X-Forwarded-Tls-Client-Cert-Info`
  header against an URL-encoded regex. This header can be set by traefik in the context of mTLS authentication and is
  useful to validate and extract authentication details about the client certificate.

  For example, the URL `/traefik/xforwardedtlsclientcertinfo/%5ESubject=%22CN=(Firstname%5C+Lastname)%22.*$` asserts
  that the `CN` field of the leaf certificate **matches and captures** `Firstname+Lastname`. On success, the captured
  value will be set as `X-Author-AuthId` header for the response; e.g. `X-Author-AuthId: Firstname+Lastname`.


## Example
By default, author is shipped as a distro-less docker container and can be easily run via e.g. `docker compose`:
```yaml
services:
  author:
    image: ghcr.io/kizzycode/author:latest
    init: true
    ports:
      - 127.0.0.1:8080:8080/tcp
    cap_drop:
      - ALL
    security_opt:
      - no-new-privileges:true
```
