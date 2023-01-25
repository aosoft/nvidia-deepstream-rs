pub mod bounding_box;
pub mod mem;

#[cfg(feature = "helper")]
pub mod helper;

#[cfg(feature = "infer")]
pub mod infer;

#[cfg(feature = "logger")]
pub mod logger;

#[cfg(feature = "meta")]
pub mod meta;

#[cfg(feature = "obj_encode")]
pub mod obj_encode;

#[cfg(feature = "surface")]
pub mod surface;

#[cfg(feature = "surface_transform")]
pub mod surface_transform;

#[cfg(feature = "utils")]
pub mod utils;

#[cfg(feature = "yaml")]
pub mod yaml;



pub trait WrapperExt {
    type NativeType;

    fn as_native_type(&self) -> Self::NativeType;

    fn from_native_type_ref(n: &Self::NativeType) -> &Self;
    fn as_native_type_ref(&self) -> &Self::NativeType;

    fn from_native_type_mut(n: &mut Self::NativeType) -> &mut Self;
    fn as_native_type_mut(&mut self) -> &mut Self::NativeType;

    unsafe fn as_native_type_ptr(&self) -> *mut Self::NativeType;
}

#[macro_export(local_inner_macros)]
macro_rules! wrapper_impl_body {
    () => {
            #[inline]
            fn as_native_type(&self) -> Self::NativeType {
                self.0
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

            #[inline]
            unsafe fn as_native_type_ptr(&self) -> *mut Self::NativeType {
                self.as_native_type_ref() as *const _ as *mut _
            }
    }
}

#[macro_export(local_inner_macros)]
macro_rules! wrapper_impl {
    ($W:ident, $N:ty) => {
        #[derive(Copy, Clone)]
        pub struct $W($N);

        impl $W {
            #[inline]
            #[allow(dead_code)]
            fn from_native_type(n: $N) -> Self {
                Self(n)
            }
        }

        impl crate::WrapperExt for $W {
            type NativeType = $N;
            crate::wrapper_impl_body!();
        }

        impl Default for $W {
            fn default() -> Self {
                unsafe { std::mem::zeroed::<$W>() }
            }
        }
    };
}

#[macro_export(local_inner_macros)]
macro_rules! wrapper_impl_with_lifetime_body {
    ($W:ident, $N:ty) => {
        impl<'a> crate::WrapperExt for $W<'a> {
            type NativeType = $N;
            crate::wrapper_impl_body!();
        }

        impl<'a> Default for $W<'a> {
            fn default() -> Self {
                unsafe { std::mem::zeroed::<$W<'a>>() }
            }
        }
    };
}

#[macro_export(local_inner_macros)]
macro_rules! wrapper_impl_with_lifetime {
    ($W:ident, $N:ty) => {
        #[derive(Copy, Clone)]
        pub struct $W<'a>($N, core::marker::PhantomData<&'a $N>);

        impl<'a> crate::WrapperExt for $W<'a> {
            type NativeType = $N;
            crate::wrapper_impl_body!();
        }

        impl<'a> Default for $W<'a> {
            fn default() -> Self {
                unsafe { std::mem::zeroed::<$W<'a>>() }
            }
        }
    }
}
