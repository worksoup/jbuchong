package io.github.worksoup.function;

import org.astonbitecode.j4rs.api.Instance;
import org.astonbitecode.j4rs.api.java2rust.Java2RustUtils;

import java.util.Arrays;
import java.util.List;
import java.util.function.Supplier;

public class JBuChongSupplier<T> implements Supplier<T> {
    @Override
    public T get() {
        System.out.println("LumiaSupplier");
        System.out.println(Arrays.toString(this.rustSupplier));
        var rustSupplierAsByteList = Arrays.stream(this.rustSupplier).toList();
        var instance = nativeGet(Java2RustUtils.createInstance(rustSupplierAsByteList));
        return Java2RustUtils.getObjectCasted(instance);
    }

    public JBuChongSupplier(Byte[] rustSupplier) {
        this.rustSupplier = rustSupplier;
    }

    private native Instance<T> nativeGet(Instance<List<Byte>> rustConsumerInstance);

    static {
        System.loadLibrary("jclosure");
    }

    private final Byte[] rustSupplier;
}
