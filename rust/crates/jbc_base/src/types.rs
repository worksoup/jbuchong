use crate::traits::{GetInstanceTrait, TryFromInstanceTrait};
use j4rs::{errors::J4RsError, Instance, InvocationArg, Jvm};
use jbc_derive::{java_type, AsInstanceDerive, GetInstanceDerive, NewType, TryFromInstanceDerive};
use std::ops::Deref;

mod jbuchong {
    pub use crate::*;
    pub use jbc_derive::*;
}

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
        T::try_from_instance(Jvm::attach_thread()?.clone_instance(self)?)
    }
}
#[java_type("io.github.worksoup.LumiaPair")]
#[derive(NewType)]
pub struct Pair<F, S>((F, S));
impl<F, S> Pair<F, S> {
    pub fn new(f: F, s: S) -> Pair<F, S> {
        Pair((f, s))
    }
}
// impl<F, S> From<(F, S)> for Pair<F, S> {
//     fn from(other: (F, S)) -> Pair<F, S> {
//         Pair(other)
//     }
// }
//
// impl<F, S> Deref for Pair<F, S> {
//     type Target = (F, S);
//     fn deref(&self) -> &Self::Target {
//         &self.0
//     }
// }
// impl<F, S> DerefMut for Pair<F, S>
// where
//     Self: Deref<Target = (F, S)>,
// {
//     fn deref_mut(&mut self) -> &mut Self::Target {
//         &mut self.0
//     }
// }
// impl<F, S> Pair<F, S> {
//     #[doc = r" Unwrap to the inner type"]
//     pub fn into_inner(self) -> (F, S) {
//         self.0
//     }
// }
impl<F, S> GetInstanceTrait for Pair<F, S>
where
    F: GetInstanceTrait,
    S: GetInstanceTrait,
{
    fn get_instance(&self) -> Result<Instance, J4RsError> {
        let jvm = Jvm::attach_thread()?;
        let v1 = InvocationArg::from(self.0 .0.get_instance()?);
        let v2 = InvocationArg::from(self.0 .1.get_instance()?);
        jvm.create_instance("io.github.worksoup.LumiaPair", &[v1, v2])
    }
}
impl<F, S> TryFromInstanceTrait for Pair<F, S>
where
    F: TryFromInstanceTrait,
    S: TryFromInstanceTrait,
{
    fn try_from_instance(instance: Instance) -> Result<Pair<F, S>, J4RsError> {
        let jvm = Jvm::attach_thread()?;
        let instance = jvm.cast(&instance, "io.github.worksoup.LumiaPair")?;
        let val1 = jvm.invoke(&instance, "first", InvocationArg::empty())?;
        let val2 = jvm.invoke(&instance, "second", InvocationArg::empty())?;
        let val1 = F::try_from_instance(val1)?;
        let val2 = S::try_from_instance(val2)?;
        Ok(Self::new(val1, val2))
    }
}
#[java_type("kotlin.Unit")]
#[derive(NewType)]
pub struct KotlinPair<F, S>((F, S));
impl<F, S> KotlinPair<F, S> {
    pub fn new(f: F, s: S) -> KotlinPair<F, S> {
        KotlinPair((f, s))
    }
}
// impl<F, S> From<(F, S)> for KotlinPair<F, S> {
//     fn from(other: (F, S)) -> KotlinPair<F, S> {
//         KotlinPair(other)
//     }
// }
//
// impl<F, S> Deref for KotlinPair<F, S> {
//     type Target = (F, S);
//     fn deref(&self) -> &Self::Target {
//         &self.0
//     }
// }
// impl<F, S> DerefMut for KotlinPair<F, S>
// where
//     Self: Deref<Target = (F, S)>,
// {
//     fn deref_mut(&mut self) -> &mut Self::Target {
//         &mut self.0
//     }
// }
// impl<F, S> KotlinPair<F, S> {
//     #[doc = r" Unwrap to the inner type"]
//     pub fn into_inner(self) -> (F, S) {
//         self.0
//     }
// }
impl<F, S> GetInstanceTrait for KotlinPair<F, S>
where
    F: GetInstanceTrait,
    S: GetInstanceTrait,
{
    fn get_instance(&self) -> Result<Instance, J4RsError> {
        let jvm = Jvm::attach_thread()?;
        let v1 = InvocationArg::from(self.0 .0.get_instance()?);
        let v2 = InvocationArg::from(self.0 .1.get_instance()?);
        jvm.create_instance("kotlin.Pair", &[v1, v2])
    }
}
impl<F, S> TryFromInstanceTrait for KotlinPair<F, S>
where
    F: TryFromInstanceTrait,
    S: TryFromInstanceTrait,
{
    fn try_from_instance(instance: Instance) -> Result<KotlinPair<F, S>, J4RsError> {
        let jvm = Jvm::attach_thread()?;
        let instance = jvm.cast(&instance, "kotlin.Pair")?;
        let val1 = jvm.invoke(&instance, "getFirst", InvocationArg::empty())?;
        let val2 = jvm.invoke(&instance, "getSecond", InvocationArg::empty())?;
        let val1 = F::try_from_instance(val1)?;
        let val2 = S::try_from_instance(val2)?;
        Ok(Self::new(val1, val2))
    }
}
#[derive(
    jbuchong::AsInstanceDerive,
    jbuchong::TryFromInstanceDerive,
    jbuchong::GetInstanceDerive,
    jbuchong::ToArgDerive,
    jbuchong::IntoArgDerive,
)]
#[jbuchong::java_type("kotlin. Unit")]
#[derive(NewType)]
pub struct KotlinUnit(Instance);

impl KotlinUnit {
    pub fn new() -> Result<Self, J4RsError> {
        Jvm::attach_thread()?
            .static_class("kotlin.Unit$INSTANCE")
            .map(Self::from)
    }
}

impl Default for KotlinUnit {
    fn default() -> Self {
        Self::new().unwrap()
    }
}
impl From<()> for KotlinUnit {
    fn from(_: ()) -> Self {
        Self::default()
    }
}
