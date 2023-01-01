pub mod bounding_box;
pub mod buffer;
pub mod element;
pub mod meta;
pub mod osd;

pub trait WrapperExt {
    type NativeType;

    fn from_native_type(n: Self::NativeType) -> Self;
    fn from_native_type_ref(n: &Self::NativeType) -> &Self;
    fn as_native_type_ref(&self) -> &Self::NativeType;

    fn from_native_type_mut(n: &mut Self::NativeType) -> &mut Self;
    fn as_native_type_mut(&mut self) -> &mut Self::NativeType;
}

#[macro_export(local_inner_macros)]
macro_rules! wrapper_impl {
    ($W:ident, $N:ty) => {
        #[derive(Debug, Copy, Clone)]
        pub struct $W($N);

        impl crate::WrapperExt for $W {
            type NativeType = $N;

            #[inline]
            fn from_native_type(n: Self::NativeType) -> Self {
                Self(n)
            }

            #[inline]
            fn from_native_type_ref(n: &Self::NativeType) -> &Self {
                unsafe { std::mem::transmute(n) }
            }

            #[inline]
            fn as_native_type_ref(&self) -> &Self::NativeType {
                &self.0
            }

            #[inline]
            fn from_native_type_mut(n: &mut Self::NativeType) -> &mut Self {
                unsafe { std::mem::transmute(n) }
            }

            #[inline]
            fn as_native_type_mut(&mut self) -> &mut Self::NativeType {
                &mut self.0
            }
        }
    };
}
