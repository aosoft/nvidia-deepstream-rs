#[repr(i32)]
pub enum MetaType {
    InvalidMeta = nvidia_deepstream_sys::NvDsMetaType_NVDS_INVALID_META as _,
    BatchMeta = nvidia_deepstream_sys::NvDsMetaType_NVDS_BATCH_META as _,
    FrameMeta = nvidia_deepstream_sys::NvDsMetaType_NVDS_FRAME_META as _,
    ObjMeta = nvidia_deepstream_sys::NvDsMetaType_NVDS_OBJ_META as _,
    DisplayMeta = nvidia_deepstream_sys::NvDsMetaType_NVDS_DISPLAY_META as _,
    ClassifierMeta = nvidia_deepstream_sys::NvDsMetaType_NVDS_CLASSIFIER_META as _,
    LabelInfoMeta = nvidia_deepstream_sys::NvDsMetaType_NVDS_LABEL_INFO_META as _,
    UserMeta = nvidia_deepstream_sys::NvDsMetaType_NVDS_USER_META as _,
    PayloadMeta = nvidia_deepstream_sys::NvDsMetaType_NVDS_PAYLOAD_META as _,
    EventMsgMeta = nvidia_deepstream_sys::NvDsMetaType_NVDS_EVENT_MSG_META as _,
    OpticalFlowMeta = nvidia_deepstream_sys::NvDsMetaType_NVDS_OPTICAL_FLOW_META as _,
    LatencyMeasurementMeta = nvidia_deepstream_sys::NvDsMetaType_NVDS_LATENCY_MEASUREMENT_META as _,
    TensorOutputMeta = nvidia_deepstream_sys::NvDsMetaType_NVDSINFER_TENSOR_OUTPUT_META as _,
    InferSegmentationMeta = nvidia_deepstream_sys::NvDsMetaType_NVDSINFER_SEGMENTATION_META as _,
    CropImageDataMeta = nvidia_deepstream_sys::NvDsMetaType_NVDS_CROP_IMAGE_META as _,
    TrackerPastFrameMeta = nvidia_deepstream_sys::NvDsMetaType_NVDS_TRACKER_PAST_FRAME_META as _,
    AudioBatchMeta = nvidia_deepstream_sys::NvDsMetaType_NVDS_AUDIO_BATCH_META as _,
    AudioFrameMeta = nvidia_deepstream_sys::NvDsMetaType_NVDS_AUDIO_FRAME_META as _,
    PreprocessFrameMeta = nvidia_deepstream_sys::NvDsMetaType_NVDS_PREPROCESS_FRAME_META as _,
    PreprocessBatchMeta = nvidia_deepstream_sys::NvDsMetaType_NVDS_PREPROCESS_BATCH_META as _,
    CustomMsgBlob = nvidia_deepstream_sys::NvDsMetaType_NVDS_CUSTOM_MSG_BLOB as _,
    ReservedMeta = nvidia_deepstream_sys::NvDsMetaType_NVDS_RESERVED_META as _,
    GstCustomMeta = nvidia_deepstream_sys::NvDsMetaType_NVDS_GST_CUSTOM_META as _,
    StartUserMeta = nvidia_deepstream_sys::NvDsMetaType_NVDS_START_USER_META as _,
}

pub struct BaseMeta<'a> {
    data: &'a nvidia_deepstream_sys::NvDsBaseMeta,
}

impl BaseMeta<'_> {
    pub fn meta_type(&self) -> MetaType {
        unsafe { std::mem::transmute((*(self.data)).meta_type) }
    }

    pub unsafe fn user_context(&self) -> *mut () {
        (*(self.data)).uContext as _
    }
}

pub struct BatchMeta {
    owned: bool,
    data: *mut nvidia_deepstream_sys::NvDsBatchMeta,
}

impl Drop for BatchMeta {
    fn drop(&mut self) {
        if self.owned {
            unsafe {
                nvidia_deepstream_sys::nvds_destroy_batch_meta(self.data);
            }
        }
    }
}

impl BatchMeta {
    pub fn new(max_batch_size: u32) -> Option<BatchMeta> {
        unsafe {
            let p = nvidia_deepstream_sys::nvds_create_batch_meta(max_batch_size);
            if p != std::ptr::null_mut() {
                Some(BatchMeta {
                    owned: true,
                    data: p,
                })
            } else {
                None
            }
        }
    }

    pub fn base_meta(&self) -> BaseMeta {
        unsafe {
            BaseMeta { data: &(*self.data).base_meta }
        }
    }

    pub fn max_frames_in_batch(&self) -> u32 {
        unsafe {
            (*self.data).max_frames_in_batch
        }
    }

    pub fn num_frames_in_batch(&self) -> u32 {
        unsafe {
            (*self.data).num_frames_in_batch
        }
    }

}
