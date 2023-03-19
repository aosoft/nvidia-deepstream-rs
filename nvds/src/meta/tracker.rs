use gstreamer::glib::GStr;
use crate::WrapperExt;
crate::wrapper_impl_ref_type!(PastFrameObj, nvidia_deepstream_sys::NvDsPastFrameObj);

impl PastFrameObj {
    pub fn frame_num(&self) -> u32 {
        self.as_native_type_ref().frameNum
    }

    pub fn bbox(&self) -> &super::osd::RectParams {
        super::osd::RectParams::from_native_type_ref(&self.as_native_type_ref().tBbox)
    }

    pub fn confidence(&self) -> f32 {
        self.as_native_type_ref().confidence
    }

    pub fn age(&self) -> u32 {
        self.as_native_type_ref().age
    }
}

crate::wrapper_impl_ref_type!(
    PastFrameObjList,
    nvidia_deepstream_sys::NvDsPastFrameObjList
);

impl PastFrameObjList {
    pub fn list(&self) -> &[PastFrameObj] {
        unsafe {
            std::slice::from_raw_parts(
                self.as_native_type_ref().list as _,
                self.as_native_type_ref().numObj as _,
            )
        }
    }

    pub fn unique_id(&self) -> u64 {
        self.as_native_type_ref().uniqueId
    }

    pub fn class_id(&self) -> u16 {
        self.as_native_type_ref().classId
    }

    pub fn obj_label(&self) -> &GStr {
        unsafe { GStr::from_ptr(self.as_native_type_ref().objLabel.as_ptr()) }
    }
}

crate::wrapper_impl_ref_type!(
    PastFrameObjStream,
    nvidia_deepstream_sys::NvDsPastFrameObjStream
);

impl PastFrameObjStream {
    pub fn list(&self) -> &[PastFrameObj] {
        unsafe {
            std::slice::from_raw_parts(
                self.as_native_type_ref().list as _,
                self.as_native_type_ref().numFilled as _,
            )
        }
    }

    pub fn stream_id(&self) -> u32 {
        self.as_native_type_ref().streamID
    }

    pub fn surface_stream_id(&self) -> u64 {
        self.as_native_type_ref().surfaceStreamID
    }

    pub fn num_allocated(&self) -> u32 {
        self.as_native_type_ref().numAllocated
    }
}

crate::wrapper_impl_ref_type!(
    PastFrameObjBatch,
    nvidia_deepstream_sys::NvDsPastFrameObjBatch
);

impl PastFrameObjBatch {
    pub fn list(&self) -> &[PastFrameObjStream] {
        unsafe {
            std::slice::from_raw_parts(
                self.as_native_type_ref().list as _,
                self.as_native_type_ref().numFilled as _,
            )
        }
    }
    pub fn num_allocated(&self) -> u32 {
        self.as_native_type_ref().numAllocated
    }
}
