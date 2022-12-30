use std::marker::PhantomData;
use gstreamer::glib;
use std::path::Iter;
use std::ptr::NonNull;

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

pub struct MetaListIterator<'a, T> {
    current: Option<NonNull<nvidia_deepstream_sys::GList>>,
    phantom: PhantomData<&'a T>
}

impl<'a, T> Iterator for MetaListIterator<'a, T> {
    type Item = &'a T;

    fn next(&mut self) -> Option<Self::Item> {
        match self.current {
            Some(cur) => unsafe {
                self.current = NonNull::new(cur.as_ref().next);
                let item = &*(&cur.as_ref().data as *const glib::ffi::gpointer as *const T);
                Some(item)
            },
            None => None
        }
    }
}

pub struct MetaList<'a, T> {
    list: NonNull<nvidia_deepstream_sys::GList>,
    phantom: PhantomData<&'a T>
}

impl<'a, T> MetaList<'a, T> {
    pub fn new(list: NonNull<nvidia_deepstream_sys::GList>) -> MetaList<'a, T> {
        MetaList { list, phantom: PhantomData }
    }

    fn iter(&self) -> MetaListIterator<T> {
        MetaListIterator::<T> { current: Some(self.list), phantom: PhantomData }
    }
}

pub struct Meta(NonNull<nvidia_deepstream_sys::NvDsMeta>);

pub struct MetaPool<'a>(&'a nvidia_deepstream_sys::NvDsMetaPool);

impl MetaPool<'_> {
    pub fn meta_type(&self) -> MetaType {
        unsafe { std::mem::transmute(self.0.meta_type) }
    }

    pub fn max_elements_in_pool(&self) -> u32 {
        self.0.max_elements_in_pool
    }

    pub fn element_size(&self) -> u32 {
        self.0.element_size
    }

    pub fn num_empty_elements(&self) -> u32 {
        self.0.num_empty_elements
    }

    pub fn num_full_elements(&self) -> u32 {
        self.0.num_full_elements
    }

    pub fn empty_list(&self) -> MetaList<Meta> {
        unsafe { MetaList::<Meta>::new(NonNull::new(self.0.empty_list).unwrap()) }
    }

    pub fn full_list(&self) -> MetaList<Meta> {
        unsafe { MetaList::<Meta>::new(NonNull::new(self.0.full_list).unwrap()) }
    }
}

pub struct BaseMeta<'a>(&'a nvidia_deepstream_sys::NvDsBaseMeta);

impl BaseMeta<'_> {
    pub fn batch_meta(&self) -> Option<BatchMeta> {
        unsafe {
            if self.0.batch_meta != std::ptr::null_mut() {
                Some(BatchMeta(&*self.0.batch_meta))
            } else {
                None
            }
        }
    }

    pub fn meta_type(&self) -> MetaType {
        unsafe { std::mem::transmute(self.0.meta_type) }
    }

    pub unsafe fn user_context(&self) -> *mut () {
        self.0.uContext as _
    }
}

pub struct BatchMeta<'a>(&'a nvidia_deepstream_sys::NvDsBatchMeta);

impl BatchMeta<'_> {
    pub fn base_meta(&self) -> BaseMeta {
        unsafe { BaseMeta(&self.0.base_meta) }
    }

    pub fn max_frames_in_batch(&self) -> u32 {
        unsafe { self.0.max_frames_in_batch }
    }

    pub fn num_frames_in_batch(&self) -> u32 {
        unsafe { self.0.num_frames_in_batch }
    }

    pub fn frame_meta_pool(&self) -> MetaPool {
        unsafe { MetaPool(&*self.0.frame_meta_pool) }
    }

    pub fn obj_meta_pool(&self) -> MetaPool {
        unsafe { MetaPool(&*self.0.obj_meta_pool) }
    }

    pub fn classifier_meta_pool(&self) -> MetaPool {
        unsafe { MetaPool(&*self.0.classifier_meta_pool) }
    }

    pub fn display_meta_pool(&self) -> MetaPool {
        unsafe { MetaPool(&*self.0.display_meta_pool) }
    }

    pub fn user_meta_pool(&self) -> MetaPool {
        unsafe { MetaPool(&*self.0.user_meta_pool) }
    }

    pub fn label_info_meta_pool(&self) -> MetaPool {
        unsafe { MetaPool(&*self.0.label_info_meta_pool) }
    }

    /*pub fn frame_meta_list(&self) -> MetaPool {
        unsafe { MetaPool(&*self.0.frame_meta_list) }
    }*/
}
