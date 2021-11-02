# Changelog

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
