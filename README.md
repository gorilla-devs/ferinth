# Modrinth API

Modrinth API is a simple library for using, you guessed it, the Modrinth API in Rust projects. It uses `reqwest` as its HTTP(S) client and deserialises responses to typed structs using `serde`.

As of now, the following features have been implemented
- All structure definitions based on https://github.com/modrinth/labrinth/wiki/API-Documentation#structure-definitions
- The following calls:
  - Get mod by mod ID
  - List mod categories
  - List mod loaders
  - List game versions
  - Get user by user ID
  - List versions by mod ID
  - Get version by version ID
  - Download version file

This means that the following features have not yet been implemented
- Search mods (expected in `1.1.0`)
- User authentication
- Get current user (constrained by lack of user authentication)

## Use

To use the Modrinth API, you first need to decide which version of the API you want to use. The current version, v1, is only for the beta. A new better documented version, v2, is expected in the coming months and will be supported in version `modrinth-api v2`. **The major version of this crate's version directly corresponds to the API version it supports**.
After you have decided the API version to use, you can add this library under the `[dependencies]` section in your `Cargo.toml`.
