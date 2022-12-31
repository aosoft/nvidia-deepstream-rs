pub mod bounding_box;
pub mod element;
pub mod meta;
pub mod osd;

#[inline(always)]
pub(crate) fn to_wrapper_ref<W: Sized, N: Sized>(r: &N) -> &W {
    debug_assert!(std::mem::size_of::<W>() == std::mem::size_of::<N>());
    unsafe { std::mem::transmute(r) }
}