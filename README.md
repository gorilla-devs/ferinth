# Ferinth

[![github badge](https://img.shields.io/badge/GitHub-Ferinth-informational?style=for-the-badge&logo=github&labelColor=555555)](https://github.com/theRookieCoder/ferinth/)
[![crates badge](https://img.shields.io/crates/v/ferinth?logo=rust&style=for-the-badge)](https://crates.io/crates/ferinth/)
[![docs.rs](https://img.shields.io/docsrs/ferinth?logo=data%3Aimage%2Fsvg%2Bxml%3Bbase64%2CPHN2ZyByb2xlPSJpbWciIHhtbG5zPSJodHRwOi8vd3d3LnczLm9yZy8yMDAwL3N2ZyIgdmlld0JveD0iMCAwIDUxMiA1MTIiPjxwYXRoIGZpbGw9IiNmNWY1ZjUiIGQ9Ik00ODguNiAyNTAuMkwzOTIgMjE0VjEwNS41YzAtMTUtOS4zLTI4LjQtMjMuNC0zMy43bC0xMDAtMzcuNWMtOC4xLTMuMS0xNy4xLTMuMS0yNS4zIDBsLTEwMCAzNy41Yy0xNC4xIDUuMy0yMy40IDE4LjctMjMuNCAzMy43VjIxNGwtOTYuNiAzNi4yQzkuMyAyNTUuNSAwIDI2OC45IDAgMjgzLjlWMzk0YzAgMTMuNiA3LjcgMjYuMSAxOS45IDMyLjJsMTAwIDUwYzEwLjEgNS4xIDIyLjEgNS4xIDMyLjIgMGwxMDMuOS01MiAxMDMuOSA1MmMxMC4xIDUuMSAyMi4xIDUuMSAzMi4yIDBsMTAwLTUwYzEyLjItNi4xIDE5LjktMTguNiAxOS45LTMyLjJWMjgzLjljMC0xNS05LjMtMjguNC0yMy40LTMzLjd6TTM1OCAyMTQuOGwtODUgMzEuOXYtNjguMmw4NS0zN3Y3My4zek0xNTQgMTA0LjFsMTAyLTM4LjIgMTAyIDM4LjJ2LjZsLTEwMiA0MS40LTEwMi00MS40di0uNnptODQgMjkxLjFsLTg1IDQyLjV2LTc5LjFsODUtMzguOHY3NS40em0wLTExMmwtMTAyIDQxLjQtMTAyLTQxLjR2LS42bDEwMi0zOC4yIDEwMiAzOC4ydi42em0yNDAgMTEybC04NSA0Mi41di03OS4xbDg1LTM4Ljh2NzUuNHptMC0xMTJsLTEwMiA0MS40LTEwMi00MS40di0uNmwxMDItMzguMiAxMDIgMzguMnYuNnoiPjwvcGF0aD48L3N2Zz4K&style=for-the-badge)](https://docs.rs/ferinth/)

Ferinth is a simple library for using the Modrinth API in Rust projects. It uses `reqwest` as its HTTP(S) client and deserialises responses to typed structs using `serde`.

As of now, the following features have been implemented
- All structure definitions based on https://github.com/modrinth/labrinth/wiki/API-Documentation#structure-definitions
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

Unfortunately, I am not planning to implement any of these features in version 1 due to poor documentation. I will add these features in version 2.

## Use

**The major version of this crate's version directly corresponds to the API version it uses**. So if you want to use the Modrinth API version 1, then specify this crate version as `"1"`, and so on.  
After you have decided the API version to use, you can add this crate to your `Cargo.toml` like so
```toml
[dependencies]
ferinth = "1"
```
