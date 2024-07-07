//! 这里存放了开发 `mirai_j4rs` 时用到的一些宏。
use proc_macro::TokenStream;
use quote::quote;
use syn::{Data, DeriveInput, Field, Fields, Type};
#[proc_macro]
pub fn impl_kt_func_n(_input: TokenStream) -> TokenStream {
    let import = r#"
use crate::Func2;
use j4rs::errors::J4RsError;
use j4rs::{{Instance, InvocationArg, Jvm}};
use jbc_base::{{GetInstanceTrait, TryFromInstanceTrait}};
use jbc_derive::GetInstanceDerive;
use jbc_base::DataWrapper;
use jbc_base::TypeName;
    "#.to_string();
    let mut r = vec![import];
    for n in 3..=16 {
        r.push(impl_kt_func_n_(n));
    }
    let r = r.join("\n");
    let mut tokens = proc_macro2::TokenStream::new();
    let token: proc_macro2::TokenStream = r.parse().unwrap();
    tokens.extend(quote! {#token});
    tokens.into()
}
fn impl_kt_func_n_(n: usize) -> String {
    let type_params = &"ABCDEFGHIJKLMNOP"[0..n];
    let upper_params = type_params
        .chars()
        .map(|c| format!("{c}"))
        .collect::<Vec<_>>();
    let type_params_1 = upper_params
        .join(", ");
    let type_params_2 = upper_params[2..]
        .join(", ");
    let type_params_3 = "ABCDEFGHIJKLMNOP"[0..n]
        .chars()
        .map(|c| format!("{}: InvocationArg", c.to_lowercase()))
        .collect::<Vec<_>>()
        .join(", ");
    let type_params_4 = "ABCDEFGHIJKLMNOP"[0..n]
        .chars()
        .map(|c| format!("{}", c.to_lowercase(), ))
        .collect::<Vec<_>>()
        .join(", ");
    let type_params_5 = "ABCDEFGHIJKLMNOP"[2..n]
        .chars()
        .map(|c| format!("{}: {}", c.to_lowercase(), c))
        .collect::<Vec<_>>()
        .join(", ");
    let where_params = "ABCDEFGHIJKLMNOP"[0..n]
        .chars()
        .map(|c| format!("{c}: TryFromInstanceTrait"))
        .collect::<Vec<_>>()
        .join(",\n");
    let type_name = format!("Func{n}");
    let last_type_name = format!("Func{}", n - 1);
    format!(
        r#"
#[derive(GetInstanceDerive)]
pub struct {type_name}<{type_params_1}, R> {{
    instance: Instance,
    func: {last_type_name}<DataWrapper<(A, B), TypeName<"kotlin.Pair">>, {type_params_2}, R>,
}}
impl<{type_params_1}, R> {type_name}<{type_params_1}, R> {{
    pub fn drop(self) {{
        self.func.drop()
    }}
}}
impl<{type_params_1}, R> {type_name}<{type_params_1}, R>
where
    R: TryFromInstanceTrait,
{{
    pub fn invoke(&self, {type_params_3}) -> Result<R, J4RsError> {{
        let jvm = Jvm::attach_thread()?;
        let result = jvm.invoke(&self.get_instance()?, "invoke", &[{type_params_4}])?;
        R::try_from_instance(result)
    }}
}}

impl<{type_params_1}, R> {type_name}<{type_params_1}, R>
where
    {where_params},
    R: GetInstanceTrait,
{{
    pub fn new<Func>(closure: Func) -> {type_name}<{type_params_1}, R>
    where
        Func: Fn({type_params_1}) -> R + 'static,
    {{
        let internal_fn = move |v: DataWrapper<(A, B), TypeName<"kotlin.Pair">>, {type_params_5}| -> R {{ let (a, b) = v.get_pair(); closure({type_params_4}) }};
        let func = {last_type_name}::new(internal_fn);
        let jvm = Jvm::attach_thread().unwrap();
        let instance = jvm
            .create_instance(
                "io.github.worksoup.function.LumiaKt{type_name}",
                &[InvocationArg::from(func.get_instance().unwrap())],
            )
            .unwrap();
        {type_name} {{ instance, func }}
    }}
}}
        "#,
    )
}

fn impl_get_as(
    ast_data: &Data,
    name: &proc_macro2::Ident,
    struct_impl: proc_macro2::TokenStream,
    fn_name: proc_macro2::TokenStream,
) -> proc_macro2::TokenStream {
    match &ast_data {
        Data::Struct(_) => struct_impl,
        Data::Enum(data_enum) => {
            let variants = &data_enum.variants;
            let tokens = variants.iter().map(|variant| {
                let ident = &variant.ident;
                quote!(
                    #name::#ident(a) => a.#fn_name(),
                )
            });
            quote!(
                match self {
                    #(#tokens)*
                }
            )
        }
        Data::Union(_) => {
            panic!("不支持使用 `union`!")
        }
    }
}

/// ### `GetInstanceDerive`
///
/// 为特定的结构体和枚举类型实现 [`GetInstanceTrait`](jbc_base::GetInstanceTrait).
///
/// 这些类型需要满足如下条件：
///
/// - 结构体必须拥有 `instance: j4rs::Instance,` 字段。
/// - 枚举值则必须类似于此：
///   ```rust
///   struct AType;
///   struct BType;
///   enum Enum{
///     A(AType),
///     B(BType),
///   }
///   ```
///   并且如上代码，`AType` 和 `BType` 都必须实现 `GetInstanceTrait`.
#[proc_macro_derive(GetInstanceDerive)]
pub fn get_instance_derive(input: TokenStream) -> TokenStream {
    let ast: &DeriveInput = &syn::parse(input).unwrap();
    let name = &ast.ident;
    let generics = &ast.generics;
    let r#impl = impl_get_as(
        &ast.data,
        name,
        quote!(j4rs::Jvm::attach_thread()?.clone_instance(&self.instance)),
        quote!(get_instance),
    );
    let (impl_generics, ty_generics, where_clause) = generics.split_for_impl();
    let gen = quote! {
        impl #impl_generics jbc_base::GetInstanceTrait for #name #ty_generics #where_clause {
            fn get_instance(&self) -> Result<j4rs::Instance,j4rs::errors::J4RsError>{
                #r#impl
            }
        }
    };
    gen.into()
}

