# Changelog

## `2.7.0`
### 01.10.2022

- Update dependencies
- Add support for authorised requests
- Remove `_calls` and `_structs` from filenames
- Change `resolve_id_slug()` to `does_exist()`
- Add `follow()`, `add_user()`, `list_multiple_teams_members()`, `join_team()`, `transfer_ownership()`, `get_current_user()`, `get_notification()`, `followed_projects()`, and `submit_report()`
- Add `ProjectType::Plugin`, `FileExt`, `Notification`, `Report`, `ReportSubmission`, `NotificationAction`, `ReportItemType`, `NotificationType`

## `2.6.0`
### 12.08.2022

- Update dependencies
- Add a [new API call](https://docs.modrinth.com/api-spec/#tag/projects/operation/checkProjectValidity)
- Add resource packs to the project types enum
- Make `ProjectType`, `UserRole`, and `DependencyType` non-exhaustive

## `2.5.1`
### 10.07.2022

- Fixed a bug where if an empty string was provided for an `Option<URL>` field, the deserialiser would try to parse the url and error out rather than return `None`
- Renamed the `Datetime` alias to `UtcTime`

## `2.5.0`
### 04.07.2022

- Manually implement `Default` for `Ferinth` so that the user agent is set to `crate_name/version`
- Add `Ferinth::new` with a user agent based on the [documentation](https://docs.modrinth.com/api-spec/#section/User-Agents)
- Added `DependencyType::Embedded` to fix [gorilla-devs/ferium#169](https://github.com/gorilla-devs/ferium/issues/169)

## `2.4.0`
### 19.06.2022

- [#7](https://github.com/gorilla-devs/ferinth/pull/7) File hashes are no longer behind an `Option`
- Remove the `Option` from the `Project.donation_urls` vector
- Replace `String` with `url::Url` where appropriate

## `2.3.0`
### 12.06.2022

- Changed the way URLs are constructed and requests are made
- Specified dependencies properly
- Replaced `Furse::new()` with `Default`
- Added all the new API calls (that do not need authentication)
- Slugs with underscores are now considered valid

## `2.2.1`
### 08.05.2022

Add support for rate limits using a new error

## `2.2.0`
### 03.05.2022

Update all structs to the latest <https://docs.modrinth.com/api-spec> specifications (Labrinth `v2.2.4`)

## `2.1.1`
### 01.05.2022

- Update description
- Fix `Ferinth` doctest

## [2.1.0] - 26.03.2022

- Removed example project and the one in the description
- Added `get_project_dependencies()`
- Moved `ProjectType` to _project_structs.rs_
- Added new tag calls and their respective structs
- Made tag call struct fields public
- Improved documentation
- Removed user agent

## [2.0.1] - 25.03.2022

- All enums now consistently have `PartialEq` and `Eq` implemented
- All structs now have proper doc comments

## [2.0.0] - 03.02.2022

Update to the [Modrinth API v2](https://docs.modrinth.com). The most breaking changes are the following:

- `get_mod()` is now `get_project()`
- Most of the structure definition have had fields added/changed

A lot of the changed are under the hood

Check out [PR #4](https://github.com/theRookieCoder/ferinth/pull/4) to see all the changes made

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
