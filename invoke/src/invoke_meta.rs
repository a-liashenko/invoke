use std::ffi::c_void;

pub trait InvokeMeta {
    /// # Safety
    ///
    /// TODO
    fn get_method_id_raw_ptr(func_ptr: *const c_void) -> Option<u16>;
    fn get_method_id(name: &str) -> Option<u16>;
}

pub trait InvokeMetaExt {
    /// # Safety
    ///
    /// TODO
    fn get_method_id_raw<Fn>(ptr: &Fn) -> Option<u16>;
}

impl<T: InvokeMeta + ?Sized> InvokeMetaExt for T {
    fn get_method_id_raw<Fn>(ptr: &Fn) -> Option<u16> {
        T::get_method_id_raw_ptr(ptr as *const _ as *const c_void)
    }
}
