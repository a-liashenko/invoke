use crate::{FnId, InvokeError};
use std::{any::Any, ffi::c_void, ptr::null};

pub trait Invoke {
    fn invoke(&self, fn_id: FnId, args: Option<&dyn Any>) -> Result<(), InvokeError>;

    /// .
    ///
    /// # Errors
    ///
    /// This function will return an error if .
    ///
    /// # Safety
    ///
    /// .
    unsafe fn invoke_ptr(&self, fn_id: FnId, args: *const c_void) -> Result<(), InvokeError>;
}

pub trait InvokeMut {
    fn invoke_mut(&mut self, fn_id: FnId, args: Option<&dyn Any>) -> Result<(), InvokeError>;

    /// .
    ///
    /// # Errors
    ///
    /// This function will return an error if .
    ///
    /// # Safety
    ///
    /// .
    unsafe fn invoke_mut_ptr(
        &mut self,
        fn_id: FnId,
        args: *const c_void,
    ) -> Result<(), InvokeError>;
}

pub trait InvokeExt {
    /// # Safety
    ///
    /// TODO
    unsafe fn invoke_raw<Args>(&self, fn_id: u16, args: Option<&Args>) -> Result<(), InvokeError>;
}

impl<T: Invoke + ?Sized> InvokeExt for T {
    unsafe fn invoke_raw<Args>(&self, fn_id: u16, args: Option<&Args>) -> Result<(), InvokeError> {
        match args {
            Some(v) => {
                let ptr: *const c_void = v as *const _ as *const _;
                self.invoke_ptr(fn_id, ptr)
            }
            None => self.invoke_ptr(fn_id, null()),
        }
    }
}

pub trait InvokeMutExt {
    /// # Safety
    ///
    /// TODO
    unsafe fn invoke_mut_raw<Args>(
        &mut self,
        fn_id: u16,
        args: Option<&Args>,
    ) -> Result<(), InvokeError>;
}

impl<T: InvokeMut + ?Sized> InvokeMutExt for T {
    unsafe fn invoke_mut_raw<Args>(
        &mut self,
        fn_id: u16,
        args: Option<&Args>,
    ) -> Result<(), InvokeError> {
        match args {
            Some(v) => {
                let ptr: *const c_void = v as *const _ as *const _;
                self.invoke_mut_ptr(fn_id, ptr)
            }
            None => self.invoke_mut_ptr(fn_id, null()),
        }
    }
}
