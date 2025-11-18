//! `typenum` like boolean flags. Includes an [`Unknown`] type to cover cases

use crate::traits::maybe_bool::sealed::Sealed;
use core::ops::{BitAnd, BitOr, Not};

pub trait MaybeBool: Sealed + Sized {
    const MAYBE_VALUE: Option<bool>;
    const IS_TRUE: bool = matches!(Self::MAYBE_VALUE, Some(true));
    const IS_FALSE: bool = matches!(Self::MAYBE_VALUE, Some(false));
    const IS_UNKNOWN: bool = Self::MAYBE_VALUE.is_none();
}
pub trait Bool: MaybeBool {
    const VALUE: bool = Self::MAYBE_VALUE.unwrap();
}

mod sealed {
    pub trait Sealed {}
}

/// With [`MaybeBool`] inputs will always output a [`Bool`]. [`True`] as an input
/// will always output [`True`]. Two [`Unknown`] inputs will output [`False`].
pub type Or<T, U> = <T as BitOr<U>>::Output;
/// With [`MaybeBool`] inputs will always output a [`Bool`]. [`Unknown`] or [`False`] as an input
/// will always output [`False`].
pub type And<T, U> = <T as BitAnd<U>>::Output;
/// [`False`] will always output [`True`]. [`True`] will always output [`False`]. [`Unknown`] will
/// always output [`Unknown`].
pub type NotValue<T> = <T as Not>::Output;

pub struct True;
impl sealed::Sealed for True {}
impl MaybeBool for True {
    const MAYBE_VALUE: Option<bool> = Some(true);
}
impl Bool for True {
    const VALUE: bool = true;
}
impl<T: MaybeBool> BitOr<T> for True {
    type Output = True;

    fn bitor(self, _rhs: T) -> Self::Output {
        self
    }
}
impl<T> BitAnd<T> for True
where
    T: Bool,
{
    type Output = T;

    fn bitand(self, rhs: T) -> Self::Output {
        rhs
    }
}
impl BitAnd<Unknown> for True {
    type Output = False;

    fn bitand(self, _rhs: Unknown) -> Self::Output {
        False
    }
}
impl Not for True {
    type Output = False;

    fn not(self) -> Self::Output {
        False
    }
}

pub struct False;
impl sealed::Sealed for False {}
impl MaybeBool for False {
    const MAYBE_VALUE: Option<bool> = Some(false);
}
impl Bool for False {
    const VALUE: bool = false;
}
impl<T: MaybeBool> BitOr<T> for False {
    type Output = T;

    fn bitor(self, rhs: T) -> Self::Output {
        rhs
    }
}
impl<T> BitAnd<T> for False
where
    T: Bool,
{
    type Output = False;

    fn bitand(self, _rhs: T) -> Self::Output {
        self
    }
}
impl Not for False {
    type Output = True;

    fn not(self) -> Self::Output {
        True
    }
}

pub struct Unknown;
impl sealed::Sealed for Unknown {}
impl MaybeBool for Unknown {
    const MAYBE_VALUE: Option<bool> = None;
}
impl<T> BitOr<T> for Unknown
where
    T: Bool,
{
    type Output = T;

    fn bitor(self, rhs: T) -> Self::Output {
        rhs
    }
}
impl BitOr<Unknown> for Unknown {
    type Output = False;

    fn bitor(self, _rhs: Unknown) -> Self::Output {
        False
    }
}
impl<T> BitAnd<T> for Unknown
where
    T: MaybeBool,
{
    type Output = False;

    fn bitand(self, _rhs: T) -> Self::Output {
        False
    }
}
impl Not for Unknown {
    type Output = Unknown;

    fn not(self) -> Self::Output {
        self
    }
}
