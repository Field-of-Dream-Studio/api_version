//! # av — attribute macros for API documentation
//!
//! A suite of five attribute macros that document API contracts the type system
//! cannot express. Each macro renders a structured rustdoc section on the item it
//! decorates.
//!
//! | Macro | Section | Purpose |
//! |---|---|---|
//! | [`macro@ver`]     | Version History (current)    | The current version of this API |
//! | [`macro@verlog`]  | Version History (historical) | A past version entry |
//! | [`macro@safety`]  | Safety                       | What the caller must uphold for `unsafe fn` |
//! | [`macro@panics`]  | Panics                       | When and why the function panics |
//! | [`macro@author`]  | Authors                      | Who owns this API |
//!
//! ## Cargo features
//!
//! - `deprecated_for_unstable` (off by default) — when enabled,
//!   `#[ver(unstable, ...)]` also emits a `#[deprecated]` attribute with an
//!   `[UNSTABLE]` note prefix, so callers of unstable APIs get a compiler warning.

use proc_macro::TokenStream;

use crate::helper::insert_docs_before_body;

pub(crate) mod helper;
pub(crate) mod fields;
pub(crate) mod versioning;

pub(crate) mod doc_section;
pub(crate) mod safety;
pub(crate) mod panics;
pub(crate) mod author;
pub(crate) mod cite;

/// Record the current version of an API item.
///
/// Conventionally, use one `#[ver(...)]` per item. It renders a highlighted version heading;
/// when the status is `deprecated` (or `unstable` under the
/// `deprecated_for_unstable` feature), also emits a `#[deprecated]` attribute.
///
/// Grammar: `status, since = "…" [, note = "…"] [, date = "…"]`. Statuses are
/// `unstable`, `stable`, `update`, `update_unstable`, `deprecated` (case-insensitive).
/// Authorship is a separate concern — see [`macro@author`].
///
/// Historical version entries live on stacked `#[verlog(...)]` attributes.
///
/// ```ignore
/// #[ver(stable, since = "1.1.0", note = "Stabilised")]
/// #[verlog(unstable, since = "0.1.0", note = "Prototype")]
/// pub fn my_api() { }
/// ```
///
/// **Ordering:** the attribute stack reads top-to-bottom in the same order the
/// rendered docs appear. Put `#[ver]` above older `#[verlog]` entries for
/// newest-first history. Every macro in this suite inserts its docs AFTER any
/// user `///` lines, so the user's description always comes first.
///
/// The `;`-separated multi-entry form (`#[ver(a; b; c)]`) from earlier versions
/// is no longer accepted.
#[proc_macro_attribute]
pub fn ver(attr: TokenStream, item: TokenStream) -> TokenStream {
    let mut cursor = attr.into_iter().peekable();
    match versioning::generate_ver_docs(&mut cursor) {
        Ok(docs) => insert_docs_before_body(item, docs),
        Err(err) => err,
    }
}

/// Record a historical version entry, rendered as a plain (non-highlighted) log line.
///
/// Zero or more `#[verlog(...)]` per item. Never emits `#[deprecated]` — that role
/// belongs solely to `#[ver]`. Same field shape as `#[ver]`.
///
/// See [`macro@ver`] for the ordering convention (the attribute stack reads in
/// the same order as the rendered docs).
///
/// ```ignore
/// #[ver(update, since = "1.1.0", note = "Added new parameter")]
/// #[verlog(stable, since = "1.0.0", note = "First stable release")]
/// #[verlog(unstable, since = "0.1.0", note = "Prototype")]
/// pub fn my_api() { }
/// ```
#[proc_macro_attribute]
pub fn verlog(attr: TokenStream, item: TokenStream) -> TokenStream {
    let mut cursor = attr.into_iter().peekable();
    match versioning::generate_verlog_docs(&mut cursor) {
        Ok(docs) => insert_docs_before_body(item, docs),
        Err(err) => err,
    }
}

