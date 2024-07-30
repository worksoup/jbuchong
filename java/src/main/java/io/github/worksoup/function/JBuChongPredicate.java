package io.github.worksoup.function;

import java.util.function.Predicate;

public class JBuChongPredicate<T> implements Predicate<T> {
    @Override
    public boolean test(T item) {
        return this.function.apply(item);
    }

    public JBuChongPredicate(JBuChongFunction<T, Boolean> function) {
        this.function = function;
    }

    private final JBuChongFunction<T, Boolean> function;
}
