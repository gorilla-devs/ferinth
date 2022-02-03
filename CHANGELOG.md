# Changelog

## [1.4.2] - 02.02.2022

- Implement `Clone` and `Debug` for `Ferinth`

## [1.4.1] - 28.01.2022

- Edit doctest for `user_calls::list_mods()` so that the order of the mod_ids returned doesn't matter
- The following were implemented as per a suggestion by [4JX](https://github.com/4JX) [here](https://github.com/theRookieCoder/ferinth/pull/1)
- Removed `regex` and used `lazy-regex` which is faster as the regexes are parsed at compile time
- Update base62 and SHA1 checkers to use `lazy_regex::regex_is_match` for much faster and more readable code

## [1.4.0] - 27.01.2022

- New error `NotSHA1` to show that an argument provided an invalid SHA1 hash
- Added a new function to verify that a string is SHA1 compliant
- Added new api call, `get_version_from_file_hash()` which gets the version of a version file from the version file's hash ([PR #1](https://github.com/theRookieCoder/ferinth/pull/1) by [@4JX](https://github.com/4JX))

## [1.3.0] - 11.11.2021

- Added doctests for every method and struct, which means more documentation and tests!
- Update readme and docs front page
- Added more extensive error handling
- Made many return statements more concise
- Methods accepting IDs now verify that they are base62 compliant using a RegEx
- Added `Clone` to all structs and enums
- Added `PartialEq` to all enums

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
