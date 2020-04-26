//! NT Status codes.
#![allow(non_camel_case_types)]
#![allow(overflowing_literals)]

use core::ops::Try;

#[repr(C)]
#[derive(Clone, Copy)]
pub struct NTSTATUS(pub i32);

pub const STATUS_SUCCESS: NTSTATUS = NTSTATUS(0);

impl Try for NTSTATUS {
    type Ok = i32;
    type Error = i32;
    fn into_result(self) -> Result<Self::Ok, Self::Error> {
        match self.0 {
            i if i >= 0 => Ok(i),
            i => Err(i)
        }
    }
    fn from_error(v: Self::Error) -> Self {
        NTSTATUS(v)
    }
    fn from_ok(v: Self::Ok) -> Self {
        NTSTATUS(v)
    }
}

impl Default for NTSTATUS {
    fn default() -> Self {
        NTSTATUS(0)
    }
}
