use super::tuple_combine::*;

pub trait TupleFlatten {
    type Flattened;

    fn flatten(self) -> Self::Flattened;
}

impl<T0, T1> TupleFlatten for (T0, T1)
where (T0, T1): TupleJoin {
    type Flattened = <(T0, T1) as TupleJoin>::Joined;
    fn flatten(self) -> Self::Flattened {
        self.join()
    }
}

impl<T0, T1, T2> TupleFlatten for (T0, T1, T2)
where
(T0, T1): TupleJoin,
(<(T0, T1) as TupleJoin>::Joined, T2): TupleJoin {
    type Flattened = <(<(T0, T1) as TupleJoin>::Joined, T2) as TupleJoin>::Joined;

    fn flatten(self) -> Self::Flattened {
        let (t0, t1, t2) = self;
        ((t0, t1).join(), t2).join()
    }
}

impl<T0, T1, T2, T3> TupleFlatten for (T0, T1, T2, T3)
where
(T0, T1): TupleJoin,
(T2, T3): TupleJoin,
(
    <(T0, T1) as TupleJoin>::Joined,
    <(T2, T3) as TupleJoin>::Joined
): TupleJoin {
    type Flattened = <(
        <(T0, T1) as TupleJoin>::Joined,
        <(T2, T3) as TupleJoin>::Joined
    ) as TupleJoin>::Joined;

    fn flatten(self) -> Self::Flattened {
        let (t0, t1, t2, t3) = self;
        ((t0, t1).join(), (t2, t3).join()).join()
    }
}

impl<T0, T1, T2, T3, T4> TupleFlatten for (T0, T1, T2, T3, T4)
where
(T0, T1, T2, T3): TupleFlatten,
(
    <(T0, T1, T2, T3) as TupleFlatten>::Flattened,
    T4
): TupleJoin {
    type Flattened = <(
        <(T0, T1, T2, T3) as TupleFlatten>::Flattened,
        T4
    ) as TupleJoin>::Joined;

    fn flatten(self) -> Self::Flattened {
        let (t0, t1, t2, t3, t4) = self;
        (
            (t0, t1, t2, t3).flatten(),
            t4
        ).join()
    }
}

impl<T0, T1, T2, T3, T4, T5> TupleFlatten for (T0, T1, T2, T3, T4, T5)
where
(T0, T1, T2, T3): TupleFlatten,
(T4, T5): TupleJoin,
(
    <(T0, T1, T2, T3) as TupleFlatten>::Flattened,
    <(T4, T5) as TupleJoin>::Joined
): TupleJoin {
    type Flattened = <(
        <(T0, T1, T2, T3) as TupleFlatten>::Flattened,
        <(T4, T5) as TupleJoin>::Joined
    ) as TupleJoin>::Joined;

    fn flatten(self) -> Self::Flattened {
        let (t0, t1, t2, t3, t4, t5) = self;
        (
            (t0, t1, t2, t3).flatten(),
            (t4, t5).join()
        ).join()
    }
}

impl<T0, T1, T2, T3, T4, T5, T6> TupleFlatten for (T0, T1, T2, T3, T4, T5, T6)
where
(T0, T1, T2, T3): TupleFlatten,
(T4, T5, T6): TupleFlatten,
(
    <(T0, T1, T2, T3) as TupleFlatten>::Flattened,
    <(T4, T5, T6) as TupleFlatten>::Flattened,
): TupleJoin {
    type Flattened = <(
        <(T0, T1, T2, T3) as TupleFlatten>::Flattened,
        <(T4, T5, T6) as TupleFlatten>::Flattened
    ) as TupleJoin>::Joined;

    fn flatten(self) -> Self::Flattened {
        let (t0, t1, t2, t3, t4, t5, t6) = self;
        (
            (t0, t1, t2, t3).flatten(),
            (t4, t5, t6).flatten()
        ).join()
    }
}

impl<T0, T1, T2, T3, T4, T5, T6, T7> TupleFlatten for (T0, T1, T2, T3, T4, T5, T6, T7)
where
(T0, T1, T2, T3): TupleFlatten,
(T4, T5, T6, T7): TupleFlatten,
(
    <(T0, T1, T2, T3) as TupleFlatten>::Flattened,
    <(T4, T5, T6, T7) as TupleFlatten>::Flattened,
): TupleJoin {
    type Flattened = <(
        <(T0, T1, T2, T3) as TupleFlatten>::Flattened,
        <(T4, T5, T6, T7) as TupleFlatten>::Flattened
    ) as TupleJoin>::Joined;

    fn flatten(self) -> Self::Flattened {
        let (t0, t1, t2, t3, t4, t5, t6, t7) = self;
        (
            (t0, t1, t2, t3).flatten(),
            (t4, t5, t6, t7).flatten()
        ).join()
    }
}