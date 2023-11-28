use core::{ffi::c_void, marker::PhantomData};

///
pub unsafe trait CxxFunctionImpl<Ret, Args> {
    #[doc(hidden)]
    fn __invoke(f: *mut c_void, a: Args) -> Ret;
}

/// Bridge to std::function<Fn>
#[repr(C)]
pub struct CxxFunction<I: ?Sized, F> {
    // A thing, because repr(C) structs are not allowed to consist exclusively
    // of PhantomData fields.
    _void: [c_void; 0],
    _impl: PhantomData<I>,
    _fn: PhantomData<F>,
}

macro_rules! impl_invoke {
    ($($name:ident : $ty:ident),*) => {
        impl<I, Out, $($ty,)*> CxxFunction<I, fn($($name: $ty,)*) -> Out>
        where
            I: CxxFunctionImpl<Out, ($($ty,)*)>,
        {
            /// Run the std::function
            pub fn invoke(&mut self, $($name : $ty,)*) -> Out {
                let args = ($($name,)*);
                <I as CxxFunctionImpl<Out, ($($ty,)*),>>::__invoke(self as *mut _ as _, args)
            }
        }
    };
}

impl_invoke! {}
impl_invoke! {a_0: A0}
impl_invoke! {a_0: A0, a_1: A1}
impl_invoke! {a_0: A0, a_1: A1, a_2: A2}
impl_invoke! {a_0: A0, a_1: A1, a_2: A2, a_3: A3}
impl_invoke! {a_0: A0, a_1: A1, a_2: A2, a_3: A3, a_4: A4}
impl_invoke! {a_0: A0, a_1: A1, a_2: A2, a_3: A3, a_4: A4, a_5: A5}
impl_invoke! {a_0: A0, a_1: A1, a_2: A2, a_3: A3, a_4: A4, a_5: A5, a_6: A6}
impl_invoke! {a_0: A0, a_1: A1, a_2: A2, a_3: A3, a_4: A4, a_5: A5, a_6: A6, a_7: A7}
impl_invoke! {a_0: A0, a_1: A1, a_2: A2, a_3: A3, a_4: A4, a_5: A5, a_6: A6, a_7: A7, a_8: A8}
impl_invoke! {a_0: A0, a_1: A1, a_2: A2, a_3: A3, a_4: A4, a_5: A5, a_6: A6, a_7: A7, a_8: A8, a_9: A9}
