# Ferinth

[![github badge](https://img.shields.io/badge/GitHub-Ferinth-informational?style=for-the-badge&logo=github&labelColor=555555)](https://github.com/theRookieCoder/ferinth/)
[![crates badge](https://img.shields.io/crates/v/ferinth?logo=rust&style=for-the-badge)](https://crates.io/crates/ferinth/)
[![docs.rs](https://img.shields.io/docsrs/ferinth?logo=data%3Aimage%2Fsvg%2Bxml%3Bbase64%2CPHN2ZyByb2xlPSJpbWciIHhtbG5zPSJodHRwOi8vd3d3LnczLm9yZy8yMDAwL3N2ZyIgdmlld0JveD0iMCAwIDUxMiA1MTIiPjxwYXRoIGZpbGw9IiNmNWY1ZjUiIGQ9Ik00ODguNiAyNTAuMkwzOTIgMjE0VjEwNS41YzAtMTUtOS4zLTI4LjQtMjMuNC0zMy43bC0xMDAtMzcuNWMtOC4xLTMuMS0xNy4xLTMuMS0yNS4zIDBsLTEwMCAzNy41Yy0xNC4xIDUuMy0yMy40IDE4LjctMjMuNCAzMy43VjIxNGwtOTYuNiAzNi4yQzkuMyAyNTUuNSAwIDI2OC45IDAgMjgzLjlWMzk0YzAgMTMuNiA3LjcgMjYuMSAxOS45IDMyLjJsMTAwIDUwYzEwLjEgNS4xIDIyLjEgNS4xIDMyLjIgMGwxMDMuOS01MiAxMDMuOSA1MmMxMC4xIDUuMSAyMi4xIDUuMSAzMi4yIDBsMTAwLTUwYzEyLjItNi4xIDE5LjktMTguNiAxOS45LTMyLjJWMjgzLjljMC0xNS05LjMtMjguNC0yMy40LTMzLjd6TTM1OCAyMTQuOGwtODUgMzEuOXYtNjguMmw4NS0zN3Y3My4zek0xNTQgMTA0LjFsMTAyLTM4LjIgMTAyIDM4LjJ2LjZsLTEwMiA0MS40LTEwMi00MS40di0uNnptODQgMjkxLjFsLTg1IDQyLjV2LTc5LjFsODUtMzguOHY3NS40em0wLTExMmwtMTAyIDQxLjQtMTAyLTQxLjR2LS42bDEwMi0zOC4yIDEwMiAzOC4ydi42em0yNDAgMTEybC04NSA0Mi41di03OS4xbDg1LTM4Ljh2NzUuNHptMC0xMTJsLTEwMiA0MS40LTEwMi00MS40di0uNmwxMDItMzguMiAxMDIgMzguMnYuNnoiPjwvcGF0aD48L3N2Zz4K&style=for-the-badge)](https://docs.rs/ferinth/)

Ferinth is a simple library to use the Modrinth API in Rust projects

It provides API bindings for the [Modrinth API](https://docs.modrinth.com), is intuitive to use, and provides typed structs for all the structures used.

## Use

**The major version of this crate's version directly corresponds to the Modrinth API version it uses**. So for example if you want to use the Modrinth API version 2, which is the latest one available now, then specify this crate version as `2`.  
After you have decided the API version to use, you can add this crate to your `Cargo.toml` like so
```toml
[dependencies]
ferinth = "2" # For API version 2
```

### Builing
Currently the Dotium dependency is added by path for easier development, it needs to be cloned into the same directory as Ferinth.
```
git clone https://github.com/gorilla-devs/dotium.git
cd dotium
git checkout lib-2-lib-dotium
cd ..
git clone https://github.com/gorilla-devs/ferinth.git
cd ferinth
git checkout possibly-ferinth-rewrite
```