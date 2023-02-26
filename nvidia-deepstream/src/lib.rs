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

    unsafe fn from_native_type_ptr(n: *mut Self::NativeType) -> *mut Self;
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
        unsafe fn from_native_type_ptr(n: *mut Self::NativeType) -> *mut Self {
            unsafe { std::mem::transmute(n) }
        }

        #[inline]
        unsafe fn as_native_type_ptr(&self) -> *mut Self::NativeType {
            self.as_native_type_ref() as *const _ as *mut _
        }
    };
}

#[macro_export(local_inner_macros)]
macro_rules! wrapper_impl_base {
    ($W:ident, $N:ty) => {
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
                unsafe { std::mem::zeroed::<Self>() }
            }
        }
    };
}

#[macro_export(local_inner_macros)]
macro_rules! wrapper_impl_ref_type {
    ($W:ident, $N:ty) => {
        pub struct $W($N);

        wrapper_impl_base!($W, $N);
    };
}

#[macro_export(local_inner_macros)]
macro_rules! wrapper_impl_value_type {
    ($W:ident, $N:ty) => {
        #[derive(Clone, Copy, Debug)]
        pub struct $W($N);

        wrapper_impl_base!($W, $N);
    };
}

#[macro_export(local_inner_macros)]
macro_rules! wrapper_impl_with_lifetime_base {
    ($W:ident, $N:ty) => {
        impl<'a> crate::WrapperExt for $W<'a> {
            type NativeType = $N;
            crate::wrapper_impl_body!();
        }

        impl<'a> Default for $W<'a> {
            fn default() -> Self {
                unsafe { std::mem::zeroed::<Self>() }
            }
        }
    };
}

#[macro_export(local_inner_macros)]
macro_rules! wrapper_impl_ref_type_with_lifetime {
    ($W:ident, $N:ty) => {
        pub struct $W<'a>($N, core::marker::PhantomData<&'a $N>);

        wrapper_impl_with_lifetime_base!($W, $N);
    };
}

#[macro_export(local_inner_macros)]
macro_rules! wrapper_impl_value_type_with_attr_lifetime {
    ($W:ident, $N:ty) => {
        #[derive(Clone, Copy, Debug)]
        pub struct $W<'a>($N, core::marker::PhantomData<&'a $N>);

        wrapper_impl_with_lifetime_base!($W, $N);
    };
}

pub(crate) unsafe fn duplicate_glib_string(
    src: *const nvidia_deepstream_sys::gchar,
) -> *mut nvidia_deepstream_sys::gchar {
    if src != std::ptr::null() {
        nvidia_deepstream_sys::g_strdup(src)
    } else {
        std::ptr::null_mut()
    }
}

pub(crate) unsafe fn glib_free<T>(p: *mut T) {
    if p != std::ptr::null_mut() {
        nvidia_deepstream_sys::g_free(p as _);
    }
}

pub struct Version {
    major: u32,
    minor: u32,
    micro: u32,
}

impl Version {
    pub fn new(major: u32, minor: u32, micro: u32) -> Version {
        Version {
            major,
            minor,
            micro,
        }
    }

    pub fn current_sdk_version() -> Version {
        Self::new(
            nvidia_deepstream_sys::NVDS_VERSION_MAJOR,
            nvidia_deepstream_sys::NVDS_VERSION_MINOR,
            nvidia_deepstream_sys::NVDS_VERSION_MICRO,
        )
    }
}
