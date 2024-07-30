package io.github.worksoup.function

class JBuChongKtFunc1<in T, out R>(private var func: JBuChongFunction<T, R>) : Function1<T, R> {
    override fun invoke(p1: T): R {
        return func.apply(p1)
    }
}