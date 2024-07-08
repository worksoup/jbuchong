use j4rs::errors::J4RsError;
use j4rs::{Instance, InvocationArg, Jvm};
use jbc_base::{DataWrapper, GetInstanceTrait, TryFromInstanceTrait};
use jbc_derive::GetInstanceDerive;
use crate::Consumer;

#[derive(GetInstanceDerive)]
pub struct BiConsumer<T1, T2, > {
    instance: Instance,
    func: Consumer<DataWrapper<(T1, T2)>, >,
}
impl<T1, T2, > BiConsumer<T1, T2, > {
    pub fn drop(self) {
        self.func.drop()
    }
    pub fn accept(&self, v1: InvocationArg, v2: InvocationArg) -> Result<(), J4RsError> {
        let jvm = Jvm::attach_thread()?;
        let _ = jvm.invoke(&self.get_instance()?, "accept", &[v1, v2])?;
        Ok(())
    }
}

impl<T1, T2, > BiConsumer<T1, T2, >
where
    T1: TryFromInstanceTrait,
    T2: TryFromInstanceTrait,
{
    pub fn new<F>(closure: F) -> BiConsumer<T1, T2, >
    where
        F: Fn(T1, T2) + 'static,
    {
        let internal_fn = move |v: DataWrapper<(T1, T2)>| {
            let (v1, v2) = v.get_pair();
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
