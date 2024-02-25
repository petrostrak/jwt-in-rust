## JWT implementation in Rust
How you can implement authentication using JSON Web Tokens (JWTs) in Rust.

## What is a JWT?
A JSON Web Token (JWT) is a compact, URL-safe way to transfer data (”claims”) between two parties over the Web. The data is typically encoded using a JSON Web Signature or as part of a JSON Web Encryption (JWE) structure, and can be encrypted (and signed!).

JWTs are a popular option when deciding on an auth strategy. The client stores all the information via the JWT, allowing for a stateless API. This can make user authentication much easier in some cases. While unencrypted and unsigned JWTs can be manipulated, it is simple for the server to be able to disregard manipulated JWTs as long as the secret key used to create them remains secret.
