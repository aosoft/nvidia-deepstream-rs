use crate::WrapperExt;
crate::wrapper_impl_value_type!(
    FlowVector,
    nvidia_deepstream_sys::NvOFFlowVector
);

impl FlowVector {
    pub fn new(x: i16, y: i16) -> FlowVector {
        FlowVector::from_native_type(nvidia_deepstream_sys::NvOFFlowVector { flowx: x, flowy: y })
    }

    pub fn x(&self) -> i16 {
        self.as_native_type_ref().flowx
    }

    pub fn y(&self) -> i16 {
        self.as_native_type_ref().flowy
    }
}

crate::wrapper_impl_ref_type!(OpticalFlowMeta, nvidia_deepstream_sys::NvDsOpticalFlowMeta);

impl OpticalFlowMeta {
    pub fn rows(&self) -> u32 {
        self.as_native_type_ref().rows
    }

    pub fn cols(&self) -> u32 {
        self.as_native_type_ref().cols
    }

    pub fn frame_num(&self) -> u64 {
        self.as_native_type_ref().frame_num
    }

    pub fn data(&self) -> &[FlowVector] {
        unsafe {
            std::slice::from_raw_parts(
                self.as_native_type_ref().data as _,
                self.as_native_type_ref().mv_size as usize / std::mem::size_of::<FlowVector>(),
            )
        }
    }
}
