package io.github.worksoup.function;

import org.astonbitecode.j4rs.api.Instance;
import org.astonbitecode.j4rs.api.java2rust.Java2RustUtils;

import java.util.Arrays;
import java.util.List;
import java.util.function.Consumer;

public final class JBuChongConsumer<T> implements Consumer<T> {
    @Override
    public void accept(T e) {
        System.out.println("JBuChongConsumer");
        System.out.println(Arrays.toString(this.rustConsumer));
        var rustConsumerAsByteList = Arrays.stream(this.rustConsumer).toList();
        nativeAccept(Java2RustUtils.createInstance(rustConsumerAsByteList), Java2RustUtils.createInstance(e));
    }

    public JBuChongConsumer(Byte[] rustConsumer) {
        this.rustConsumer = rustConsumer;
    }

    private native void nativeAccept(Instance<List<Byte>> rustConsumerInstance, Instance<T> arg);

    static {
        System.loadLibrary("jclosure");
    }

    private final Byte[] rustConsumer;
}
