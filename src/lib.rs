//! <div align="center">
//!  <img src="https://github.com/mitsuhiko/insta/blob/master/assets/logo.png?raw=true" width="250" height="250">
//!  <p><strong>insta: a snapshot testing library for Rust</strong></p>
//! </div>
//!
//! # What are snapshot tests
//!
//! Snapshots tests (also sometimes called approval tests) are tests that
//! assert values against a reference value (the snapshot).  This is similar
//! to how `assert_eq!` lets you compare a value against a reference value but
//! unlike simple string assertions snapshot tests let you test against complex
//! values and come with comprehensive tools to review changes.
//!
//! Snapshot tests are particularly useful if your reference values are very
//! large or change often.
//!
//! # How it operates
//!
//! This crate exports multiple macros for snapshot testing:
//!
//! - `assert_snapshot!` for comparing basic string snapshots.
//! - `assert_debug_snapshot!` for comparing `Debug` outputs of values.
//! - `assert_display_snapshot!` for comparing `Display` outputs of values.
//! - `assert_yaml_snapshot!` for comparing YAML serialized
//!   output of types implementing `serde::Serialize`.
//! - `assert_ron_snapshot!` for comparing RON serialized output of
//!   types implementing `serde::Serialize`. (requires the `ron` feature)
//! - `assert_json_snapshot!` for comparing JSON serialized output of
//!   types implementing `serde::Serialize`.
//!
//! Snapshots are stored in the `snapshots` folder right next to the test file
//! where this is used.  The name of the file is `<module>__<name>.snap` where
//! the `name` of the snapshot.  Snapshots can either be explicitly named or the
//! name is derived from the test name.
//!
//! Additionally snapshots can also be stored inline.  In that case the
//! [`cargo-insta`](https://crates.io/crates/cargo-insta) tool is necessary.
//! See [inline snapshots](#inline-snapshots) for more information.
//!
//! For macros that work with `serde::Serialize` this crate also permits
//! redacting of partial values.  See [redactions](#redactions) for more
//! information.
//!
//! # What it looks like:
//!
//! This is what the cargo insta review UI looks like:
//!
//! <img src="https://github.com/mitsuhiko/insta/blob/master/assets/review.png?raw=true" alt="">
//!
//! There is also a screencast that shows the entire workflow: [watch the insta
//! introduction screencast](https://www.youtube.com/watch?v=7mIDiKK4eyo&feature=youtu.be)
//!
//! # Example
//!
//! Install `insta`:
//!
//! Recommended way if you have `cargo-edit` installed:
//!
//! ```ignore
//! $ cargo add --dev insta
//! ```
//!
//! Alternatively edit your `Cargo.toml` manually and add `insta` as manual
//! dependency.
//!
//! And for an improved review experience also install `cargo-insta`:
//!
//! ```ignore
//! $ cargo install cargo-insta
//! ```
//!
//! ```rust,ignore
//! use insta::assert_debug_snapshot;
//!
//! #[test]
//! fn test_snapshots() {
//!     let value = vec![1, 2, 3];
//!     assert_debug_snapshot!(value);
//! }
//! ```
//!
//! The recommended flow is to run the tests once, have them fail and check
//! if the result is okay.  By default the new snapshots are stored next
//! to the old ones with the extra `.new` extension.  Once you are satisifed
//! move the new files over.  To simplify this workflow you can use
//! `cargo insta review` which will let you interactively review them:
//!
//! ```ignore
//! $ cargo test
//! $ cargo insta review
//! ```
//!
//! For more information on updating see [Snapshot Updating].
//!
//! [Snapshot Updating]: #snapshot-updating
//!
//! # Snapshot files
//!
//! The committed snapshot files will have a header with some meta information
//! that can make debugging easier and the snapshot:
//!
//! ```ignore
//! ---
//! expression: "&User{id: Uuid::new_v4(), username: \"john_doe\".to_string(),}"
//! source: tests/test_user.rs
//! ---
//! [
//!     1,
//!     2,
//!     3
//! ]
//! ```
//!
//! # Snapshot updating
//!
//! During test runs snapshots will be updated according to the `INSTA_UPDATE`
//! environment variable.  The default is `auto` which will write all new
//! snapshots into `.snap.new` files if no CI is detected.
//!
//! `INSTA_UPDATE` modes:
//!
//! - `auto`: the default. `no` for CI environments or `new` otherwise
//! - `always`: overwrites old snapshot files with new ones unasked
//! - `new`: write new snapshots into `.snap.new` files.
//! - `no`: does not update snapshot files at all (just runs tests)
//!
//! When `new` is used as mode the `cargo-insta` command can be used to review
//! the snapshots conveniently:
//!
//! ```ignore
//! $ cargo install cargo-insta
//! $ cargo test
//! $ cargo insta review
//! ```
//!
//! "enter" or "a" accepts a new snapshot, "escape" or "r" rejects,
//! "space" or "s" skips the snapshot for now.
//!
//! For more information invoke `cargo insta --help`.
//!
//! # Test assertions
//!
//! By default the tests will fail when the snapshot assertion fails.  However
//! if a test produces more than one snapshot it can be useful to force a test
//! to pass so that all new snapshots are created in one go.
//!
//! This can be enabled by setting `INSTA_FORCE_PASS` to `1`:
//!
//! ```ignore
//! $ INSTA_FORCE_PASS=1 cargo test --no-fail-fast
//! ```
//!
//! A better way to do this is to run `cargo insta test --review` which will
//! run all tests with force pass and then bring up the review tool:
//!
//! ```ignore
//! $ cargo insta test --review
//! ```
//!
//! # Named snapshots
//!
//! All snapshot assertion functions let you leave out the snapshot name in
//! which case the snapshot name is derived from the test name (with an optional
//! leading `test_` prefix removed.
//!
//! This works because the rust test runner names the thread by the test name
//! and the name is taken from the thread name.  In case your test spawns additional
//! threads this will not work and you will need to provide a name explicitly.
//!
//! Explicit snapshot naming can also otherwise be useful to be more explicit
//! when multiple snapshots are tested within one function as the default
//! behavior would be to just count up the snapshot names.
//!
//! To provide an explicit name provide the name of the snapshot as first
//! argument to the macro:
//!
//! ```rust,ignore
//! #[test]
//! fn test_something() {
//!     assert_snapshot!("first_snapshot", "first value");
//!     assert_snapshot!("second_snapshot", "second value");
//! }
//! ```
//!
//! This will create two snapshots: `first_snapshot` for the first value and
//! `second_snapshot` for the second value.  Without explicit naming the
//! snapshots would be called `something` and `something-2`.
//!
//! # Redactions
//!
//! **Feature:** `redactions`
//!
//! For all snapshots created based on `serde::Serialize` output `insta`
//! supports redactions.  This permits replacing values with hardcoded other
//! values to make snapshots stable when otherwise random or otherwise changing
//! values are involved.  Redactions became an optional feature in insta
//! 0.11 and can be enabled with the `redactions` feature./
//!
//! Redactions can be defined as the third argument to those macros with
//! the syntax `{ selector => replacement_value }`.
//!
//! The following selectors exist:
//!
//! - `.key`: selects the given key
//! - `["key"]`: alternative syntax for keys
//! - `[index]`: selects the given index in an array
//! - `[]`: selects all items on an array
//! - `[:end]`: selects all items up to `end` (excluding, supports negative indexing)
//! - `[start:]`: selects all items starting with `start`
//! - `[start:end]`: selects all items from `start` to `end` (end excluding,
//!   supports negative indexing).
//! - `.*`: selects all keys on that depth
//!
//! Example usage:
//!
//! ```rust,ignore
//! #[derive(Serialize)]
//! pub struct User {
//!     id: Uuid,
//!     username: String,
//!     extra: HashMap<String, String>,
//! }
//!
//! assert_yaml_snapshot!(&User {
//!     id: Uuid::new_v4(),
//!     username: "john_doe".to_string(),
//!     extra: {
//!         let mut map = HashMap::new();
//!         map.insert("ssn".to_string(), "123-123-123".to_string());
//!         map
//!     },
//! }, {
//!     ".id" => "[uuid]",
//!     ".extra.ssn" => "[ssn]"
//! });
//! ```
//!
//! It's also possible to execute a callback that can produce a new value
//! instead of hardcoding a replacement value by using the
//! [`dynamic_redaction`](fn.dynamic_redaction.html) function:
//!
//! ```rust,ignore
//! # #[derive(Serialize)]
//! # pub struct User {
//! #     id: Uuid,
//! #     username: String,
//! # }
//! assert_yaml_snapshot!(&User {
//!     id: Uuid::new_v4(),
//!     username: "john_doe".to_string(),
//! }, {
//!     ".id" => dynamic_redaction(|value, _| {
//!         // assert that the value looks like a uuid here
//!         "[uuid]"
//!     }),
//! });
//! ```
//!
//! # Inline Snapshots
//!
//! Additionally snapshots can also be stored inline.  In that case the format
//! for the snapshot macros is `assert_snapshot!(reference_value, @"snapshot")`.
//! The leading at sign (`@`) indicates that the following string is the
//! reference value.  `cargo-insta` will then update that string with the new
//! value on review.
//!
//! Example:
//!
//! ```rust,ignore
//! #[derive(Serialize)]
//! pub struct User {
//!     username: String,
//! }
//!
//! assert_yaml_snapshot!(User {
//!     username: "john_doe".to_string(),
//! }, @"");
//! ```
//!
//! After the initial test failure you can run `cargo insta review` to
//! accept the change.  The file will then be updated automatically.
//!
//! # Features
//!
//! The following features exist:
//!
//! * `ron`: enables RON support (`assert_ron_snapshot!`)
//! * `redactions`: enables support for redactions
//!
//! # Settings
//!
//! There are some settings that can be changed on a per-thread (and thus
//! per-test) basis.  For more information see [settings](struct.Settings.html).
//!
//! # Legacy Snapshot Formats
//!
//! With insta 0.11 the snapshot format was improved for inline snapshots.  The
//! old snapshot format will continue to be available but if you want to upgrade
//! them make sure the tests pass first and then run the following command
//! to force a rewrite of them all:
//!
//! ```text,ignore
//! $ cargo insta test --accept --force-update-snapshots
//! ```
#[macro_use]
mod macros;
mod content;
mod runtime;
mod serialization;
mod settings;
mod snapshot;
mod utils;

