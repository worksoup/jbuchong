use crate::Supplier;
use j4rs::{errors::J4RsError, Instance, InvocationArg, Jvm};
use jbc_base::{self as jbuchong, GetInstanceTrait, TryFromInstanceTrait};
use jbc_derive::GetInstanceDerive;

#[derive(GetInstanceDerive)]
pub struct Func0<R> {
    instance: Instance,
    func: Supplier<R>,
}
impl<R> Func0<R> {
    pub fn drop(self) {
        self.func.drop()
    }
    pub fn call(&self) -> R {
        self.func.call()
    }
}
impl<R> Func0<R>
where
    R: TryFromInstanceTrait,
{
    pub fn invoke(&self) -> Result<R, J4RsError> {
        let jvm = Jvm::attach_thread()?;
        let result = jvm.invoke(&self.get_instance()?, "invoke", InvocationArg::empty())?;
        R::try_from_instance(result)
    }
}

impl<R> Func0<R>
where
    R: GetInstanceTrait,
{
    pub fn new<F>(closure: F) -> Func0<R>
    where
        F: Fn() -> R + 'static,
    {
        let internal_fn = move || -> R { closure() };
        let func = Supplier::new(internal_fn);
        let jvm = Jvm::attach_thread().unwrap();
        let instance = jvm
            .create_instance(
                "io.github.worksoup.function.JBuChongKtFunc0",
                &[InvocationArg::from(func.get_instance().unwrap())],
            )
            .unwrap();
        Func0 { instance, func }
    }
}
