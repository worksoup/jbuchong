package io.github.worksoup.function

class JBuChongKtFunc0<out R>(private var func: JBuChongSupplier<R>) : Function0<R> {
    override fun invoke(): R {
        return func.get()
    }
}