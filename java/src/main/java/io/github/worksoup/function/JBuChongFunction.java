package io.github.worksoup.function;

import org.astonbitecode.j4rs.api.Instance;
import org.astonbitecode.j4rs.api.java2rust.Java2RustUtils;

import java.util.Arrays;
import java.util.List;
import java.util.function.Function;

public final class JBuChongFunction<T, R> implements Function<T, R> {
    @Override
    public R apply(T arg) {
        System.out.println("JBuChongFunction");
        System.out.println(Arrays.toString(this.rustFunction));
        var rustFunctionAsByteList = Arrays.stream(this.rustFunction).toList();
        var instance = nativeApply(Java2RustUtils.createInstance(rustFunctionAsByteList), Java2RustUtils.createInstance(arg));
        return Java2RustUtils.getObjectCasted(instance);
    }

    public JBuChongFunction(Byte[] rustFunction) {
        this.rustFunction = rustFunction;
    }

    private native Instance<R> nativeApply(Instance<List<Byte>> rustFunctionInstance, Instance<T> arg);

    static {
        System.loadLibrary("jclosure");
    }

    private final Byte[] rustFunction;
}
