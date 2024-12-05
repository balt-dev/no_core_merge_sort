//! Contains lang item definitions.

use crate::{c, intrinsics, traits::Deref};

#[lang = "sized"]
pub unsafe trait Sized {}
#[lang = "copy"]
pub unsafe trait Copy {}

#[lang = "clone"]
pub trait Clone {
    fn clone(&self) -> Self;
}

impl<T: Copy> Clone for T {
    fn clone(&self) -> Self { *self }
}

#[lang = "legacy_receiver"]
pub trait LegacyReceiver {}

impl<T: ?Sized> LegacyReceiver for &T {}
impl<T: ?Sized> LegacyReceiver for &mut T {}

#[lang = "panic_location"]
pub struct Location<'a> {
    file: &'a str,
    line: u32,
    col: u32
}

#[lang = "drop"]
pub trait Drop {
    fn drop(&mut self);
}

#[lang = "drop_in_place"]
pub unsafe fn drop_in_place<T: ?Sized>(value: *mut T) { }

#[lang = "manually_drop"]
#[repr(transparent)]
pub struct ManuallyDrop<T: ?Sized> {
    value: T,
}

unsafe impl<T: Copy> Copy for ManuallyDrop<T> {}

#[lang = "Option"]
pub enum Option<T> {
    None,
    Some(T)
}

impl<T> Option<T> {
    pub fn is_none(&self) -> bool {
        crate::matches!(self => Self::None)
    }

    pub fn is_some(&self) -> bool {
        crate::matches!(self => Self::Some(_))
    }
}

#[lang = "Ordering"]
#[repr(i8)]
pub enum Ordering {
    Less = -1,
    Equal,
    Greater
}

#[lang = "alloc_layout"]
pub struct Layout {
    pub size: usize,
    pub align: u16
}

unsafe impl Copy for Layout {}

impl Layout {
    pub fn of<T>() -> Self {
        Self {
            size: intrinsics::size_of::<T>(),
            align: intrinsics::min_align_of::<T>() as u16
        }
    }

    /// Returns a copy of this layout, aligned to work with C.
    pub fn c_aligned(&self) -> Self {
        if self.size == 0 { return *self; }
        let supported_alignment = 1 << (15 - intrinsics::ctlz(self.align) as usize);
        // size = ceil(size / align) * align
        let mut supported_size = self.size / supported_alignment;
        if self.size % supported_alignment != 0 {
            supported_size = supported_size + 1;
        }
        supported_size = supported_size * supported_alignment;
        Self { size: supported_size, align: supported_alignment as u16 }
    }

    pub unsafe fn alloc(&self) -> *mut () {
        if self.size == 0 {
            // Return a dangling pointer
            return self.align as usize as *mut ();
        }
        let Self { size, align } = self.c_aligned();
        // Allocate the memory via C
        unsafe {
            //c::aligned_alloc(align as usize, size)
            // For some reason, aligned_alloc didn't work.
            c::malloc(size)
        }
    }

    pub unsafe fn dealloc(ptr: *mut ()) {
        c::free(ptr);
    }
}

#[macro_export]
macro_rules! matches {
    ($expr: expr => $pat: pat) => {
        match {$expr} { $pat => true, _ => false } 
    };
}

#[lang = "freeze"]
pub unsafe auto trait Freeze {}

#[lang = "CStr"]
// ah fuck it, why not
// these can only be constructed via cstr literals, so they
// can never not be a valid c string
pub struct CStr { inner: [u8] }

impl CStr {
    pub const fn as_ptr(&self) -> *const u8 { &self.inner as *const [u8] as *const u8 }
}

#[rustc_builtin_macro]
#[macro_export]
macro_rules! stringify {
    ($($t:tt)*) => {};
}

macro_rules! def_unreachables {
    ($($ident: ident => $literal: literal),*) => {
        $(
            #[lang = $crate::stringify!($ident)]
            #[track_caller]
            fn $ident() -> ! { unsafe { 
                $crate::c::printf($literal.as_ptr());
                $crate::c::abort()
            } }
        )*
    };
}

def_unreachables! {
    panic_const_add_overflow => c"panic! :( addition overflowed\n",
    panic_const_mul_overflow => c"panic! :( multiplication overflowed\n",
    panic_const_sub_overflow => c"panic! :( subtraction overflowed\n",
    panic_const_shl_overflow => c"panic! :( left-shift overflowed\n",
    panic_const_div_by_zero => c"panic! :( division by zero\n",
    panic_const_rem_by_zero => c"panic! :( modulus of zero\n"
}

#[lang = "panic_in_cleanup"]
fn panic_in_cleanup () -> ! { unsafe { 
    c::printf(c"panic! :( something bad happened during cleanup\n".as_ptr());
    c::abort()
} }

#[lang = "unpin"]
pub auto trait Unpin {}