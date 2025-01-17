//! 这里存放了开发 `mirai_j4rs` 时用到的一些宏。
use proc_macro::TokenStream;
use quote::{quote, ToTokens};
use std::collections::HashMap;
use syn::{Data, DeriveInput, Field, Fields, GenericParam, Type};
use zdcz::{fill_default_fields, find_needed_field_index, get_field_attr};

#[proc_macro]
pub fn impl_kt_func_n(_input: TokenStream) -> TokenStream {
    let import = r#"
use crate::Func2;
use j4rs::errors::J4RsError;
use j4rs::{{Instance, InvocationArg, Jvm}};
    "#
        .to_string();
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
    let type_params = upper_params[2..].join(", ");
    let args_1 = "ABCDEFGHIJKLMNOP"[2..n]
        .chars()
        .map(|c| format!("{}: {}", c.to_lowercase(), c))
        .collect::<Vec<_>>()
        .join(", ");
    let args_2 = "ABCDEFGHIJKLMNOP"[0..n]
        .chars()
        .map(|c| format!("{}: InvocationArg", c.to_lowercase()))
        .collect::<Vec<_>>()
        .join(", ");
    let call_args = "ABCDEFGHIJKLMNOP"[2..n]
        .chars()
        .map(|c| format!("{}", c.to_lowercase(), ))
        .collect::<Vec<_>>()
        .join(", ");
    let where_params = "ABCDEFGHIJKLMNOP"[0..n]
        .chars()
        .map(|c| format!("{c}: jbuchong::TryFromInstanceTrait"))
        .collect::<Vec<_>>()
        .join(",\n");
    let type_name = format!("Func{n}");
    let last_type_name = format!("Func{}", n - 1);
    format!(
        r#"
#[jbuchong::java]
pub struct {type_name}<A, B, {type_params}, R> {{
    instance: Instance,
    func: {last_type_name}<jbuchong::KotlinPair<A, B>, {type_params}, R>,
}}
impl<A, B, {type_params}, R> {type_name}<A, B, {type_params}, R> {{
    pub fn drop(self) {{
        self.func.drop()
    }}
    pub fn call(&self, a: A, b: B, {args_1}) -> R {{
        self.func.call(jbuchong::KotlinPair::new(a, b), {call_args})
    }}
}}
impl<A, B, {type_params}, R> {type_name}<A, B, {type_params}, R>
where
    R: jbuchong::TryFromInstanceTrait,
{{
    pub fn invoke(&self, {args_2}) -> Result<R, J4RsError> {{
        let jvm = Jvm::attach_thread()?;
        let result = jvm.invoke(&jbuchong::GetInstanceTrait::get_instance(self)?, "invoke", &[a, b, {call_args}])?;
        R::try_from_instance(result)
    }}
}}

impl<A, B, {type_params}, R> {type_name}<A, B, {type_params}, R>
where
    {where_params},
    R: jbuchong::GetInstanceTrait,
{{
    pub fn new<Func>(closure: Func) -> {type_name}<A, B, {type_params}, R>
    where
        Func: Fn(A, B, {type_params}) -> R + 'static,
    {{
        let internal_fn = move |v: jbuchong::KotlinPair<A, B>, {args_1}| -> R {{ let (a, b) = v.into_inner(); closure(a, b, {call_args}) }};
        let func = {last_type_name}::new(internal_fn);
        let jvm = Jvm::attach_thread().unwrap();
        let instance = jvm
            .create_instance(
                "io.github.worksoup.function.JBuChongKt{type_name}",
                &[InvocationArg::from(jbuchong::GetInstanceTrait::get_instance(&func).unwrap())],
            )
            .unwrap();
        {type_name} {{ instance, func }}
    }}
}}
        "#,
    )
}

