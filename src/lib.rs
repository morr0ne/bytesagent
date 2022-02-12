#![no_std] // Crate aims to me no std friendly
#![warn(missing_docs)] // I'd like for everything to be documented properly
#![allow(clippy::size_of_in_element_count)] // Clippy miss-detects this in the macros

//! A small crate to cast to and from arbitrary bytes

#[cfg(feature = "glam")]
use glam::{
    DMat2, DMat3, DMat4, DQuat, DVec2, DVec3, DVec4, IVec2, IVec3, IVec4, Mat2, Mat3, Mat4, Quat,
    UVec2, UVec3, UVec4, Vec2, Vec3, Vec4,
};
#[cfg(feature = "half")]
use half::{bf16, f16};

/// Cast to bytes
pub trait AsBytes {
    /// Re
    fn as_bytes(&self) -> &[u8];
}

/// Cast to bytes mut
pub trait AsBytesMut {
    /// Re mut
    fn as_bytes_mut(&mut self) -> &mut [u8];
}

macro_rules! impl_types {
    ($($ty:ty)+) => {
        $(
        // Plain types can be simply casted to the correct memory rappresentation.
        impl AsBytes for $ty {
            fn as_bytes(&self) -> &[u8] {
                unsafe { &*(self as *const $ty as *const [u8; core::mem::size_of::<$ty>()]) }
            }
        }
        impl AsBytesMut for $ty {
            fn as_bytes_mut(&mut self) -> &mut [u8] {
                unsafe { &mut*(self as *mut $ty as *mut [u8; core::mem::size_of::<$ty>()]) }
            }
        }
        // Unfotunatly untill const generics get stabilzed we can not do the above with arrays.
        // Instead we have the create a slice from the raw parts which *in theory* involves a memcpy.
        // This issue could be solved by implementing the method for every single (reasonable) array size like many crates do but this is fine for now.
        impl<const N: usize> AsBytes for [$ty; N] {
            fn as_bytes(&self) -> &[u8] {
                unsafe {
                    core::slice::from_raw_parts(
                        self.as_ptr().cast(),
                        self.len() * core::mem::size_of::<$ty>(),
                    )
                }
            }
        }
        impl<const N: usize> AsBytesMut for [$ty; N] {
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

// For u8 we don't need to do anything so it makes much more sense to manually implement it
impl AsBytes for u8 {
    fn as_bytes(&self) -> &[u8] {
        core::slice::from_ref(self)
    }
}
impl AsBytesMut for u8 {
    fn as_bytes_mut(&mut self) -> &mut [u8] {
        core::slice::from_mut(self)
    }
}
impl<const N: usize> AsBytes for [u8; N] {
    fn as_bytes(&self) -> &[u8] {
        self
    }
}
impl<const N: usize> AsBytesMut for [u8; N] {
    fn as_bytes_mut(&mut self) -> &mut [u8] {
        self
    }
}

// Implement for all the primitive types which are "plain old data types" so are just repprasented as a series of bytes in memory.
impl_types!(u16 u32 u64 u128 usize i8 i16 i32 i64 i128 isize f32 f64);
#[cfg(feature = "half")]
impl_types!(f16 bf16);
// This types from glam are also represented in memory by a series of bytes so this is also fine.
#[cfg(feature = "glam")]
impl_types!(DMat2 DMat3 DMat4 DQuat DVec2 DVec3 DVec4 IVec2 IVec3 IVec4 Mat2 Mat3 Mat4 Quat UVec2 UVec3 UVec4 Vec2 Vec3 Vec4);

// TODO: Implement AsBytes/AsBytesMut for other libraries and/or make it possible for other libraries to implement the trait
