use crate::lang_items::*;


#[rustc_intrinsic]
pub fn abort() -> ! { loop {} }
#[rustc_intrinsic]
pub fn size_of<T: ?Sized>() -> usize { loop {} }
#[rustc_intrinsic]
pub fn min_align_of<T: ?Sized>() -> usize { loop {} }
#[rustc_intrinsic]
pub fn three_way_compare<T>(lhs: T, rhs: T) -> Ordering { loop {} }
#[rustc_intrinsic]
pub unsafe fn transmute<T, U>(value: T) -> U { loop {} }
#[rustc_intrinsic]
pub fn ctlz<T: Copy>(value: T) -> u32 { loop {} }
#[rustc_intrinsic]
pub unsafe fn copy_nonoverlapping<T>(src: *const T, dst: *mut T, amount: usize) { }
#[rustc_intrinsic]
pub fn forget<T>(value: T) { }
#[rustc_intrinsic]
pub unsafe fn typed_swap<T>(x: *mut T, y: *mut T) { }
#[rustc_intrinsic]
pub unsafe fn float_to_int_unchecked<F: Copy, I: Copy>(value: F) -> I { loop {} }