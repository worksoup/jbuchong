use crate::{utils::raw_pointer_to_instance, RawPointer};
use j4rs::{errors::J4RsError, Instance, InvocationArg, Jvm};
use jbc_base::{self as jbuchong, GetInstanceTrait, TryFromInstanceTrait};
use jbc_derive::GetInstanceDerive;
use std::marker::PhantomData;

#[derive(GetInstanceDerive)]
pub struct Supplier<R> {
    instance: Instance,
    origin_func_raw: RawPointer,
    internal_closure_raw: RawPointer,
    _r: PhantomData<R>,
}
impl<R> Supplier<R> {
    unsafe fn get_internal_closure_raw(&self) -> *mut dyn Fn() -> Result<Instance, J4RsError> {
        unsafe { std::mem::transmute(self.internal_closure_raw) }
    }
    unsafe fn get_origin_func_raw(&self) -> *mut dyn Fn() -> R {
        unsafe { std::mem::transmute(self.origin_func_raw) }
    }
    pub fn drop(self) {
        let _ = unsafe { Box::from_raw(self.get_internal_closure_raw()) };
        let _ = unsafe { Box::from_raw(self.get_origin_func_raw()) };
    }
    pub fn call(&self) -> R {
        let func = unsafe { self.get_origin_func_raw() };
        unsafe { (*func)() }
    }
}
impl<R> Supplier<R>
where
    R: TryFromInstanceTrait,
{
    pub fn get(&self) -> Result<R, J4RsError> {
        let jvm = Jvm::attach_thread()?;
        let result = jvm.invoke(&self.get_instance()?, "get", InvocationArg::empty())?;
        R::try_from_instance(result)
    }
}
impl<R> Supplier<R>
where
    R: GetInstanceTrait,
{
    #[inline]
    fn internal_closure_as_raw_pointer(f: *mut dyn Fn() -> R) -> RawPointer {
        let call_from_java: Box<dyn Fn() -> Result<Instance, J4RsError>> =
            Box::new(move || -> Result<Instance, J4RsError> { unsafe { (*f)().get_instance() } });
        let call_from_java_raw = Box::into_raw(call_from_java);
        unsafe { std::mem::transmute(call_from_java_raw) }
    }
    pub fn new<F>(closure: F) -> Supplier<R>
    where
        F: Fn() -> R + 'static,
    {
        let origin_func: *mut dyn Fn() -> R = Box::into_raw(Box::new(closure));
        let origin_func_raw: RawPointer = unsafe { std::mem::transmute(origin_func) };
        let internal_closure_raw = Self::internal_closure_as_raw_pointer(origin_func);
        let instance = raw_pointer_to_instance(
            "io.github.worksoup.function.JBuChongSupplier",
            internal_closure_raw,
        );
        Supplier {
            instance,
            origin_func_raw,
            internal_closure_raw,
            _r: PhantomData,
        }
    }
}