/// ### `AsInstanceDerive`
///
/// 与 [`GetInstanceDerive`] 类似。
#[proc_macro_derive(AsInstanceDerive)]
pub fn as_instance_derive(input: TokenStream) -> TokenStream {
    let ast: &DeriveInput = &syn::parse(input).unwrap();
    let name = &ast.ident;
    let generics = &ast.generics;
    let r#impl = impl_get_as(&ast.data, name, quote!(&self.instance), quote!(as_instance));
    let (impl_generics, ty_generics, where_clause) = generics.split_for_impl();
    let gen = quote! {
        impl #impl_generics jbc_base::AsInstanceTrait for #name #ty_generics #where_clause {
            fn as_instance(&self) -> &j4rs::Instance{
                #r#impl
            }
        }
    };
    gen.into()
}
/// ### `TryFromInstanceDerive`
///
/// 为特定的结构体和枚举类型实现 [`TryFromInstanceTrait`](jbc_base::env::TryFromInstanceTrait).
///
/// 这些类型需要满足如下条件：
///
/// - 结构体必须拥有 `instance: `[`j4rs::Instance`]`,` 字段，且其余字段必须都是 [`std::marker::PhantomData`] 类型。
/// - 枚举值则必须类似于此：
///
///   ``` not_test
///   struct AType;
///   struct BType;
///   enum Enum{
///     A(AType),
///     #[fall] // 使用 `TryFromInstanceDerive` 时可选为分支添加 `fall` 属性。
///     B(BType),
///   }
///   ```
///
///   并且如上代码，`AType` 和 `BType` 都必须实现 `TryFromInstanceTrait`.
///   其中 `fall` 意味着未能成功转换的类型将会落到该分支中。如果没有该属性，未能成功转换时将会造成 `panic!`, 如果存在多个，则最后一个有效。
#[proc_macro_derive(TryFromInstanceDerive, attributes(fall))]
pub fn from_instance_derive(input: TokenStream) -> TokenStream {
    fn type_is_phantom_data(field: &Field) -> bool {
        if let Type::Path(ref ty) = field.ty {
            if let Some(ty) = ty.path.segments.last() {
                return ty.ident == "PhantomData";
            }
        }
        false
    }
    fn fill_phantom_data_fields(fields: &Fields) -> proc_macro2::TokenStream {
        let mut tokens = proc_macro2::TokenStream::new();
        for field in fields {
            if type_is_phantom_data(field) {
                let field_name = field.ident.as_ref();
                if let Some(field_name) = field_name {
                    tokens.extend(quote!(#field_name:std::marker::PhantomData::default(),))
                }
            }
        }
        tokens
    }
    let ast: &DeriveInput = &syn::parse(input).unwrap();
    let name = &ast.ident;
    let generics = &ast.generics;
    let impl_tokens = match &ast.data {
        Data::Struct(s) => {
            let tmp = fill_phantom_data_fields(&s.fields);
            quote!(
                Self{
                    instance,
                    #tmp
                }
            )
        }
        Data::Enum(e) => {
            let variants = &e.variants;
            let mut fall_arm = variants.first();
            let mut impl_tokens = proc_macro2::TokenStream::new();
            for variant in variants {
                let has_this_attr = if let Some(field_attr) = variant.attrs.first() {
                    if let Some(ident) = field_attr.path().get_ident() {
                        ident == "fall"
                    } else {
                        false
                    }
                } else {
                    false
                };
                if has_this_attr {
                    fall_arm = Some(variant);
                } else {
                    let ty = match &variant.fields {
                        Fields::Unnamed(fields) => {
                            &fields.unnamed.first().expect("无名枚举没有字段！").ty
                        }
                        _ => {
                            panic!("不支持无内含值的枚举以及有名枚举！")
                        }
                    };
                    let ident = &variant.ident;
                    impl_tokens.extend(quote!(
                        if <#ty as jbc_base::env::GetClassTypeTrait>::is_this_type(&instance) {
                            #name::#ident(
                                #ty::try_from_instance(
                                    <#ty as jbc_base::env::GetClassTypeTrait>::cast_to_this_type(instance)
                                )?
                            )
                        } else
                    ))
                }
            }
            if let Some(fall_arm) = fall_arm {
                let fall_arm_ty = match &fall_arm.fields {
                    Fields::Unnamed(fields) => {
                        &fields.unnamed.first().expect("无名枚举没有字段！").ty
                    }
                    _ => {
                        panic!("不支持无内含值的枚举以及有名枚举！")
                    }
                };
                let fall_arm_ident = &fall_arm.ident;
                impl_tokens.extend(quote!(
                    {#name::#fall_arm_ident(<#fall_arm_ty as jbc_base::env::TryFromInstanceTrait>::try_from_instance(instance)?)}
                ));
            } else {
                impl_tokens.extend(quote!({ panic!("意料之外的类型！") }))
            }
            impl_tokens
        }
        Data::Union(_) => panic!("不支持使用 `union`!"),
    };
    let (impl_generics, ty_generics, where_clause) = generics.split_for_impl();
    let gen = quote! {
        impl #impl_generics jbc_base::TryFromInstanceTrait for #name #ty_generics #where_clause {
            fn try_from_instance(instance: j4rs::Instance) -> Result<Self,j4rs::errors::J4RsError>{
                Ok(#impl_tokens)
            }
        }
    };
    gen.into()
}

