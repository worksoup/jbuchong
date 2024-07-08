package io.github.worksoup.function;

import io.github.worksoup.LumiaPair;

import java.util.function.BiConsumer;

public class LumiaBiConsumer<T, U> implements BiConsumer<T, U> {
    @Override
    public void accept(T t, U u) {
        this.consumer.accept(new LumiaPair<>(t, u));
    }

    public LumiaBiConsumer(LumiaConsumer<LumiaPair<T, U>> consumer) {
        this.consumer = consumer;
    }

    private final LumiaConsumer<LumiaPair<T, U>> consumer;
}
