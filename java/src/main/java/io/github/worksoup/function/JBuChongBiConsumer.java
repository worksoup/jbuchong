package io.github.worksoup.function;

import io.github.worksoup.JBuChongPair;

import java.util.function.BiConsumer;

public class JBuChongBiConsumer<T, U> implements BiConsumer<T, U> {
    @Override
    public void accept(T t, U u) {
        this.consumer.accept(new JBuChongPair<>(t, u));
    }

    public JBuChongBiConsumer(JBuChongConsumer<JBuChongPair<T, U>> consumer) {
        this.consumer = consumer;
    }

    private final JBuChongConsumer<JBuChongPair<T, U>> consumer;
}
