# Ferinth

Ferinth is a simple library for using the Modrinth API in Rust projects. It uses `reqwest` as its HTTP(S) client and deserialises responses to typed structs using `serde`.

As of now, the following features have been implemented
- All structure definitions based on https://github.com/modrinth/labrinth/wiki/API-Documentation#structure-definitions
- Structure definition for team member
- The following calls:
  - Get mod by mod ID
  - List mod categories
  - List mod loaders
  - List game versions
  - Get user by user ID
  - List team members by team ID
  - List versions by mod ID
  - Get version by version ID
  - Download version file

This means that the following features have not yet been implemented
- Search mods
- User authentication
- Get current user (constrained by lack of user authentication)

Unfortunately, I am not planning to bring any of these features in version 1 due to poor documentation. I will add these features in version 2.

## Use

To use Ferinth, you first need to decide which version of the Modrinth API you want to use. The current version, v1, is only for the Modrinth Beta. A new better version, v2, is expected in the coming months and will be supported in Ferinth version 2. **The major version of this crate's version directly corresponds to the API version it supports**.
After you have decided the API version to use, you can add this library under the `[dependencies]` section in your `Cargo.toml`.
