use std::ptr::NonNull;

pub trait NvdsDrop : crate::WrapperExt {
    fn drop(p: NonNull<Self::NativeType>);
}

pub struct NvdsBox<T: NvdsDrop>(NonNull<T::NativeType>);

impl<T: NvdsDrop> NvdsBox<T> {
    pub fn new<F: FnOnce() -> Option<NonNull<T::NativeType>>>(f: F) -> Option<NvdsBox<T>> {
        f().map(|p| NvdsBox(p))
    }
}

impl<T: NvdsDrop> Drop for NvdsBox<T> {
    fn drop(&mut self) {
        T::drop(self.0);
    }
}