use j4rs::{errors::J4RsError, Instance, InvocationArg, Jvm};
use std::cmp::Ordering;

/// 这个特征可以返回 java 中 Class 对象，监听事件的时候用。
/// 为了做泛型搞的。之后可能会改动。
pub trait GetClassTypeTrait {
    /// 获取该类在 `Java` 中的 `Class` 对象。
    fn get_class_type() -> Instance;

    fn cast_to_this_type(instance: Instance) -> Instance;

    fn get_type_name() -> &'static str;

    fn is_this_type(instance: &Instance) -> bool;
}

pub trait GetInstanceTrait {
    fn get_instance(&self) -> Result<Instance, J4RsError>;
}
impl GetInstanceTrait for bool {
    fn get_instance(&self) -> Result<Instance, J4RsError> {
        InvocationArg::try_from(self).map(|a| Instance::try_from(a).unwrap())
    }
}
impl GetInstanceTrait for Ordering {
    fn get_instance(&self) -> Result<Instance, J4RsError> {
        Instance::try_from(InvocationArg::try_from(unsafe {
            *{ self as *const Ordering as *const i8 }
        })?)
    }
}
pub trait AsInstanceTrait {
    fn as_instance(&self) -> &Instance;
}

/// 通过 `j4rs::Instance` 获得当前结构体。
pub trait TryFromInstanceTrait: Sized {
    fn try_from_instance(instance: Instance) -> Result<Self, J4RsError>;
}
/// 通过 `j4rs::Instance` 获得当前结构体。
pub trait FromInstanceTrait: Sized {
    fn from_instance(instance: Instance) -> Self;
}
impl FromInstanceTrait for Instance {
    fn from_instance(instance: Instance) -> Self {
        instance
    }
}
impl<T: TryFromInstanceTrait> FromInstanceTrait for T {
    default fn from_instance(instance: Instance) -> Self {
        Self::try_from_instance(instance).unwrap()
    }
}
macro_rules! impl_try_from_instance {
    ($($type:ty),+) => {
        $(
            impl TryFromInstanceTrait for $type {
                fn try_from_instance(instance: Instance) -> Result<Self, J4RsError> {
                    Jvm::attach_thread()?.to_rust(instance)
                }
            }
        )+
    };
}

impl_try_from_instance!(String, bool, char, i8, i16, i32, i64, f32, f64);

/// for `Comparator`.
impl TryFromInstanceTrait for Ordering {
    fn try_from_instance(instance: Instance) -> Result<Self, J4RsError> {
        Ok(i32::try_from_instance(instance)?.cmp(&0))
    }
}
impl TryFromInstanceTrait for Instance {
    fn try_from_instance(instance: Instance) -> Result<Self, J4RsError> {
        Ok(instance)
    }
}
/// 该特型表示可以将该类型构造为 [`InvocationArg`], 方便作为 java 调用的参数传入。
pub trait ToArgTrait {
    fn to_arg(&self) -> Result<InvocationArg, J4RsError>;
}
/// 该特型表示可以将该类型转换为 [`InvocationArg`], 方便作为 java 调用的参数传入。
pub trait IntoArgTrait {
    fn into_arg(self) -> Result<InvocationArg, J4RsError>;
}

impl<T, Error> ToArgTrait for T
where
    T: Copy,
    InvocationArg: TryFrom<T, Error = Error>,
    J4RsError: From<Error>,
{
    default fn to_arg(&self) -> Result<InvocationArg, J4RsError> {
        Ok(InvocationArg::try_from(*self)?)
    }
}
impl<T, Error> IntoArgTrait for T
where
    InvocationArg: TryFrom<T, Error = Error>,
    J4RsError: From<Error>,
{
    default fn into_arg(self) -> Result<InvocationArg, J4RsError> {
        Ok(InvocationArg::try_from(self)?)
    }
}
