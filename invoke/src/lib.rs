use std::{any::Any, ffi::c_void, ptr::null};

pub use invoke_derive::invoke;

#[derive(Debug)]
pub enum InvokeError {
    NoneArgs,
    BadArgs,
    UnknownMethod,
}

pub trait Invoke {
    /// # Safety
    ///
    /// TODO
    unsafe fn invoke_raw_ptr(&self, fn_id: u16, args: *const c_void) -> Result<(), InvokeError>;

    /// # Safety
    ///
    /// TODO
    unsafe fn invoke_raw_ptr_mut(
        &mut self,
        fn_id: u16,
        args: *const c_void,
    ) -> Result<(), InvokeError>;

    fn invoke(&self, fn_id: u16, args: Option<&dyn Any>) -> Result<(), InvokeError>;
    fn invoke_mut(&mut self, fn_id: u16, args: Option<&dyn Any>) -> Result<(), InvokeError>;
}

pub trait InvokeExt {
    /// # Safety
    ///
    /// TODO
    unsafe fn invoke_raw<Args>(&self, fn_id: u16, args: Option<&Args>) -> Result<(), InvokeError>;

    /// # Safety
    ///
    /// TODO
    unsafe fn invoke_raw_mut<Args>(
        &mut self,
        fn_id: u16,
        args: Option<&Args>,
    ) -> Result<(), InvokeError>;
}

impl<T: Invoke + ?Sized> InvokeExt for T {
    unsafe fn invoke_raw<Args>(&self, fn_id: u16, args: Option<&Args>) -> Result<(), InvokeError> {
        match args {
            Some(v) => {
                let ptr: *const c_void = std::mem::transmute(v);
                self.invoke_raw_ptr(fn_id, ptr)
            }
            None => self.invoke_raw_ptr(fn_id, null()),
        }
    }

    unsafe fn invoke_raw_mut<Args>(
        &mut self,
        fn_id: u16,
        args: Option<&Args>,
    ) -> Result<(), InvokeError> {
        match args {
            Some(v) => {
                let ptr: *const c_void = std::mem::transmute(v);
                self.invoke_raw_ptr_mut(fn_id, ptr)
            }
            None => self.invoke_raw_ptr_mut(fn_id, null()),
        }
    }
}
