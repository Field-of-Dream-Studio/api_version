# av — attribute macros for API documentation

A suite of five Rust proc-macro attributes that document API contracts the
type system cannot express. Each macro renders a structured rustdoc section
on the item it decorates.

| Macro         | Section                       | Purpose                                    |
|---------------|-------------------------------|--------------------------------------------|
| `#[ver]`      | Version History (current)     | The current version of this API            |
| `#[verlog]`   | Version History (historical)  | A past version entry                        |
| `#[safety]`   | Safety                        | What the caller must uphold for `unsafe fn` |
| `#[panics]`   | Panics                        | When and why the function panics            |
| `#[author]`   | Authors                       | Who owns this API                           |

## Install

```toml
[dependencies]
av = "0.2"
```

## Quick tour

Each macro is independently useful. Add whichever ones you need.

```rust
use av::{author, panics, safety, ver, verlog};

// #[ver] — one entry per item, renders a highlighted version heading.
// When the status is `deprecated`, a `#[deprecated]` attribute is emitted too.
#[ver(stable, since = "0.2.0")]
pub fn versioned() {}

// #[verlog] — historical entries, stacked. Same field shape as #[ver],
// but never emits `#[deprecated]`.
#[ver(stable, since = "1.0.0", note = "First stable release")]
#[verlog(unstable, since = "0.1.0", note = "Initial implementation")]
pub fn history() {}

// #[safety] — comma-separated list of preconditions rendered as a
// numbered `# Safety` section. See "Writing conditions" below.
#[safety("ptr is non-null")]
pub unsafe fn peek(_ptr: *const u8) {}

// #[panics] — comma-separated list of panic conditions.
// Sentinels: `never` / `none` for a non-panicking fn; `always` for a fn
// that panics unconditionally.
#[panics("the input is zero")]
pub fn may_panic(_input: u32) {}

#[panics(never)]
pub fn total() {}

#[panics(always)]
pub fn unreachable_marker() { panic!("not implemented") }

// #[author] — one entry per attribute (stacked for multiple).
// Fields: name (required), email, github, role (optional).
#[author(name = "Redstone", email = "redstone@example.com")]
pub fn authored() {}
```

Version-entry fields:
`status, since = "…" [, note = "…"] [, date = "…"]`.
Statuses: `unstable`, `stable`, `update`, `update_unstable`, `deprecated`
(case-insensitive). Authorship is a separate concern — use `#[author]`.

## Writing conditions

`#[safety]` and `#[panics]` each render their conditions after a fixed
preamble, followed by a numbered list:

- `#[safety(...)]` renders `The caller must uphold:`
- `#[panics(...)]` renders `This function panics when:`

**Write each condition as a bare clause that completes the preamble.**
Do not repeat the preamble; do not begin with `if` / `when`; do not phrase
as an error message. The preamble already supplies the frame.

```rust
// good — reads as "The caller must uphold: ptr is non-null"
#[safety("ptr is non-null")]

// good — reads as "This function panics when: the input is zero"
#[panics("the input is zero")]

// bad — reads as "This function panics when: when the input is zero"
#[panics("when the input is zero")]

// bad — reads as "This function panics when: Failed to open the file"
#[panics("Failed to open the file")]
```

Sentinels cover the trivial cases without a condition string:

- `#[panics(never)]` (or `#[panics(none)]`): the function does not panic.
- `#[panics(always)]`: the function panics on every invocation.

## Ordering of stacked attributes

**The attribute stack reads top-to-bottom in the same order as the rendered
docs.** Each macro inserts its output AFTER any user `///` doc lines and
AFTER the previous macro's output, so source order = rendered order.

For an `unsafe fn` where callers should see the description first, then
safety information, panic conditions, current version, history, and
authorship last:

```rust
#[ver(update, since = "1.3.0", note = "Renamed parameter for clarity")]
#[verlog(update, since = "1.2.0", note = "Added a debug-build bounds check")]
#[verlog(stable, since = "1.0.0", note = "Stabilised after review")]
#[verlog(unstable, since = "0.1.0", note = "Initial API")]
#[author(name = "Redstone", email = "redstone@example.com", role = "maintainer")]
#[safety(
    "src is a valid pointer to at least count readable bytes",
    "dst is a valid pointer to at least count writable bytes",
    "src and dst regions do not overlap"
)]
#[panics(
    "count exceeds 1 << 30 on debug builds",
    "the source or destination pointer is null"
)]
/// Copy `count` bytes from `src` to `dst`.
///
/// This function is a thin wrapper over `core::ptr::copy_nonoverlapping`.
pub unsafe fn copy_bytes(src: *const u8, dst: *mut u8, count: usize) {
    core::ptr::copy_nonoverlapping(src, dst, count);
}
```

Rendered (top to bottom) as:

1. User description
2. `## Updated Version: 1.3.0` — highlighted current
3. `Version: 1.2.0, **Update**`
4. `Version: 1.0.0, **Stable**`
5. `Version: 0.1.0, **Unstable**`
6. `# Authors` — bulleted list
7. `# Safety` — three numbered conditions
8. `# Panics` — two numbered conditions

See `examples/complete.rs` for the runnable version.

Convention: place `///` doc lines **directly above the `fn`** (below the
macro attribute stack). With many attributes, this keeps description and
signature adjacent instead of separating them by a wall of attributes.
The rendered output is identical whether `///` is written above or below
the attributes.

## Features

- **`deprecated_for_unstable`** (off by default) — when enabled,
  `#[ver(unstable, …)]` also emits a `#[deprecated]` attribute with an
  `[UNSTABLE]` note prefix, surfacing unstable APIs as compiler warnings
  at call sites.

## Migrating from 0.1

Two breaking changes:

- **The `;`-separated multi-entry form of `#[ver]` is removed.** Each
  version entry now lives on its own attribute: one `#[ver]` for the
  current version, plus stacked `#[verlog]` attributes for older entries.
- **The per-version `author = "…"` field is removed.** Use the standalone
  `#[author]` macro instead.

```rust
// 0.1
#[ver(
    stable,   since = "1.1.0", note = "Stabilised", author = "Akari";
    unstable, since = "0.1.0", note = "Prototype", author = "Redstone"
)]

// 0.2
#[ver(stable, since = "1.1.0", note = "Stabilised")]
#[verlog(unstable, since = "0.1.0", note = "Prototype")]
#[author(name = "Akari")]
``` 

## License

MIT — see `LICENSE`.
