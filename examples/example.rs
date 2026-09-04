use av::{author, cite, panics, safety, ver, verlog};

#[ver(stable, since = "0.2.0")]
pub fn versioned() {}

// Ordering: attribute stack reads top-to-bottom in the same order as the rendered
// docs. Convention for this crate: #[ver] above older #[verlog] entries for
// newest-first history, then #[author], then #[safety]/#[panics].
#[ver(update, since = "1.2.0", note = "Added new parameter")]
#[verlog(stable, since = "1.1.0", note = "First stable release")]
#[verlog(unstable, since = "0.1.0", note = "Initial implementation")]
#[author(name = "Akari")]
pub fn stacked_history(_value: i32) {}

#[safety("caller must ensure ptr is non-null")]
pub unsafe fn safe_annotated(_ptr: *const u8) {}

#[panics("when the input is zero")]
pub fn may_panic(_input: u32) {}

#[panics(never)]
pub fn never_panics() {}

#[panics(always)]
pub fn always_panics() -> ! {
    panic!("not implemented")
}

#[author(name = "Redstone", email = "redstone@example.com")]
pub fn authored() {}

#[cite(work = "OSRust", chapter = "6.3", listing = "6.14", page = "137",
       doi = "10.1145/1234567.1234568")]
pub fn cited() {}

fn main() {
    versioned();
    stacked_history(0);
    unsafe { safe_annotated(core::ptr::null()); }
    may_panic(1);
    never_panics();
    authored();
    cited();
}
