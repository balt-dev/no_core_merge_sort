//! Contains lang items and traits for simple operations.
use crate::{intrinsics, lang_items::*};

#[lang = "not"]
pub trait Not {
    fn not(self) -> Self;
}

#[lang = "neg"]
pub trait Neg {
    fn neg(self) -> Self;
}

#[lang = "eq"]
pub trait PartialEq<Rhs = Self> {
    fn eq(&self, other: &Rhs) -> bool;
    fn ne(&self, other: &Rhs) -> bool { !self.eq(other) }
}

#[lang = "add"]
pub trait Add<Rhs = Self> {
    type Output;

    fn add(&self, other: &Rhs) -> Self::Output;
}

#[lang = "sub"]
pub trait Sub<Rhs = Self> {
    type Output;

    fn sub(&self, other: &Rhs) -> Self::Output;
}

#[lang = "mul"]
pub trait Mul<Rhs = Self> {
    type Output;

    fn mul(&self, other: &Rhs) -> Self::Output;
}

#[lang = "div"]
pub trait Div<Rhs = Self> {
    type Output;

    fn div(&self, other: &Rhs) -> Self::Output;
}

#[lang = "rem"]
pub trait Rem<Rhs = Self> {
    type Output;

    fn rem(&self, other: &Rhs) -> Self::Output;
}

#[lang = "shl"]
pub trait Shl<Rhs = Self> {
    type Output;

    fn shl(&self, amount: &Rhs) -> Self::Output;
}

#[lang = "shr"]
pub trait Shr<Rhs = Self> {
    type Output;

    fn shr(&self, amount: &Rhs) -> Self::Output;
}

#[lang = "bitand"]
pub trait BitAnd<Rhs = Self> {
    type Output;

    fn bitand(&self, amount: &Rhs) -> Self::Output;
}

#[lang = "deref"]
pub trait Deref {
    #[lang = "deref_target"]
    type Target: ?Sized;

    fn deref(&self) -> &Self::Target;
}

#[lang = "deref_mut"]
pub trait DerefMut: Deref {
    fn deref_mut(&mut self) -> &mut Self::Target;
}

#[lang = "deref_pure"]
pub unsafe trait DerefPure {}

unsafe impl<T: ?Sized> DerefPure for &T {}

unsafe impl<T: ?Sized> DerefPure for &mut T {}

macro_rules! trivial_impl {
    ($(impl $trait: ty, for $($ty: ty $(where <$($gen: ident),+>)?),* => $impl: tt)*) => {$(
        $(
            impl $(<$($gen),+>)? $trait for $ty $impl
        )*
    )*};
    ($(unsafe impl $trait: ty, for $($ty: ty $(where <$($gen: ident),+>)?),* => $impl: tt)*) => {$(
        $(
            unsafe impl $(<$($gen),+>)? $trait for $ty $impl
        )*
    )*};
}