/// ### `java_type`
//
/// 为结构体、枚举等实现 [`GetClassTypeTrait`](jbc_base::env::GetClassTypeTrait).
///
/// 接受一个字符串字面值参数，类似于此：
///
/// ```not_test
/// use jbc_derive::java_type;
/// #[java_type("io.github.worksoup.LumiaUtils")]
/// struct LumiaUtils{}
/// ```
///
/// 对结构体或枚举等没有特殊要求。
#[proc_macro_attribute]
pub fn java_type(type_name: TokenStream, input: TokenStream) -> TokenStream {
    let ast: &DeriveInput = &syn::parse(input).unwrap();
    let type_name: &syn::LitStr = &syn::parse(type_name).expect("类型名称请用字符串表示！");
    let name = &ast.ident;
    let generics = &ast.generics;
    let (impl_generics, ty_generics, where_clause) = generics.split_for_impl();
    let gen = quote! {
        #ast
        impl #impl_generics jbc_base::GetClassTypeTrait for #name #ty_generics #where_clause {
            fn get_class_type() -> j4rs::Instance {
                j4rs::Jvm::attach_thread()
                    .unwrap()
                    .invoke_static(
                        "io.github.worksoup.LumiaUtils",
                        "forName",
                        &[j4rs::InvocationArg::try_from(
                            Self::get_type_name().as_str(),
                        )
                        .unwrap()],
                    )
                    .unwrap()
            }
            fn cast_to_this_type(instance: j4rs::Instance) -> j4rs::Instance {
                let jvm = j4rs::Jvm::attach_thread()
                    .unwrap();
                jvm.cast(&instance, Self::get_type_name().as_str()).unwrap()
            }
            fn get_type_name() -> String {
                jbc_base::MIRAI_PREFIX.to_string() + #type_name
            }
            fn is_this_type(instance: &j4rs::Instance) -> bool {
                jbc_base::utils::is_instance_of(&instance, Self::get_type_name().as_str())
            }
        }
    };
    gen.into()
}
