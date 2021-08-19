pub trait TypeListOp<T> {
    type OutAppend;
    type OutPrepend;

    fn append(self, t: T) -> Self::OutAppend;
    fn prepend(self, t: T) -> Self::OutPrepend;
}

impl<T> TypeListOp<T> for () {
    type OutAppend = (T,);
    type OutPrepend = (T,);

    fn append(self, t: T) -> Self::OutAppend {
        (t,)
    }
    
    fn prepend(self, t: T) -> Self::OutPrepend {
        (t,)
    }
}

macro_rules! impl_to_tuple {
    ( () ) => {};
    ( ( $t0:ident $(, $types:ident)* ) ) => {
        #[allow(unused_parens)]
        impl<$t0, $($types,)* T0> TypeListOp<T0> for ($t0, $($types,)*)
        {
            type OutAppend = ($t0, $($types,)* T0);
            type OutPrepend = (T0, $t0, $($types,)*);

            fn append(self, t: T0) -> Self::OutAppend {
                #[allow(non_snake_case)]
                let ($t0, $($types,)*) = self;
                ($t0, $($types,)* t)
            }

            fn prepend(self, t: T0) -> Self::OutPrepend {
                #[allow(non_snake_case)]
                let ($t0, $($types,)*) = self;
                (t, $t0, $($types,)*)
            }
        }

        impl_to_tuple! { ($($types),*) }
    };
}

impl_to_tuple!((A, B, C, D, E, F, G, H, I, J, K, L, M, N, O, P, Q, R, S, T, U, V, W, X, Y, Z));

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn prepend() {
        assert_eq!((1,), ().prepend(1));
        assert_eq!((2, 1, 0), (0,).prepend(1).prepend(2));
        assert_eq!((3, 2, 1), ().prepend(1).prepend(2).prepend(3));
    }

    #[test]
    fn append() {
        assert_eq!((), ());
        assert_eq!((1,), ().append(1));
        assert_eq!((1, 2), ().append(1).append(2));
        assert_eq!((1, 2, 3), ().append(1).append(2).append(3));
    }

    #[test]
    fn append_prepend() {
        assert_eq!((1,), ().prepend(1));
        assert_eq!((1, 2), ().prepend(1).append(2));
        assert_eq!((3, 1, 2), ().prepend(1).append(2).prepend(3));
        assert_eq!((3, 1, 2, 4), ().prepend(1).append(2).prepend(3).append(4));
    }
}
