#![allow(internal_features)]
#![feature(no_core, lang_items, rustc_attrs, auto_traits, intrinsics)]
#![no_std]

//! A visualization of Merge Sort, written using #![no_core].
//! Built on cargo 1.84.0-nightly (031049782 2024-11-01).

// In the wise words of Manish Goregaokar:
// Look at me.
// Look at me.
// I'm the libcore now.
#![no_core]

use c::TimeSpec;
use lang_items::*;
use utilities::Array;


mod intrinsics;
mod lang_items;
mod c;
mod traits;
mod utilities;

#[macro_export]
macro_rules! printf {
    ($msg: expr$(, $($args: expr),+)?) => {
        unsafe { $crate::c::printf($msg.as_ptr() $(, $($args),+)?) }
    };
}

#[macro_export]
macro_rules! bail {
    ($reason: literal$(, $($args: expr),+)?) => {{
        $crate::printf!($reason$(, $($args),+)?);
        unsafe { $crate::c::abort() }
    }};
}


// too lazy to implement Index
macro_rules! get {
    ($expr: expr => $index: expr) => {{
        let idx = ( $index );
        let Option::Some(e) = ($expr).at(idx) else { bail!(c"in merge: out-of-bounds index %d\n", idx) };
        e
    }};
}

const INF: f64 = 1.0 / 0.0;

fn main() {}

#[lang = "start"]
fn start<T>(main: fn() -> T, argc: isize, argv: *const *const u8, sigpipe: u8) -> isize {
    if argc < 3 {
        printf!(c"Usage: ./no_core_merge_sort <element count: int> <delay: double>\n");
        return -1;
    }
    let ptr_layout = Layout::of::<*const u8>();
    // Randomness, the C way!
    unsafe { c::srand(c::time(0 as *const i64) as u32) };

    let (element_count, swap_delay);
    unsafe { 
        let count = *((argv as usize + ptr_layout.size) as *const *const u8);
        let delay = *((argv as usize + ptr_layout.size * 2) as *const *const u8);

        element_count = c::atoi(count);
        swap_delay = c::atof(delay);
    }

    if element_count < 2 || element_count > 32767 {
        printf!(c"Element count must be within [2, 32767] (got %d)\n", element_count);
        return 1;
    }

    let element_count = element_count as usize;

    if !(swap_delay > 0.0) || !(swap_delay < 0x7FFF_FFFF as f64) {
        printf!(c"Swap delay must be within (0, 2147483647] (got %f)\n", swap_delay);
        return 1;
    }

    // Converts the swap delay to a TimeSpec
    let seconds = unsafe { intrinsics::float_to_int_unchecked::<f64, i64>(swap_delay) };
    let nanos = ((swap_delay - seconds as f64) * 1_000_000_000.0) as i64;
    let timespec = TimeSpec { seconds, nanos };

    // Set up the array
    let mut arr = Array::new(0, element_count);
    let mut index = 0;
    while index < element_count {
        index = index + 1;
        arr.set(index, index);
    }

    // Randomize the array
    index = 0;
    while index < element_count {
        display_arr(&arr, c"Randomizing...\n", Option::Some(index), &timespec);
        let rand_idx = unsafe { c::rand() as usize } % element_count;
        arr.swap(index, rand_idx);
        display_arr(&arr, c"Randomizing...\n", Option::Some(rand_idx), &timespec);
        index = index + 1;
    }

    display_arr(&arr, c"Waiting...\n", Option::None, &TimeSpec { seconds: 1, nanos: 0 });

    merge_sort(&mut arr, 0, element_count - 1, &timespec);

    display_arr(&arr, c"Done\n", Option::None, &TimeSpec { seconds: 0, nanos: 0 });

    return 0;
}

fn display_arr(array: &Array<usize>, message: &CStr, highlight: Option<usize>, time: &TimeSpec) {
    // Clear the screen with an ANSI escape sequence
    printf!(c"\x1b[2J\x1b[0;0H");
    printf!(message);
    let mut y = array.length() / 2 + 1;
    while y > 0 {
        y = y - 1;
        let mut x = 0;
        while x < array.length() {
            let value = get!(array => x);
            unsafe { c::putchar((
                if value >= (y * 2 + 1) { b'|' } 
                else if value >= y * 2 { b',' }
                else { b' ' }
            ) as i32) };
            x = x + 1;
        }
        unsafe { c::putchar(b'\n' as i32); }
    }
    // Add the highlight cursor
    if let Option::Some(hl) = highlight {
        let mut x = 0;
        while x < hl {
            unsafe { c::putchar(b' ' as i32); }
            x = x + 1;
        }
        printf!(c"^\n");
    }

    unsafe {
        c::nanosleep(time, 0 as *mut TimeSpec);
    }
}

// Implementation referenced from https://www.geeksforgeeks.org/in-place-merge-sort/
fn merge_sort(array: &mut Array<usize>, left: usize, right: usize, time: &TimeSpec) {
    if !(left < right) { return }
    let middle = left + (right - left) / 2;

    merge_sort(array, left, middle, time);
    merge_sort(array, middle + 1, right, time);

    merge(array, left, middle, right, time);
}

fn merge(array: &mut Array<usize>, mut left: usize, mut middle: usize, right: usize, time: &TimeSpec) {
    let mut mid_p1 = middle + 1;
    if get!(array => middle) <= get!(array => mid_p1) { return; }

    while left <= middle && mid_p1 <= right {
        if get!(array => left) <= get!(array => mid_p1) { left = left + 1; continue; }
        
        let mut index = mid_p1;

        while index > left {
            display_arr(array, c"Sorting...\n", Option::Some(index - 1), time);
            array.swap(index, index - 1);
            display_arr(array, c"Sorting...\n", Option::Some(index), time);
            index = index - 1;
        }

        left = left + 1;
        middle = middle + 1;
        mid_p1 = mid_p1 + 1;
    }
}