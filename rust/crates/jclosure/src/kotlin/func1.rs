use crate::Function;
use j4rs::{errors::J4RsError, Instance, InvocationArg, Jvm};
use jbc_base::{self as jbuchong, GetInstanceTrait, TryFromInstanceTrait};
use jbc_derive::GetInstanceDerive;

#[derive(GetInstanceDerive)]
pub struct Func1<T, R> {
    instance: Instance,
    func: Function<T, R>,
}
impl<T, R> Func1<T, R> {
    pub fn drop(self) {
        self.func.drop()
    }
    pub fn call(&self, arg: T) -> R {
        self.func.call(arg)
    }
}
impl<T, R> Func1<T, R>
where
    R: TryFromInstanceTrait,
{
    pub fn invoke(&self, t: InvocationArg) -> Result<R, J4RsError> {
        let jvm = Jvm::attach_thread()?;
        let result = jvm.invoke(&self.get_instance()?, "invoke", &[t])?;
        R::try_from_instance(result)
    }
}

impl<T, R> Func1<T, R>
where
    T: TryFromInstanceTrait,
    R: GetInstanceTrait,
{
    pub fn new<F>(closure: F) -> Func1<T, R>
    where
        F: Fn(T) -> R + 'static,
    {
        let internal_fn = move |t: T| -> R { closure(t) };
        let func = Function::new(internal_fn);
        let jvm = Jvm::attach_thread().unwrap();
        let instance = jvm
            .create_instance(
                "io.github.worksoup.function.JBuChongKtFunc1",
                &[InvocationArg::from(func.get_instance().unwrap())],
            )
            .unwrap();
        Func1 { instance, func }
    }
}
