use crate::{utils::raw_pointer_to_instance, RawPointer};
use j4rs::{errors::J4RsError, Instance, InvocationArg, Jvm};
use jbc_base::{self as jbuchong, GetInstanceTrait, InstanceWrapper, TryFromInstanceTrait};
use jbc_derive::GetInstanceDerive;
use std::marker::PhantomData;

#[derive(GetInstanceDerive)]
pub struct Function<T, R> {
    instance: Instance,
    origin_func_raw: RawPointer,
    internal_closure_raw: RawPointer,
    _t: PhantomData<T>,
    _r: PhantomData<R>,
}
impl<T, R> Function<T, R> {
    unsafe fn get_internal_closure_raw(
        &self,
    ) -> *mut dyn Fn(InstanceWrapper) -> Result<Instance, J4RsError> {
        unsafe { std::mem::transmute(self.internal_closure_raw) }
    }
    unsafe fn get_origin_func_raw(&self) -> *mut dyn Fn(T) -> R {
        unsafe { std::mem::transmute(self.origin_func_raw) }
    }
    pub fn drop(self) {
        let _ = unsafe { Box::from_raw(self.get_internal_closure_raw()) };
        let _ = unsafe { Box::from_raw(self.get_origin_func_raw()) };
    }
    pub fn call(&self, arg: T) -> R {
        let func = unsafe { self.get_origin_func_raw() };
        unsafe { (*func)(arg) }
    }
}
impl<T, R> Function<T, R>
where
    R: TryFromInstanceTrait,
{
    pub fn apply(&self, arg: InvocationArg) -> Result<R, J4RsError> {
        let jvm = Jvm::attach_thread()?;
        let result = jvm.invoke(&self.get_instance()?, "apply", &[arg])?;
        R::try_from_instance(result)
    }
}
impl<T, R> Function<T, R>
where
    T: TryFromInstanceTrait,
    R: GetInstanceTrait,
{
    #[inline]
    fn internal_closure_as_raw_pointer(origin_func: *mut dyn Fn(T) -> R) -> RawPointer {
        let call_from_java: Box<dyn Fn(InstanceWrapper) -> Result<Instance, J4RsError>> = Box::new(
            move |value: InstanceWrapper| -> Result<Instance, J4RsError> {
                unsafe { (*origin_func)(value.get::<T>()?).get_instance() }
            },
        );
        let call_from_java_raw = Box::into_raw(call_from_java);
        unsafe { std::mem::transmute(call_from_java_raw) }
    }
    pub fn new<F>(closure: F) -> Function<T, R>
    where
        F: Fn(T) -> R + 'static,
    {
        let origin_func: *mut dyn Fn(T) -> R = Box::into_raw(Box::new(closure));
        let origin_func_raw: RawPointer = unsafe { std::mem::transmute(origin_func) };
        let internal_closure_raw = Self::internal_closure_as_raw_pointer(origin_func);
        println!("closure_to_function\n{:?}", internal_closure_raw);
        let instance = raw_pointer_to_instance(
            "io.github.worksoup.function.LumiaFunction",
            internal_closure_raw,
        );
        Function {
            instance,
            origin_func_raw,
            internal_closure_raw,
            _t: PhantomData,
            _r: PhantomData,
        }
    }
}
