package io.github.worksoup.function;

import io.github.worksoup.JBuChongPair;

import java.util.function.BiFunction;

public class JBuChongBiFunction<T, U, R> implements BiFunction<T, U, R> {
    @Override
    public R apply(T t, U u) {
        return this.function.apply(new JBuChongPair<>(t, u));
    }

    public JBuChongBiFunction(JBuChongFunction<JBuChongPair<T, U>, R> function) {
        this.function = function;
    }

    private final JBuChongFunction<JBuChongPair<T, U>, R> function;
}
