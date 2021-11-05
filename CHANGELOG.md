# Changelog

## [1.2.1] - 05.11.2021

- This crate now makes `reqwest` use `rustls` because **OpenSSL = bad**
- Made API calls' return syntaz more concise

## [1.2.0] - 03.11.2021

- Made date/time fields use Chrono's `DateTime<Utc>`
- Made documentation better and more consistent
- Added an example project
- Prepared the project for crates.io release

## [1.1.0] - 02.11.2021

- Renamed the library to Ferinth (`ferinth`)
- All names `ModrinthAPI` have been renamed to `Ferinth	
- Error checking removed because only `reqwest::Error`s are encountered
- All `api_call`s now accept `&str` only
- Only `structures` is exported
- Added `list_team_members()` API call
- Added `TeamMember` and `TeamRole`
- Made version file hashes a new struct

## [1.0.0] - 01.11.2021

First commit
