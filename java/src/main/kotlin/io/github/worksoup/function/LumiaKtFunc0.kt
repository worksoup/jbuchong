package io.github.worksoup.function

class LumiaKtFunc0<out R>(private var func: LumiaSupplier<R>) : Function0<R> {
    override fun invoke(): R {
        return func.get()
    }
}