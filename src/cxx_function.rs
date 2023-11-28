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

impl<I, Out> CxxFunction<I, fn() -> Out>
where
    I: CxxFunctionImpl<Out, ()>,
{
    /// Run the std::function
    pub fn invoke(&mut self) -> Out {
        <I as CxxFunctionImpl<Out, ()>>::__invoke(self as *mut _ as _, ())
    }
}

impl<I, In, Out> CxxFunction<I, fn(In) -> Out>
where
    I: CxxFunctionImpl<Out, (In,)>,
{
    /// Run the std::function
    pub fn invoke(&mut self, a: In) -> Out {
        <I as CxxFunctionImpl<Out, (In,)>>::__invoke(self as *mut _ as _, (a,))
    }
}
