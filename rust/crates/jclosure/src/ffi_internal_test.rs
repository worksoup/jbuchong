#[cfg(test)]
mod tests {
    // use std::cmp::Ordering;

    use crate::{
        bi_function::BiFunction, comparator::Comparator, consumer::Consumer, function::Function,
        predicate::Predicate, Func0, Func1, Func2, Func3, Supplier,
    };
    use j4rs::{errors::J4RsError, ClasspathEntry, Instance, InvocationArg, Jvm, JvmBuilder};
    use jbc_base::{self as jbuchong, GetInstanceTrait};
    use jbc_derive::TryFromInstanceDerive;
    use std::cmp::Ordering;

    //
    // use crate::{
    //     comparator::Comparator, consumer::Consumer, function::Function, kt_func_0::KtFunc0,
    //     kt_func_1::KtFunc1, kt_func_2::KtFunc2, predicate::Predicate,
    // };
    //
    #[derive(TryFromInstanceDerive)]
    struct X {
        instance: Instance,
    }
    impl GetInstanceTrait for X {
        fn get_instance(&self) -> Result<Instance, J4RsError> {
            let jvm = Jvm::attach_thread().unwrap();
            jvm.clone_instance(&self.instance)
        }
    }

    impl X {
        fn fuck(&self) -> String {
            let jvm = Jvm::attach_thread().unwrap();
            jvm.chain(&self.instance)
                .unwrap()
                .invoke("getClass", &[])
                .unwrap()
                .invoke("toString", &[])
                .unwrap()
                .to_rust()
                .unwrap()
        }
    }

    fn get_a_jvm_for_test() -> Jvm {
        JvmBuilder::new()
            .classpath_entry(ClasspathEntry::new(
                "../../../java/build/libs/jvm_side-1.0-SNAPSHOT-all.jar",
            ))
            .build()
            .unwrap_or_else(|_| Jvm::attach_thread().unwrap())
    }

    #[test]
    fn closure_to_bi_function_works() {
        let _jvm = get_a_jvm_for_test();
        let a = 2;
        let f = move |x: X, y: bool| -> Ordering {
            println!("a = {a}\nThe class name is `{}`.", x.fuck());
            if y {
                Ordering::Less
            } else {
                Ordering::Greater
            }
        };
        let function = BiFunction::new(f);
        let v1 = InvocationArg::try_from(true).unwrap();
        let v2 = InvocationArg::try_from(false).unwrap();
        let x = function.apply(v1, v2).unwrap();
        function.drop();
        println!("a = {a}\nThe result is `{:?}`.", x);
    }
    #[test]
    fn closure_to_consumer_works() {
        let _jvm = get_a_jvm_for_test();
        let a = 2;
        let f = move |x: X| {
            println!("a = {a}\nThe class name is `{}`.", x.fuck());
        };
        let consumer = Consumer::new(f);
        let test_instance = InvocationArg::try_from(true).unwrap();
        consumer.accept(test_instance).unwrap();
        consumer.drop();
        let f = move |x: bool| {
            println!("x = {x}.");
        };
        let consumer = Consumer::new(f);
        consumer.call(false);
        consumer.drop();
    }

