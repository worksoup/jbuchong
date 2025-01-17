use crate::BiFunction;
use j4rs::{errors::J4RsError, Instance, InvocationArg, Jvm};
use jbc_base::{self as jbuchong, GetInstanceTrait, TryFromInstanceTrait};
use jbc_derive::GetInstanceDerive;

#[derive(GetInstanceDerive)]
pub struct Func2<T, U, R> {
    instance: Instance,
    func: BiFunction<T, U, R>,
}
impl<T, U, R> Func2<T, U, R> {
    pub fn drop(self) {
        self.func.drop()
    }
    pub fn call(&self, t: T, u: U) -> R {
        self.func.call(t, u)
    }
}
impl<T, U, R> Func2<T, U, R>
where
    R: TryFromInstanceTrait,
{
    pub fn invoke(&self, t: InvocationArg, u: InvocationArg) -> Result<R, J4RsError> {
        let jvm = Jvm::attach_thread()?;
        let result = jvm.invoke(&self.get_instance()?, "invoke", &[t, u])?;
        R::try_from_instance(result)
    }
}

impl<T, U, R> Func2<T, U, R>
where
    T: TryFromInstanceTrait,
    U: TryFromInstanceTrait,
    R: GetInstanceTrait,
{
    pub fn new<F>(closure: F) -> Func2<T, U, R>
    where
        F: Fn(T, U) -> R + 'static,
    {
        let internal_fn = move |t: T, u: U| -> R { closure(t, u) };
        let func = BiFunction::new(internal_fn);
        let jvm = Jvm::attach_thread().unwrap();
        let instance = jvm
            .create_instance(
                "io.github.worksoup.function.JBuChongKtFunc2",
                &[InvocationArg::from(func.get_instance().unwrap())],
            )
            .unwrap();
        Func2 { instance, func }
    }
}
