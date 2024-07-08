use std::marker::PhantomData;
use std::ops::Deref;
use j4rs::errors::J4RsError;
use j4rs::{Instance, InvocationArg, Jvm};
use jbc_derive::{AsInstanceDerive, GetInstanceDerive, java_type, NewType, TryFromInstanceDerive};
use crate::traits::{GetInstanceTrait, TryFromInstanceTrait};
use crate as jbuchong;

#[derive(NewType)]
pub struct JavaString(String);


impl TryFromInstanceTrait for JavaString {
    fn try_from_instance(instance: Instance) -> Result<Self, J4RsError> {
        let jvm = Jvm::attach_thread().unwrap();
        jvm.to_rust::<String>(instance).map(|r| r.into())
    }
}

impl GetInstanceTrait for JavaString {
    fn get_instance(&self) -> Result<Instance, J4RsError> {
        Instance::try_from(InvocationArg::try_from(self.deref())?)
    }
}

#[derive(NewType)]
pub struct JavaBytes(Vec<i8>);

impl TryFromInstanceTrait for JavaBytes {
    fn try_from_instance(instance: Instance) -> Result<Self, J4RsError> {
        let jvm = Jvm::attach_thread().unwrap();
        jvm.to_rust::<Vec<_>>(instance).map(|r| r.into())
    }
}

#[derive(NewType, GetInstanceDerive, TryFromInstanceDerive, AsInstanceDerive)]
pub struct InstanceWrapper(Instance);

impl InstanceWrapper {
    pub fn get<T>(&self) -> Result<T, J4RsError>
    where
        T: TryFromInstanceTrait,
    {
        T::try_from_instance(Jvm::attach_thread()?.clone_instance(&self)?)
    }
}

#[java_type("io.github.worksoup.LumiaPair")]
#[derive(NewType, GetInstanceDerive, TryFromInstanceDerive, AsInstanceDerive)]
pub struct Pair<F, S> (Instance, PhantomData<(F, S)>,
);

impl<F, S> Pair<F, S>
where
    F: GetInstanceTrait,
    S: GetInstanceTrait,
{
    pub fn new(f: F, s: S) -> Result<Self, J4RsError> {
        let jvm = Jvm::attach_thread()?;
        let v1 = InvocationArg::from(f.get_instance()?);
        let v2 = InvocationArg::from(s.get_instance()?);
        jvm.create_instance("io.github.worksoup.LumiaPair", &[v1, v2]).map(Self::from)
    }
}
impl<F, S> Pair<F, S>
where
    F: TryFromInstanceTrait,
    S: TryFromInstanceTrait,
{
    pub fn get_pair(&self) -> Result<(F, S), J4RsError> {
        let jvm = Jvm::attach_thread()?;
        let instance = jvm.cast(&self, "io.github.worksoup.LumiaPair")?;
        let val1 = jvm.invoke(&instance, "first", InvocationArg::empty())?;
        let val2 = jvm.invoke(&instance, "second", InvocationArg::empty())?;
        let val1 = F::try_from_instance(val1)?;
        let val2 = S::try_from_instance(val2)?;
        Ok((val1, val2))
    }
}
#[java_type("kotlin.Pair")]
#[derive(NewType, GetInstanceDerive, TryFromInstanceDerive, AsInstanceDerive)]
pub struct KotlinPair<F, S> (Instance, PhantomData<(F, S)>);
impl<F, S> KotlinPair<F, S>
where
    F: GetInstanceTrait,
    S: GetInstanceTrait,
{
    pub fn new(f: F, s: S) -> Result<Self, J4RsError> {
        let jvm = Jvm::attach_thread()?;
        let v1 = InvocationArg::from(f.get_instance()?);
        let v2 = InvocationArg::from(s.get_instance()?);
        jvm.create_instance("kotlin.Pair", &[v1, v2]).map(Self::from)
    }
}
impl<F, S> KotlinPair<F, S>
where
    F: TryFromInstanceTrait,
    S: TryFromInstanceTrait,
{
    pub fn get_pair(&self) -> Result<(F, S), J4RsError> {
        let jvm = Jvm::attach_thread()?;
        let instance = jvm.cast(&self, "kotlin.Pair")?;
        let val1 = jvm.invoke(&instance, "getFirst", InvocationArg::empty())?;
        let val2 = jvm.invoke(&instance, "getSecond", InvocationArg::empty())?;
        let val1 = F::try_from_instance(val1)?;
        let val2 = S::try_from_instance(val2)?;
        Ok((val1, val2))
    }
}
#[java_type("kotlin.Unit")]
#[derive(NewType, GetInstanceDerive, TryFromInstanceDerive, AsInstanceDerive)]
pub struct KotlinUnit(Instance);

impl KotlinUnit {
    pub fn new() -> Result<Self, J4RsError> {
        Jvm::attach_thread()?.static_class("kotlin.Unit$INSTANCE").map(Self::from)
    }
}

impl Default for KotlinUnit {
    fn default() -> Self {
        Self::new().unwrap()
    }
}