use crate::FnId;
use std::ffi::c_void;

pub trait InvokeMeta {
    fn get_method_id_raw_ptr(func_ptr: *const c_void) -> Option<&'static FnId>;
    fn get_method_id(name: &str) -> Option<&'static FnId>;

    fn get_method_name(id: &FnId) -> Option<&'static str>;
}

pub trait InvokeMetaSelf {
    fn get_method_id_raw_ptr(&self, func_ptr: *const c_void) -> Option<&'static FnId>;
    fn get_method_id(&self, name: &str) -> Option<&'static FnId>;

    fn get_method_name(&self, id: &FnId) -> Option<&'static str>;
}

pub trait InvokeMetaExt {
    fn get_method_id_raw<Fn>(ptr: &Fn) -> Option<&'static FnId>;
}

impl<T: InvokeMeta + ?Sized> InvokeMetaExt for T {
    fn get_method_id_raw<Fn>(ptr: &Fn) -> Option<&'static FnId> {
        T::get_method_id_raw_ptr(ptr as *const _ as *const c_void)
    }
}
