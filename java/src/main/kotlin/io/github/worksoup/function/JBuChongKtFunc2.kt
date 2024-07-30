package io.github.worksoup.function

class JBuChongKtFunc2<in T, in U, out R>(private var func: JBuChongBiFunction<T, U, R>) : Function2<T, U, R> {
    override fun invoke(t: T, u: U): R {
        return func.apply(t, u)
    }
}