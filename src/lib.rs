#![no_std] // Crate aims to me no std friendly
#![allow(clippy::size_of_in_element_count)] // Clippy miss-detects this in the macros
#![warn(clippy::pedantic)]

//! A small crate to cast to and from arbitrary bytes

#[cfg(feature = "glam")]
use glam::{
    DMat2, DMat3, DMat4, DQuat, DVec2, DVec3, DVec4, IVec2, IVec3, IVec4, Mat2, Mat3, Mat4, Quat,
    UVec2, UVec3, UVec4, Vec2, Vec3, Vec4,
};
#[cfg(feature = "half")]
use half::{bf16, f16};

pub unsafe trait Pod {
    fn as_bytes(&self) -> &[u8];
    fn as_bytes_mut(&mut self) -> &mut [u8];
}

macro_rules! impl_pod {
    ($($ty:ty)+) => {
        $(
        // Plain types can be simply asked to the correct memory representation.
        unsafe impl Pod for $ty {
            fn as_bytes(&self) -> &[u8] {
                unsafe { &*(self as *const $ty as *const [u8; core::mem::size_of::<$ty>()]) }
            }
            fn as_bytes_mut(&mut self) -> &mut [u8] {
                unsafe { &mut*(self as *mut $ty as *mut [u8; core::mem::size_of::<$ty>()]) }
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
