#![allow(clippy::size_of_in_element_count)] // Clippy miss-detects this in the macros
#![warn(clippy::pedantic)]
#![cfg_attr(not(feature = "std"), no_std)]

//! A small crate to cast to and from arbitrary bytes

#[cfg(feature = "glam")]
use glam::{
    DMat2, DMat3, DMat4, DQuat, DVec2, DVec3, DVec4, IVec2, IVec3, IVec4, Mat2, Mat3, Mat4, Quat,
    UVec2, UVec3, UVec4, Vec2, Vec3, Vec4,
};
#[cfg(feature = "half")]
use half::{bf16, f16};

#[derive(Debug)]
pub enum Error {
    Size,
}

#[cfg(feature = "std")]
impl std::error::Error for Error {}
impl core::fmt::Display for Error {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        match self {
            Error::Size => write!(f, ""),
        }
    }
}

pub type Result<T, E = Error> = core::result::Result<T, E>;

/// # Safety
/// Implementors must ensure that the type is nothing more than a sequence of bytes
pub unsafe trait Pod {
    fn as_bytes(&self) -> &[u8];
    fn as_bytes_mut(&mut self) -> &mut [u8];
    /// # Errors
    /// TODO
    fn from_bytes(bytes: &[u8]) -> Result<&Self>
    where
        Self: Sized,
    {
        if bytes.len() == core::mem::size_of::<Self>() {
            Ok(unsafe { Self::from_bytes_unchecked(bytes) })
        } else {
            Err(Error::Size)
        }
    }
    /// # Safety
    /// TODO
    unsafe fn from_bytes_unchecked(bytes: &[u8]) -> &Self
    where
        Self: Sized,
    {
        &*bytes.as_ptr().cast::<Self>()
    }
    /// # Errors
    /// TODO
    fn from_bytes_mut(bytes: &mut [u8]) -> Result<&mut Self>
    where
        Self: Sized,
    {
        if bytes.len() == core::mem::size_of::<Self>() {
            Ok(unsafe { Self::from_bytes_mut_unchecked(bytes) })
        } else {
            Err(Error::Size)
        }
    }

    /// # Safety
    /// TODO
    unsafe fn from_bytes_mut_unchecked(bytes: &mut [u8]) -> &mut Self
    where
        Self: Sized,
    {
        &mut *bytes.as_mut_ptr().cast::<Self>()
    }
}

macro_rules! impl_pod {
    ($($ty:ty)+) => {
        $(
        // Plain types can be simply asked to the correct memory representation.
        unsafe impl Pod for $ty {
            fn as_bytes(&self) -> &[u8] {
                unsafe { &*(self as *const $ty).cast::<[u8; 4]>() }
            }
            fn as_bytes_mut(&mut self) -> &mut [u8] {
                unsafe { &mut*(self as *mut $ty).cast::<[u8; 4]>() }
            }
        }
        // Unfortunately until const generics get stabilized we cannot do the above with arrays.
        // This however is a non issue: the final asm (at least in release mode) is the same.
        unsafe impl<const N: usize> Pod for [$ty; N] {
            fn as_bytes(&self) -> &[u8] {
                unsafe {
                    core::slice::from_raw_parts(
                        self.as_ptr().cast(),
                        self.len() * core::mem::size_of::<$ty>(),
                    )
                }
            }
            fn as_bytes_mut(&mut self) -> &mut [u8] {
                unsafe {
                    core::slice::from_raw_parts_mut(
                        self.as_mut_ptr().cast(),
                        self.len() * core::mem::size_of::<$ty>(),
                    )
                }
            }
        }
        )*
    }
}

// Implement for all the primitive types which are "plain old data types" so are just repprasented as a series of bytes in memory.
impl_pod!(u8 u16 u32 u64 u128 usize i8 i16 i32 i64 i128 isize  f32 f64);
#[cfg(feature = "half")]
impl_pod!(f16 bf16);
// This types from glam are also represented in memory by a series of bytes so this is also fine.
#[cfg(feature = "glam")]
impl_pod!(DMat2 DMat3 DMat4 DQuat DVec2 DVec3 DVec4 IVec2 IVec3 IVec4 Mat2 Mat3 Mat4 Quat UVec2 UVec3 UVec4 Vec2 Vec3 Vec4);
