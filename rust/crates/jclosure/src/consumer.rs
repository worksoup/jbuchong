use crate::utils::raw_pointer_to_instance;
use crate::RawPointer;
use j4rs::{errors::J4RsError, Instance, InvocationArg, Jvm};
use jbc_base::{self as jbuchong, GetInstanceTrait, InstanceWrapper, TryFromInstanceTrait};
use jbc_derive::GetInstanceDerive;
use std::marker::PhantomData;

#[derive(GetInstanceDerive)]
pub struct Consumer<T> {
    instance: Instance,
    origin_func_raw: RawPointer,
    internal_closure_raw: RawPointer,
    _t: PhantomData<T>,
}

impl<T> Consumer<T> {
    unsafe fn get_internal_closure_raw(
        &self,
    ) -> *mut dyn Fn(InstanceWrapper) -> Result<(), J4RsError> {
        unsafe { std::mem::transmute(self.internal_closure_raw) }
    }
    unsafe fn get_origin_func_raw(&self) -> *mut dyn Fn(T) {
        unsafe { std::mem::transmute(self.origin_func_raw) }
    }
    pub fn drop(self) {
        let _ = unsafe { Box::from_raw(self.get_internal_closure_raw()) };
        let _ = unsafe { Box::from_raw(self.get_origin_func_raw()) };
    }
    pub fn accept(&self, arg: InvocationArg) -> Result<(), J4RsError> {
        Jvm::attach_thread()?.invoke(&self.get_instance()?, "accept", &[arg])?;
        Ok(())
    }
    pub fn call(&self, arg: T) {
        unsafe { (*self.get_origin_func_raw())(arg) }
    }
}
impl<T> Consumer<T>
where
    T: TryFromInstanceTrait,
{
    #[inline]
    fn internal_closure_as_raw_pointer(f: *mut dyn Fn(T)) -> RawPointer {
        let call_from_java: Box<dyn Fn(InstanceWrapper) -> Result<(), J4RsError>> =
            Box::new(move |value: InstanceWrapper| -> Result<(), J4RsError> {
                unsafe {
                    (*f)(value.get::<T>()?);
                }
                Ok(())
            });
        let call_from_java_raw = Box::into_raw(call_from_java);
        unsafe { std::mem::transmute(call_from_java_raw) }
    }
    pub fn new<F>(closure: F) -> Consumer<T>
    where
        F: Fn(T) + 'static,
    {
        let origin_func: *mut dyn Fn(T) = Box::into_raw(Box::new(closure));
        let origin_func_raw: RawPointer = unsafe { std::mem::transmute(origin_func) };
        let internal_closure_raw = Self::internal_closure_as_raw_pointer(origin_func);
        println!("closure_to_function\n{:?}", internal_closure_raw);
        let instance = raw_pointer_to_instance(
            "io.github.worksoup.function.JBuChongConsumer",
            internal_closure_raw,
        );
        Consumer {
            instance,
            origin_func_raw,
            internal_closure_raw,
            _t: PhantomData,
        }
    }
}