/// Document the preconditions a caller must uphold when invoking an `unsafe fn`.
///
/// Accepts a comma-separated list of string literals; each is rendered as one entry
/// in a numbered list under `# Safety`.
///
/// ```ignore
/// #[safety("ptr is a valid pointer to at least len bytes")]
/// pub unsafe fn peek(ptr: *const u8, len: usize) -> u8 { ... }
/// ```
#[proc_macro_attribute]
pub fn safety(attr: TokenStream, item: TokenStream) -> TokenStream {
    let mut cursor = attr.into_iter().peekable();
    match safety::generate_safety_docs(&mut cursor) {
        Ok(docs) => insert_docs_before_body(item, docs),
        Err(err) => err,
    }
}

/// Document when a function panics.
///
/// Accepts a comma-separated list of string literals rendered under `# Panics`, or
/// one of these bare sentinels:
///
/// - `never` / `none` documents that the function does not panic.
/// - `always` documents that the function panics on every invocation.
///
/// ```ignore
/// #[panics("when index is out of bounds", "when the lock is poisoned")]
/// pub fn get(index: usize) -> u8 { ... }
///
/// #[panics(never)]
/// pub fn try_get() -> Option<u8> { ... }
///
/// #[panics(always)]
/// pub fn not_implemented() -> ! { panic!("not implemented") }
/// ```
///
/// Sentinels are exclusive and cannot be combined with condition strings.
///
/// ```compile_fail
/// use av::panics;
///
/// #[panics(always, "a fallback condition")]
/// pub fn invalid() { panic!("not implemented") }
/// ```
#[proc_macro_attribute]
pub fn panics(attr: TokenStream, item: TokenStream) -> TokenStream {
    let mut cursor = attr.into_iter().peekable();
    match panics::generate_panics_docs(&mut cursor) {
        Ok(docs) => insert_docs_before_body(item, docs),
        Err(err) => err,
    }
}

/// Record the author(s) of a function or type under `# Authors`.
///
/// Each `#[author(...)]` records one entry. Fields: `name` (required),
/// `email`, `github`, `role` (optional).
///
/// ```ignore
/// #[author(name = "Redstone", email = "redstone@example.com")]
/// #[author(name = "Akari", github = "akari", role = "maintainer")]
/// pub fn my_api() { }
/// ```
///
/// **Note:** each stacked `#[author]` currently emits its own `# Authors`
/// heading, so multi-author items render with a repeated heading. A future
/// release will merge sibling `#[author]` attributes into one section — see
/// the TODO in `src/author.rs`.
#[proc_macro_attribute]
pub fn author(attr: TokenStream, item: TokenStream) -> TokenStream {
    let mut cursor = attr.into_iter().peekable();
    match author::generate_author_docs(&mut cursor) {
        Ok(docs) => insert_docs_before_body(item, docs),
        Err(err) => err,
    }
}

/// Cite an external written work — book, paper, RFC, spec, blog post, tutorial
/// — that discusses or reproduces this item. Renders under `# Reference`.
///
/// Fields: `work` (required); optional `volume`, `edition`, `chapter`,
/// `section`, `listing`, `figure`, `page`, `caption`, `doi`, `url`.
/// All values are string literals — see `docs/cite.md` for rationale.
///
/// ```ignore
/// #[cite(work = "OSRust", chapter = "6.3", listing = "6.14", page = "137")]
/// #[cite(work = "SafeWrappers2026", section = "3.2")]
/// pub fn safe_pipe_split() { ... }
/// ```
///
/// **Note:** stacked `#[cite]`s each emit their own `# Reference` heading —
/// same sibling-blindness caveat as [`macro@author`]. See the TODO in
/// `src/cite.rs` for the intended merge behaviour.
#[proc_macro_attribute]
pub fn cite(attr: TokenStream, item: TokenStream) -> TokenStream {
    let mut cursor = attr.into_iter().peekable();
    match cite::generate_cite_docs(&mut cursor) {
        Ok(docs) => insert_docs_before_body(item, docs),
        Err(err) => err,
    }
}