#[macro_use]
mod legacy_macros;

#[cfg(feature = "redactions")]
mod redaction;

#[cfg(test)]
mod test;

pub use crate::settings::Settings;
pub use crate::snapshot::{MetaData, Snapshot};

/// Exposes some library internals.
///
/// You're unlikely to want to work with these objects but they
/// are exposed for documentation primarily.
pub mod internals {
    pub use crate::content::Content;
    pub use crate::runtime::AutoName;
    pub use crate::snapshot::{MetaData, SnapshotContents};
    #[cfg(feature = "redactions")]
    pub use crate::{
        redaction::{ContentPath, Redaction},
        settings::Redactions,
    };
}

// exported for cargo-insta only
#[doc(hidden)]
pub use crate::{
    runtime::print_snapshot_diff, snapshot::PendingInlineSnapshot, snapshot::SnapshotContents,
};

// useful for redactions
#[cfg(feature = "redactions")]
pub use crate::redaction::dynamic_redaction;

// these are here to make the macros work
#[doc(hidden)]
pub mod _macro_support {
    pub use crate::content::Content;
    pub use crate::runtime::{assert_snapshot, AutoName, ReferenceValue};
    pub use crate::serialization::{serialize_value, SerializationFormat, SnapshotLocation};

    #[cfg(feature = "redactions")]
    pub use crate::{
        redaction::Redaction, redaction::Selector, serialization::serialize_value_redacted,
    };
}
