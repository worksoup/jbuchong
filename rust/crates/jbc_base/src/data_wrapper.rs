//! 有时 rust 中的类型可以对应于 java 中不同的类型，
//! 例如基本类型对应了 java 中装箱与否的两种类型；
//! 二元元组类型又可能对应了不同库中实现的 `Pair` 类型。
//!
//! [`DataWrapper`] 是一个简单的包装类型，通过一个自定义的标识来使这样的对应关系更好处理。
use std::marker::PhantomData;
use std::ops::Deref;
use j4rs::errors::J4RsError;
use j4rs::{Instance, InvocationArg, Jvm};
use crate::traits::{GetInstanceTrait, TryFromInstanceTrait};
/// 标记结构体，用以填充 [`DataWrapper`](crate::data_wrapper::DataWrapper) 的第二个类型参数，表示为默认实现。
#[derive(Default)]
pub struct DefaultCast;

#[derive(Default)]
pub struct TypeName<const TYPE_NAME: &'static str>;
impl<const TYPE_NAME: &'static str> TypeName<TYPE_NAME> {
    pub fn get_type_name(self) -> &'static str {
        TYPE_NAME
    }
}
pub struct DataWrapper<T, M = DefaultCast>
where
    M: Default,
{
    data: T,
    _label: PhantomData<M>,
}

impl<T, M: Default> DataWrapper<T, M> {
    pub fn get_marker() -> M {
        M::default()
    }
}

impl<T> Deref for DataWrapper<T, DefaultCast> {
    type Target = T;

    fn deref(&self) -> &Self::Target {
        &self.data
    }
}

impl<T, M: Default> From<T> for DataWrapper<T, M> {
    fn from(data: T) -> Self {
        Self { data, _label: PhantomData }
    }
}

impl TryFromInstanceTrait for DataWrapper<String, DefaultCast> {
    fn try_from_instance(instance: Instance) -> Result<Self, J4RsError> {
        let jvm = Jvm::attach_thread().unwrap();
        jvm.to_rust::<String>(instance).map(|r| r.into())
    }
}

impl GetInstanceTrait for DataWrapper<String, DefaultCast> {
    fn get_instance(&self) -> Result<Instance, J4RsError> {
        Instance::try_from(InvocationArg::try_from(&self.data)?)
    }
}

impl TryFromInstanceTrait for DataWrapper<Vec<i8>, DefaultCast> {
    fn try_from_instance(instance: Instance) -> Result<Self, J4RsError> {
        let jvm = Jvm::attach_thread().unwrap();
        jvm.to_rust::<Vec<_>>(instance).map(|r| r.into())
    }
}

impl<P1, P2, M: Default> DataWrapper<(P1, P2), M>
where
    P1: TryFromInstanceTrait,
    P2: TryFromInstanceTrait,
{
    pub fn get_pair(self) -> (P1, P2) {
        self.data
    }
}

impl<M: Default> DataWrapper<Instance, M> {
    pub fn get<T>(&self) -> Result<T, J4RsError>
    where
        T: TryFromInstanceTrait,
    {
        T::try_from_instance(Jvm::attach_thread()?.clone_instance(&self.data)?)
    }
}

impl GetInstanceTrait for DataWrapper<Instance, DefaultCast> {
    fn get_instance(&self) -> Result<Instance, J4RsError> {
        let jvm = Jvm::attach_thread().unwrap();
        jvm.clone_instance(&self.data)
    }
}

impl TryFromInstanceTrait for DataWrapper<Instance, DefaultCast> {
    fn try_from_instance(instance: Instance) -> Result<Self, J4RsError> {
        Ok(Self { data: instance, _label: PhantomData })
    }
}

impl<T: TryFromInstanceTrait, M: Default> TryFromInstanceTrait for DataWrapper<T, M> {
    default fn try_from_instance(instance: Instance) -> Result<Self, J4RsError> {
        <T as TryFromInstanceTrait>::try_from_instance(instance).map(|r| r.into())
    }
}
impl<T: GetInstanceTrait, M: Default> GetInstanceTrait for DataWrapper<T, M> {
    fn get_instance(&self) -> Result<Instance, J4RsError> {
        <T as GetInstanceTrait>::get_instance(&self.data)
    }
}
