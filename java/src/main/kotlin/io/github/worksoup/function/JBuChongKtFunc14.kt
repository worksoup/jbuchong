package io.github.worksoup.function

class JBuChongKtFunc14<in A, in B, in C, in D, in E, in F, in G, in H, in I, in J, in K, in L, in M, in N, out R>(private var func: JBuChongKtFunc13<Pair<A, B>, C, D, E, F, G, H, I, J, K, L, M, N, R>) :
    Function14<A, B, C, D, E, F, G, H, I, J, K, L, M, N, R> {
    override fun invoke(
        p0: A,
        p1: B,
        p2: C,
        p3: D,
        p4: E,
        p5: F,
        p6: G,
        p7: H,
        p8: I,
        p9: J,
        p10: K,
        p11: L,
        p12: M,
        p13: N
    ): R {
        return func(Pair(p0, p1), p2, p3, p4, p5, p6, p7, p8, p9, p10, p11, p12, p13)
    }
}
