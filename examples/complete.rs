//! Complete example — one unsafe function annotated with every macro in the suite.
//!
//! **Ordering.** Each macro inserts its docs AFTER any user `///` doc lines and
//! AFTER the previous macro's output, so the attribute stack reads top-to-bottom
//! in the same order as the rendered docs. Rendered, in order:
//!
//! 1. User `///` description lines
//! 2. `## Updated Version: 1.3.0` — highlighted current version
//! 3. `Version: 1.2.0, **Update**` — historical, newest first
//! 4. `Version: 1.0.0, **Stable**`
//! 5. `Version: 0.1.0, **Unstable**` — oldest
//! 6. `# Authors`
//! 7. `# Safety`
//! 8. `# Panics`

use av::{author, panics, safety, ver, verlog};

#[ver(update, since = "1.3.0", note = "Renamed parameter for clarity", date = "2026-08-16")]
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
#[allow(unsafe_op_in_unsafe_fn)]
/// Copy `count` bytes from `src` to `dst`.
///
/// This function is a thin wrapper over `core::ptr::copy_nonoverlapping`.
pub unsafe fn copy_bytes(src: *const u8, dst: *mut u8, count: usize) {
    core::ptr::copy_nonoverlapping(src, dst, count);
}

fn main() {
    let src = [1u8, 2, 3, 4];
    let mut dst = [0u8; 4];
    unsafe { copy_bytes(src.as_ptr(), dst.as_mut_ptr(), src.len()); }
    println!("{dst:?}");
}