    #[test]
    fn closure_to_comparator_works() {
        let _jvm = get_a_jvm_for_test();
        let a = 2;
        let f = move |x1: X, x2: X| -> Ordering {
            let jvm = Jvm::attach_thread().unwrap(); // jvm 不能直接捕获，否则会卡死或崩溃。
            let x1 = x1.get_instance().unwrap();
            let x2 = x2.get_instance().unwrap();
            let val1: i32 = jvm.to_rust(x1).unwrap();
            let val2: i32 = jvm.to_rust(x2).unwrap();
            val1.cmp(&val2)
        };
        let comparator = Comparator::new(f);
        let test_instance1 = InvocationArg::try_from(22).unwrap_or_else(|err| panic!("{}", err));
        let test_instance2 = InvocationArg::try_from(55).unwrap();
        let x = comparator.compare(test_instance1, test_instance2);
        comparator.drop();
        println!("a = {a}\nThe ordering is `{:?}`.", x);
    }
    #[test]
    fn closure_to_function_works() {
        let _jvm = get_a_jvm_for_test();
        let a = 2;
        let f = move |x: X| -> X {
            println!("a = {a}\nThe class name is `{}`.", x.fuck());
            x
        };
        let function = Function::new(f);
        let test_instance = InvocationArg::try_from(true).unwrap();
        let x = function.apply(test_instance).unwrap();
        println!("a = {a}\nThe class name is `{}`.", x.fuck());
        let y = function.call(x);
        println!("a = {a}\nThe class name is `{}`.", y.fuck());
        function.drop();
    }

    #[test]
    fn closure_to_predicate_works() {
        let _jvm = get_a_jvm_for_test();
        let a = 2;
        let f = move |x1: X| -> bool {
            let jvm = Jvm::attach_thread().unwrap(); // jvm不能直接捕获，否则会卡死。
            let val1: i32 = jvm.to_rust(x1.get_instance().unwrap()).unwrap();
            val1 > 0
        };
        let predicate = Predicate::new(f);
        // println!("sleep");
        // sleep(std::time::Duration::from_millis(10000));
        let test_value = InvocationArg::try_from(22).unwrap_or_else(|err| panic!("{}", err));
        let x = predicate.test(test_value);
        predicate.drop();
        println!("a = {a}\n And `test_value > 0` is `{:?}`.", x);
    }

    #[test]
    fn closure_to_supplier_works() {
        let _jvm = get_a_jvm_for_test();
        let a = 2;
        let f = move || -> bool {
            println!("Supplier called. a = {}.", a);
            true
        };
        let supplier = Supplier::new(f);
        let x = supplier.get().unwrap();
        supplier.drop();
        println!("Result is {}", x);
    }

