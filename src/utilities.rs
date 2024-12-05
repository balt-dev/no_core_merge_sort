use crate::lang_items::*;
use crate::traits::*;
use crate::intrinsics;

/// A statically-sized array of copyable values.
pub struct Array<T: Copy> {
    // storing this is suboptimal but i couldn't get ctfe to work
    layout: Layout,
    inner: *mut T,
    len: usize,
}

impl<T: Copy> Array<T> {
    pub fn new(value: T, length: usize) -> Self {
        // Allocate a pointer and copy the value to it a bunch of times
        let mut layout = Layout::of::<T>().c_aligned();
        layout.size = layout.size * length; // c_aligned guarantees this works
        let ptr;
        unsafe {
            ptr = layout.alloc() as *mut T; // uses C11's aligned_alloc (we're linking to c stdlib)
            let mut index = 0;
            let mut offset_ptr = ptr;
            while index < length {
                intrinsics::copy_nonoverlapping(&value as *const T, offset_ptr, 1);
                offset_ptr = (offset_ptr as usize + layout.align as usize) as *mut T;
                index = index + 1;
            }
        }
        Self { layout, inner: ptr, len: length }
    }

    fn ptr_at(&self, index: usize) -> *mut T {
        (self.inner as usize + (self.layout.align as usize * index)) as *mut T
    }

    pub fn at(&self, index: usize) -> Option<T> {
        if index >= self.len { return Option::None; }
        Option::Some(unsafe { *self.ptr_at(index) })
    }

    pub fn set(&mut self, index: usize, value: T) {
        if index >= self.len { return; }
        unsafe { *self.ptr_at(index) = value; }
    }

    pub fn swap(&mut self, idx1: usize, idx2: usize) {
        if idx1 >= self.len { return; }
        if idx2 >= self.len { return; }
        
        let lhs = self.ptr_at(idx1);
        let rhs = self.ptr_at(idx2);
        unsafe {
            intrinsics::typed_swap(lhs, rhs)
        }
    }

    pub fn length(&self) -> usize { self.len }
}

impl<T: Copy> Drop for Array<T> {
    fn drop(&mut self) {
        unsafe { Layout::dealloc(self.inner as *mut ()) }
    }
}

