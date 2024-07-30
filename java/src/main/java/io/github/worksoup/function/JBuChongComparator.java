package io.github.worksoup.function;

import java.util.Comparator;

public final class JBuChongComparator<T> implements Comparator<T> {
    @Override
    public int compare(T val1, T val2) {
        return this.biFunction.apply(val1, val2);
    }

    public JBuChongComparator(JBuChongBiFunction<T, T, Byte> biFunction) {
        this.biFunction = biFunction;
    }

    private final JBuChongBiFunction<T, T, Byte> biFunction;
}