fn derive_impl<F: Fn(proc_macro2::TokenStream) -> proc_macro2::TokenStream>(
    ast_data: &Data,
    name: &proc_macro2::Ident,
    gen_fn_content: F,
    fn_name: proc_macro2::TokenStream,
) -> proc_macro2::TokenStream {
    match &ast_data {
        Data::Struct(st) => {
            let fields = &st.fields;
            let (len, th, fields_name) = find_needed_field_index(fields, type_is_instance);
            if len != 1 {
                panic!("存在多个 `Instance` 类型的字段！请确保只有一个。")
            }
            gen_fn_content(if let Some(fields_name) = fields_name {
                fields_name.to_token_stream()
            } else {
                th.to_string().parse().unwrap()
            })
        }
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
/// 为特定的结构体和枚举类型实现 [`GetInstanceTrait`](jbuchong::GetInstanceTrait).
///
/// 这些类型需要满足如下条件：
/// -
/// - 元组结构体和结构体必须有一个 [`j4rs::Instance`] 类型的字段。
/// - 枚举值则必须类似于此：
///   ```rust
///   struct AType;
///   struct BType;
///   enum Enum{
///     A(AType),
///     B(BType),
///   }
///   ```
///   如上代码，`AType` 和 `BType` 都必须实现 `GetInstanceTrait`.
#[proc_macro_derive(GetInstanceDerive)]
pub fn get_instance_derive(input: TokenStream) -> TokenStream {
    let ast: &DeriveInput = &syn::parse(input).unwrap();
    let name = &ast.ident;
    let generics = &ast.generics;
    let r#impl = derive_impl(
        &ast.data,
        name,
        |c| {
            quote! {
                j4rs::Jvm::attach_thread()?.clone_instance(&self.#c)
            }
        },
        quote!(get_instance),
    );
    let (impl_generics, ty_generics, where_clause) = generics.split_for_impl();
    let gen = quote! {
        impl #impl_generics jbuchong::GetInstanceTrait for #name #ty_generics #where_clause {
            fn get_instance(&self) -> Result<j4rs::Instance, j4rs::errors::J4RsError> {
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
    let r#impl = derive_impl(&ast.data, name, |c| quote!(&self.#c), quote!(as_instance));
    let (impl_generics, ty_generics, where_clause) = generics.split_for_impl();
    let gen = quote! {
        impl #impl_generics jbuchong::AsInstanceTrait for #name #ty_generics #where_clause {
            fn as_instance(&self) -> &j4rs::Instance{
                #r#impl
            }
        }
    };
    gen.into()
}

/// ### `ToArgDerive`
///
/// 与 [`GetInstanceDerive`] 类似。
#[proc_macro_derive(ToArgDerive)]
pub fn to_arg_derive(input: TokenStream) -> TokenStream {
    let ast: &DeriveInput = &syn::parse(input).unwrap();
    let name = &ast.ident;
    let generics = &ast.generics;
    let r#impl = derive_impl(
        &ast.data,
        name,
        |c| {
            quote! {
                j4rs::InvocationArg::try_from(j4rs::Jvm::attach_thread()?.clone_instance(&self.#c))
            }
        },
        quote!(to_arg),
    );
    let (impl_generics, ty_generics, where_clause) = generics.split_for_impl();
    let gen = quote! {
        impl #impl_generics jbuchong::ToArgTrait for #name #ty_generics #where_clause {
            fn to_arg(&self) -> Result<j4rs::InvocationArg, j4rs::errors::J4RsError>{
                #r#impl
            }
        }
    };
    gen.into()
}
/// ### `IntoArgDerive`
///
/// 与 [`GetInstanceDerive`] 类似。
#[proc_macro_derive(IntoArgDerive)]
pub fn into_arg_derive(input: TokenStream) -> TokenStream {
    let ast: &DeriveInput = &syn::parse(input).unwrap();
    let name = &ast.ident;
    let generics = &ast.generics;
    let r#impl = derive_impl(
        &ast.data,
        name,
        |c| {
            quote! {
                Ok(j4rs::InvocationArg::try_from(self.#c)?)
            }
        },
        quote!(into_arg),
    );
    let (impl_generics, ty_generics, where_clause) = generics.split_for_impl();
    let gen = quote! {
        impl #impl_generics jbuchong::IntoArgTrait for #name #ty_generics #where_clause {
            fn into_arg(self) -> Result<j4rs::InvocationArg, j4rs::errors::J4RsError>{
                #r#impl
            }
        }
    };
    gen.into()
}

fn type_is_instance(field: &Field) -> bool {
    if let Type::Path(ref ty) = field.ty {
        if let Some(ty) = ty.path.segments.last() {
            return ty.ident == "Instance";
        }
    }
    false
}
/// ### `TryFromInstanceDerive`
///
/// 为特定的结构体和枚举类型实现 [`TryFromInstanceTrait`](jbuchong::TryFromInstanceTrait).
///
/// 这些类型需要满足如下条件：
///
/// - 元组结构体或结构体的第一个[`j4rs::Instance`] 类型的字段会被传入的 `instance` 填充，其余的字段需要实现 `Default` 特型或指定了 `#[default(value|fn)]` 属性。
///   > 例如：
///   > - `#[default(value = v)]` 会生成类似于 `let field_name = v;` 的代码，然后初始化字段时会使用 `field_name`;
///     - `#[default(fn_name = fn_name)]` 会成类似于 `let field_name = fn_name(&instance);` 的代码，然后初始化字段时会使用 `field_name`;
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
#[proc_macro_derive(TryFromInstanceDerive, attributes(fall, default))]
pub fn from_instance_derive(input: TokenStream) -> TokenStream {
    let ast: &DeriveInput = &syn::parse(input).unwrap();
    let name = &ast.ident;
    let generics = &ast.generics;
    let impl_tokens = match &ast.data {
        Data::Struct(s) => {
            let (fields, init) =
                fill_default_fields(&s.fields, type_is_instance, &"instance".parse().unwrap());
            quote!(
                #(#init)*
                Ok(Self #fields)
            )
        }
        Data::Enum(e) => {
            let variants = &e.variants;
            let mut fall_arm = variants.first();
            let mut impl_tokens = proc_macro2::TokenStream::new();
            for variant in variants {
                let this_attr = get_field_attr(variant.attrs.iter(), "fall");
                if this_attr.is_some() {
                    fall_arm = Some(variant);
                } else {
                    match &variant.fields {
                        Fields::Unnamed(fields) => {
                            let ty = &fields.unnamed.first().expect("无名枚举没有字段！").ty;
                            let ident = &variant.ident;
                            impl_tokens.extend(quote!(
                                if <#ty as jbuchong::GetClassTypeTrait>::is_this_type(&instance) {
                                    #name::#ident(
                                        <#ty>::try_from_instance(
                                            <#ty as jbuchong::GetClassTypeTrait>::cast_to_this_type(instance)
                                        )?
                                    )
                                } else
                            ))
                        }
                        Fields::Unit => {
                            eprintln!("该枚举值可能不会被使用。")
                        }
                        _ => {
                            panic!("不支持无内含值的枚举以及有名枚举！")
                        }
                    }
                }
            }
            if let Some(fall_arm) = fall_arm {
                match &fall_arm.fields {
                    Fields::Unnamed(fields) => {
                        let ty = &fields.unnamed.first().expect("无名枚举没有字段！").ty;
                        let fall_arm_ident = &fall_arm.ident;
                        impl_tokens.extend(quote!(
                            {#name::#fall_arm_ident(<#ty as jbuchong::TryFromInstanceTrait>::try_from_instance(instance)?)}
                        ));
                    }
                    Fields::Unit => {
                        let fall_arm_ident = &fall_arm.ident;
                        impl_tokens.extend(quote!(
                            {#name::#fall_arm_ident}
                        ));
                    }
                    _ => {
                        panic!("不支持无内含值的枚举以及有名枚举！")
                    }
                }
            } else {
                impl_tokens.extend(quote!({ panic!("意料之外的类型！") }))
            }
            quote!(Ok(#impl_tokens))
        }
        Data::Union(_) => panic!("不支持使用 `union`!"),
    };
    let (impl_generics, ty_generics, where_clause) = generics.split_for_impl();
    let gen = quote! {
        impl #impl_generics jbuchong::TryFromInstanceTrait for #name #ty_generics #where_clause {
            fn try_from_instance(instance: j4rs::Instance) -> Result<Self,j4rs::errors::J4RsError>{
                #impl_tokens
            }
        }
    };
    gen.into()
}

/// ### `java_type`
//
/// 为结构体、枚举等实现 [`GetClassTypeTrait`](jbuchong::GetClassTypeTrait).
///
/// 接受一个字符串字面值参数，类似于此：
///
/// ```not_test
/// use jbc_derive::java_type;
/// #[java_type("io.github.worksoup.JBuChongUtils")]
/// struct JBuChongUtils{}
/// ```
///
/// 对结构体或枚举等没有特殊要求。
///
/// 对于有泛型参数的结构体或枚举，可以使用如下语法：
/// ``` not_test
/// #[java_type("io.github.worksoup.JBuChongUtils", A = i32, B = i32)]
/// struct JBuChongUtils<A, B>{}
/// ```
/// 相当于：
/// ``` not_test
/// struct JBuChongUtils<A, B>{}
/// impl GetClassTypeTrait for JBuChongUtils<i32, i32>{}
/// ```
/// 不必指定全部的泛型参数。
#[proc_macro_attribute]
pub fn java_type(type_name_and_attr: TokenStream, input: TokenStream) -> TokenStream {
    let ast: &DeriveInput = &mut syn::parse(input).unwrap();
    let type_name_and_attr = type_name_and_attr.to_string();
    let mut type_name_and_attr = type_name_and_attr
        .split(',')
        .map(|s| s.trim())
        .collect::<Vec<_>>();
    let type_name = type_name_and_attr.first().expect("请指定 java 类型！");
    let type_name: &syn::LitStr = &syn::parse(type_name.parse().expect("类型名称请用字符串表示！"))
        .expect("类型名称请用字符串表示！");
    let name = &ast.ident;
    type_name_and_attr.remove(0);
    let mut generics = ast.generics.clone();
    let attrs = type_name_and_attr
        .into_iter()
        .map(|attr| {
            let attr = attr.split('=').map(|s| s.trim()).collect::<Vec<_>>();
            (
                *attr.first().expect("泛型参数指定格式错误！"),
                *attr.get(1).expect("泛型参数指定格式错误！"),
            )
        })
        .collect::<HashMap<_, _>>();
    generics.params = generics
        .params
        .clone()
        .into_iter()
        .filter(|p| match p {
            GenericParam::Lifetime(_) => true,
            GenericParam::Type(t) => !attrs.contains_key(t.ident.to_string().as_str()),
            GenericParam::Const(c) => !attrs.contains_key(c.ident.to_string().as_str()),
        })
        .collect();
    let (_, ty_generics, _) = ast.generics.split_for_impl();
    let ty_generics = ty_generics.to_token_stream().to_string();
    let ty_generics = ty_generics
        .trim()
        .trim_start_matches('<')
        .trim_end_matches('>')
        .split(',')
        .map(|s| {
            let s = s.trim();
            attrs.get(s).copied().unwrap_or(s)
        })
        .collect::<Vec<_>>()
        .join(",");
    let ty_generics: proc_macro2::TokenStream = if ty_generics.is_empty() {
        ty_generics
    } else {
        format!("<{ty_generics}>")
    }
        .parse()
        .unwrap();
    let (impl_generics, _, where_clause) = generics.split_for_impl();
    // let (impl_generics, ty_generics, where_clause) = ast.generics.split_for_impl();
    let gen = quote! {
        #ast
        impl #impl_generics jbuchong::GetClassTypeTrait for #name #ty_generics #where_clause {
            fn get_class_type() -> j4rs::Instance {
                j4rs::Jvm::attach_thread()
                    .unwrap()
                    .invoke_static(
                        "io.github.worksoup.JBuChongUtils",
                        "forName",
                        &[j4rs::InvocationArg::try_from(
                            Self::get_type_name(),
                        )
                        .unwrap()],
                    )
                    .unwrap()
            }
            fn cast_to_this_type(instance: j4rs::Instance) -> j4rs::Instance {
                let jvm = j4rs::Jvm::attach_thread()
                    .unwrap();
                jvm.cast(&instance, Self::get_type_name()).unwrap()
            }
            fn get_type_name() -> &'static str {
                #type_name
            }
            fn is_this_type(instance: &j4rs::Instance) -> bool {
                jbuchong::utils::is_instance_of(&instance, Self::get_type_name())
            }
        }
    };
    gen.into()
}

#[inline]
fn java_impl(type_name: TokenStream, input: TokenStream) -> proc_macro2::TokenStream {
    let ast: &DeriveInput = &syn::parse(input).unwrap();
    if type_name.is_empty() {
        quote! {
            #[derive(jbuchong::AsInstanceDerive, jbuchong::GetInstanceDerive, jbuchong::ToArgDerive, jbuchong::IntoArgDerive)]
            #ast
        }
    } else {
        let type_name: syn::LitStr = syn::parse(type_name).unwrap();
        quote! {
            #[derive(jbuchong::AsInstanceDerive, jbuchong::GetInstanceDerive, jbuchong::ToArgDerive, jbuchong::IntoArgDerive)]
            #[jbuchong::java_type(#type_name)]
            #ast
        }
    }
}
/// ### `java_all`
///
/// 同时应用 [`TryFromInstanceDerive`], 和 [`java`](macro@java).
#[proc_macro_attribute]
pub fn java_all(type_name: TokenStream, input: TokenStream) -> TokenStream {
    let gen = java_impl(type_name, input);
    let gen = quote! {
        #[derive(jbuchong::TryFromInstanceDerive)]
        #gen
    };
    gen.into()
}

/// ### `java`
///
/// 同时应用 [`GetInstanceDerive`], [`AsInstanceDerive`], [`ToArgDerive`], [`IntoArgDerive`] 和 [`java_type`](macro@java_type).
///
/// 接受一个字符串字面值参数传递给 `java_type` 属性。如不传入字符串，则不实现 `java_type` 属性。
#[proc_macro_attribute]
pub fn java(type_name: TokenStream, input: TokenStream) -> TokenStream {
    java_impl(type_name, input).into()
}
