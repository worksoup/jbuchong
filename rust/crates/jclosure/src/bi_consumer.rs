use crate::Consumer;
use j4rs::{errors::J4RsError, Instance, InvocationArg, Jvm};
use jbc_base::{self as jbuchong, GetInstanceTrait, Pair, TryFromInstanceTrait};
use jbc_derive::GetInstanceDerive;

#[derive(GetInstanceDerive)]
pub struct BiConsumer<T1, T2> {
    instance: Instance,
    func: Consumer<Pair<T1, T2>>,
}
impl<T1, T2> BiConsumer<T1, T2> {
    pub fn drop(self) {
        self.func.drop()
    }
    pub fn accept(&self, v1: InvocationArg, v2: InvocationArg) -> Result<(), J4RsError> {
        let jvm = Jvm::attach_thread()?;
        let _ = jvm.invoke(&self.get_instance()?, "accept", &[v1, v2])?;
        Ok(())
    }
    pub fn call(&self, v1: T1, v2: T2) {
        self.func.call(Pair::new(v1, v2))
    }
}

impl<T1, T2> BiConsumer<T1, T2>
where
    T1: TryFromInstanceTrait,
    T2: TryFromInstanceTrait,
{
    pub fn new<F>(closure: F) -> BiConsumer<T1, T2>
    where
        F: Fn(T1, T2) + 'static,
    {
        let internal_fn = move |v: Pair<T1, T2>| {
            let (v1, v2) = v.into_inner();
            closure(v1, v2)
        };
        let func = Consumer::new(internal_fn);
        let jvm = Jvm::attach_thread().unwrap();
        let instance = jvm
            .create_instance(
                "io.github.worksoup.function.LumiaBiConsumer",
                &[InvocationArg::from(func.get_instance().unwrap())],
            )
            .unwrap();
        BiConsumer { instance, func }
    }
}