    #[test]
    fn closure_to_func0_works() {
        let _jvm = get_a_jvm_for_test();
        let a = 2;
        let f = move || -> bool {
            println!("Supplier called. a = {}.", a);
            true
        };
        let func0 = Func0::new(f);
        let x = func0.invoke().unwrap();
        func0.drop();
        println!("Result is {}", x);
    }
    #[test]
    fn closure_to_func1_works() {
        let _jvm = get_a_jvm_for_test();
        let a = 2;
        let f = move |x: X| -> X {
            println!("a = {a}\nThe class name is `{}`.", x.fuck());
            x
        };
        let function = Func1::new(f);
        let test_instance = InvocationArg::try_from(true).unwrap();
        let x = function.invoke(test_instance).unwrap();
        function.drop();
        println!("a = {a}\nThe class name is `{}`.", x.fuck());
    }
    #[test]
    fn closure_to_func2_works() {
        let _jvm = get_a_jvm_for_test();
        let f = move |a: bool, b: bool| -> bool {
            let r = a | b;
            println!("a = {a}\nThe result is `{}`.", r);
            r
        };
        let function = Func2::new(f);
        let a = InvocationArg::try_from(false).unwrap();
        let b = InvocationArg::try_from(true).unwrap();
        let r = function.invoke(a, b).unwrap();
        function.drop();
        println!("The result is `{}`.", r);
    }
    #[test]
    fn closure_to_func3_works() {
        let _jvm = get_a_jvm_for_test();
        let f = move |a: bool, b: bool, c: bool| -> bool {
            let r = a | b | c;
            println!("a = {a}\nThe result is `{}`.", r);
            r
        };
        let function = Func3::new(f);
        let a = InvocationArg::try_from(false).unwrap();
        let b = InvocationArg::try_from(false).unwrap();
        let c = InvocationArg::try_from(true).unwrap();
        let r = function.invoke(a, b, c).unwrap();
        function.drop();
        println!("The result is `{}`.", r);
    }
    //
    // #[test]
    // fn closure_to_kt_func_1_works() {
    //     let _jvm = get_a_jvm_for_test();
    //     let a = 2;
    //     let f = |x: X| -> X {
    //         println!("a = {a}\nThe class name is `{}`.", x.fuck());
    //         x
    //     };
    //     let kt_func_1 = KtFunc1::new(&f);
    //     let test_instance = InvocationArg::try_from(true).unwrap();
    //     let x = kt_func_1.invoke(test_instance);
    //     let _ = kt_func_1.drop_and_to_raw();
    //     println!("a = {a}\nThe class name is `{}`.", x.fuck());
    // }
    //
    // #[test]
    // fn closure_to_kt_func_2_works() {
    //     let top_jvm = get_a_jvm_for_test();
    //     let a = 2;
    //     let f = move |x1: X, x2: X| -> X {
    //         let jvm = Jvm::attach_thread().unwrap(); // jvm 不能直接捕获，否则会卡死或崩溃。
    //         let x1 = x1.get_instance();
    //         let x2 = x2.get_instance();
    //         let val1: i32 = jvm.to_rust(x1).unwrap();
    //         let val2: i32 = jvm.to_rust(x2).unwrap();
    //         let b = InvocationArg::try_from(val1 - val2)
    //             .unwrap()
    //             .into_primitive()
    //             .unwrap();
    //         let instance = jvm.create_instance("java.lang.Integer", &[b]).unwrap(); // 需要通过参数对象创建对象，不能直接 Instance::try_from, 否则会出错。
    //         X { instance }
    //     };
    //     let kt_func_2 = KtFunc2::new(&f);
    //     let test_instance1 = InvocationArg::try_from(22).unwrap_or_else(|err| panic!("{}", err));
    //     let test_instance2 = InvocationArg::try_from(55).unwrap();
    //     let x = kt_func_2.invoke(test_instance1, test_instance2);
    //     let _ = kt_func_2.drop_and_to_raw();
    //     println!(
    //         "a = {a}\nThe ordering is `{:?}`.",
    //         top_jvm.to_rust::<i32>(x.get_instance()).unwrap()
    //     );
    // }

    fn gen(pc: usize) -> String {
        let type_params = &"ABCDEFGHIJKLMNOP"[0..pc];
        let type_params_1 = type_params
            .chars()
            .map(|c| format!("in {c}"))
            .collect::<Vec<_>>()
            .join(", ");
        let type_params_2 = [
            vec!["Pair<A, B>".to_string()],
            "ABCDEFGHIJKLMNOP"[2..pc]
                .chars()
                .map(|c| c.to_string())
                .collect::<Vec<_>>(),
        ]
        .concat()
        .join(", ");
        let type_params_3 = "ABCDEFGHIJKLMNOP"[0..pc]
            .chars()
            .map(|c| c.to_string())
            .collect::<Vec<_>>()
            .join(", ");
        let type_params_4 = type_params
            .chars()
            .enumerate()
            .map(|(pc, c)| format!("p{pc}:{c}"))
            .collect::<Vec<_>>()
            .join(", ");
        let type_params_5 = [
            vec!["Pair(p0, p1)".to_string()],
            (2..pc).map(|c| format!("p{c}")).collect::<Vec<_>>(),
        ]
        .concat()
        .join(", ");
        format!(
            r#"package io.github.worksoup.function

class JBuChongKtFunc{}<{}, out R>(private var func: JBuChongKtFunc{}<{}, R>) :
    Function{}<{}, R> {{
    override fun invoke({}): R {{
        return func({})
    }}
}}"#,
            pc,
            type_params_1,
            pc - 1,
            type_params_2,
            pc,
            type_params_3,
            type_params_4,
            type_params_5
        )
    }
    #[test]
    fn gen_jvm_side_kt_func() {
        for pc in 3..=16 {
            println!("{}", gen(pc));
        }
    }
}
