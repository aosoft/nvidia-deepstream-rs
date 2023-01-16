use crate::WrapperExt;

#[repr(u32)]
pub enum SurfaceType {
    NONE = nvidia_deepstream_sys::NvDsSurfaceType_NVDS_META_SURFACE_NONE as _,
    FishPushbroom = nvidia_deepstream_sys::NvDsSurfaceType_NVDS_META_SURFACE_FISH_PUSHBROOM as _,
    FishVertcyl = nvidia_deepstream_sys::NvDsSurfaceType_NVDS_META_SURFACE_FISH_VERTCYL as _,
    PerspectivePerspective = nvidia_deepstream_sys::NvDsSurfaceType_NVDS_META_SURFACE_PERSPECTIVE_PERSPECTIVE as _,
}

crate::wrapper_impl!(DewarperSurfaceMeta, nvidia_deepstream_sys::NvDewarperSurfaceMeta);

impl DewarperSurfaceMeta {
    pub fn surface_type(&self) -> &[SurfaceType] {
        unsafe { std::mem::transmute::<_, [SurfaceType; 4usize]>(self.as_native_type_ref().type_).as_slice() }
    }

    pub fn index(&self) -> &[u32] {
        self.as_native_type_ref().index.as_slice()
    }

    pub fn source_id(&self) -> u32 {
        self.as_native_type_ref().source_id
    }

    pub fn num_filled_surfaces(&self) -> u32 {
        self.as_native_type_ref().num_filled_surfaces
    }
}