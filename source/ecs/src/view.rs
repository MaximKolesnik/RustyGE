use std::marker::PhantomData;

pub trait Viewable {
    fn get_component_types() -> Vec<std::any::TypeId>;
}

pub struct View<V: Viewable> {
    _view: PhantomData<V>,
    component_types: Vec<std::any::TypeId>,
}

impl<V: Viewable> View<V> {
    fn new() -> Self {
        Self {
            _view: PhantomData,
            component_types: V::get_component_types(),
        }
    }

    pub fn test(&self) {
        for id in V::get_component_types().iter() {
            println!("View begin");
            println!("{:?}", id);
            println!("View end");
        }
    }
}

pub trait IntoView: Viewable + Sized {
    fn create_view() -> View<Self>;
}

impl<T: Viewable> IntoView for T {
    fn create_view() -> View<Self> {
        View::new()
    }
}

pub trait ViewTuple {

}

macro_rules! view_tuple {
    ($head_ty:ident) => {
        impl_view_tuple!($head_ty);
    };
    ($head_ty:ident, $( $tail_ty:ident ),*) => (
        impl_view_tuple!($head_ty, $( $tail_ty ),*);
        view_tuple!($( $tail_ty ),*);
    );
}

macro_rules! impl_view_tuple {
    ( $( $ty: ident ),+ ) => {
        impl<$($ty),*> ViewTuple for ($( $ty, )+) {
        }
    }
}

view_tuple!(A, B, C, D, E, F, G, H, I, J, K, L, M, N, O, P, Q, R, S, T, U, V, W, X, Y, Z);

macro_rules! viewable_tuple {
    ($head_ty:ident) => {
        impl_viewable_tuple!($head_ty);
    };
    ($head_ty:ident, $( $tail_ty:ident ),*) => (
        impl_viewable_tuple!($head_ty, $( $tail_ty ),*);
        viewable_tuple!($( $tail_ty ),*);
    );
}

macro_rules! impl_viewable_tuple {
    ($ty: ident) => {
        #[allow(unused_parens)]
        impl<$ty: crate::Component + 'static> Viewable for ($ty) {
            fn get_component_types() -> Vec<std::any::TypeId> {
                vec!(std::any::TypeId::of::<$ty>())
            }
        }
    };
    ( $( $ty: ident ),+ ) => {
        impl<$( $ty: crate::Component + 'static),*> Viewable for ($( $ty, )+) {
            fn get_component_types() -> Vec<std::any::TypeId> {
                vec!($(std::any::TypeId::of::<$ty>(),)*)
            }
        }
    }
}

viewable_tuple!(A, B, C, D, E, F, G, H, I, J, K, L, M, N, O, P, Q, R, S, T, U, V, W, X, Y, Z);
