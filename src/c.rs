//! Contains functions imported from C.

// Duration definition from POSIX C.
#[repr(C)]
pub struct TimeSpec {
    pub seconds: i64,
    pub nanos: i64
}

#[link(name = "c")]
extern "C" {
    // Import printf and putchar from C so we can print to the console,
    pub fn printf(string: *const u8, ...) -> i32;
    pub fn putchar(chr: i32) -> i32;
    // malloc so we can allocate memory,
    pub fn malloc(size: usize) -> *mut ();
    // and free so we can deallocate it
    pub fn free(ptr: *mut ());
    // String conversion functions
    pub fn atof(string: *const u8) -> f64;
    pub fn atoi(string: *const u8) -> i32;
    // Randomness
    pub fn srand(seed: u32);
    pub fn rand() -> i32;
    // and timekeeping
    pub fn time(time: *const i64) -> i64;
    pub fn nanosleep(time: *const TimeSpec, rem: *mut TimeSpec);
    // For unexpected outcomes
    pub fn abort() -> !;
}