trivial_impl! {
    unsafe impl Copy, for 
        bool, u8, u16, u32, u64, u128, usize, 
        i8, i16, i32, i64, i128, isize, 
        f32, f64, &T where <T>, *const T where <T>, *mut T where <T> 
        => {}
}
trivial_impl! {
    impl Not, for bool => {
        fn not(self) -> Self { !self }
    }
    impl Not, for u8, u16, u32, u64, u128, usize, i8, i16, i32, i64, i128, isize => {
        fn not(self) -> Self { !self }
    }
    impl Neg, for u8, u16, u32, u64, u128, usize, i8, i16, i32, i64, i128, isize => {
        fn neg(self) -> Self { -self }
    }
    impl PartialEq, for bool, u8, u16, u32, u64, u128, usize, i8, i16, i32, i64, i128, isize, f32, f64 => {
        fn eq(&self, other: &Self) -> bool { *self == *other }
    }
    impl Add, for u8, u16, u32, u64, u128, usize, i8, i16, i32, i64, i128, isize, f32, f64 => {
        type Output = Self;
        fn add(&self, other: &Self) -> Self { *self + *other }
    }
    impl Sub, for u8, u16, u32, u64, u128, usize, i8, i16, i32, i64, i128, isize, f32, f64 => {
        type Output = Self;
        fn sub(&self, other: &Self) -> Self { *self - *other }
    }
    impl Mul, for u8, u16, u32, u64, u128, usize, i8, i16, i32, i64, i128, isize, f32, f64 => {
        type Output = Self;
        fn mul(&self, other: &Self) -> Self { *self * *other }
    }
    impl Div, for u8, u16, u32, u64, u128, usize, i8, i16, i32, i64, i128, isize, f32, f64 => {
        type Output = Self;
        fn div(&self, other: &Self) -> Self { *self / *other }
    }
    impl Rem, for u8, u16, u32, u64, u128, usize, i8, i16, i32, i64, i128, isize, f32, f64 => {
        type Output = Self;
        fn rem(&self, other: &Self) -> Self { *self % *other }
    }
    impl BitAnd, for u8, u16, u32, u64, u128, usize, i8, i16, i32, i64, i128, isize => {
        type Output = Self;
        fn bitand(&self, other: &Self) -> Self { *self & *other }
    }
    impl Shl, for u8, u16, u32, u64, u128, usize, i8, i16, i32, i64, i128, isize => {
        type Output = Self;
        fn shl(&self, amount: &Self) -> Self { *self << *amount }
    }
    impl Shr, for u8, u16, u32, u64, u128, usize, i8, i16, i32, i64, i128, isize => {
        type Output = Self;
        fn shr(&self, amount: &Self) -> Self { *self >> *amount }
    }
    impl Ord, for u8, u16, u32, u64, u128, usize, i8, i16, i32, i64, i128, isize => {
        fn cmp(&self, other: &Self) -> Ordering { intrinsics::three_way_compare(*self, *other) }
    }
}

impl<T: ?Sized> Deref for &T {
    type Target = T;
    fn deref(&self) -> &T { *self }
}
impl<T: ?Sized> Deref for &mut T {
    type Target = T;
    fn deref(&self) -> &T { *self }
}
impl<T: ?Sized> DerefMut for &mut T {
    fn deref_mut(&mut self) -> &mut T { *self }
}

#[lang = "partial_ord"]
pub trait PartialOrd<Rhs = Self> {
    fn partial_cmp(&self, other: &Rhs) -> Option<Ordering>;

    fn lt(&self, other: &Rhs) -> bool { crate::matches!(self.partial_cmp(other) => Option::Some(Ordering::Less)) }
    fn le(&self, other: &Rhs) -> bool {
        let cmp = self.partial_cmp(other);
        !cmp.is_none() && !crate::matches!(cmp => Option::Some(Ordering::Greater))
    }

    fn gt(&self, other: &Rhs) -> bool { crate::matches!(self.partial_cmp(other) => Option::Some(Ordering::Greater)) }
    fn ge(&self, other: &Rhs) -> bool {
        let cmp = self.partial_cmp(other);
        !cmp.is_none() && !crate::matches!(cmp => Option::Some(Ordering::Less))
    }
}

pub trait Ord<Rhs = Self> {
    fn cmp(&self, other: &Rhs) -> Ordering;
}

// seriously, why isn't this normally a thing
impl<Lhs, Rhs> PartialOrd<Rhs> for Lhs where Lhs: Ord<Rhs> {
    fn partial_cmp(&self, other: &Rhs) -> Option<Ordering> {
        Option::Some(self.cmp(other))
    }
}

impl PartialOrd for f64 {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        // Make sure neither are NaN
        if *self != *self || *other != *other { return Option::None }
        let lhs = unsafe { intrinsics::transmute::<f64, i64>(*self) };
        let rhs = unsafe { intrinsics::transmute::<f64, i64>(*other) };
        // 0 and -0
        if (lhs == 0 && rhs == -1) || (lhs == -1 && rhs == 0) { return Option::Some(Ordering::Equal) }
        // Past this point, we can just compare the integers
        lhs.partial_cmp(&rhs)
    }
}
