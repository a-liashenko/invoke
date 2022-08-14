use crate::FnId;
use std::ffi::c_void;

pub trait InvokeMeta {
    fn get_method_id_raw_ptr(func_ptr: *const c_void) -> Option<FnId>;
    fn get_method_id(name: &str) -> Option<FnId>;
}

pub trait InvokeMetaExt {
    fn get_method_id_raw<Fn>(ptr: &Fn) -> Option<FnId>;
}

impl<T: InvokeMeta + ?Sized> InvokeMetaExt for T {
    fn get_method_id_raw<Fn>(ptr: &Fn) -> Option<u16> {
        T::get_method_id_raw_ptr(ptr as *const _ as *const c_void)
    }
}
