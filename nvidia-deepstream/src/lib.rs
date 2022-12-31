pub mod bounding_box;
pub mod element;
pub mod meta;
pub mod osd;

pub trait Wrapper {
    type NativeType;

    fn from_native_type(n: Self::NativeType) -> Self;
    fn from_native_type_ref(n: &Self::NativeType) -> &Self;
    fn as_native_type_ref(&self) -> &Self::NativeType;
}

#[macro_export(local_inner_macros)]
macro_rules! wrapper_impl {
    ($W:ident, $N:ty) => {
        pub struct $W($N);

        impl crate::Wrapper for $W {
            type NativeType = $N;

            #[inline]
            fn from_native_type(n: Self::NativeType) -> Self { Self(n) }

            #[inline]
            fn from_native_type_ref(n: &Self::NativeType) -> &Self { unsafe { std::mem::transmute(n) } }

            #[inline]
            fn as_native_type_ref(&self) -> &Self::NativeType { &self.0 }
        }
    }
}
