use std::ffi::CStr;
use gstreamer::glib;
use std::marker::PhantomData;
use std::ptr::NonNull;
use crate::Wrapper;

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
    phantom: PhantomData<&'a T>,
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
            None => None,
        }
    }
}

pub struct MetaList<'a, T> {
    list: NonNull<nvidia_deepstream_sys::GList>,
    phantom: PhantomData<&'a T>,
}

impl<'a, T> MetaList<'a, T> {
    pub fn new(list: NonNull<nvidia_deepstream_sys::GList>) -> MetaList<'a, T> {
        MetaList {
            list,
            phantom: PhantomData,
        }
    }

    fn iter(&self) -> MetaListIterator<T> {
        MetaListIterator::<T> {
            current: Some(self.list),
            phantom: PhantomData,
        }
    }
}

crate::wrapper_impl!(Meta, nvidia_deepstream_sys::NvDsMeta);

crate::wrapper_impl!(MetaPool, nvidia_deepstream_sys::NvDsMetaPool);

impl MetaPool {
    pub fn meta_type(&self) -> MetaType {
        unsafe { std::mem::transmute(self.as_native_type_ref().meta_type) }
    }

    pub fn max_elements_in_pool(&self) -> u32 {
        self.as_native_type_ref().max_elements_in_pool
    }

    pub fn element_size(&self) -> u32 {
        self.as_native_type_ref().element_size
    }

    pub fn num_empty_elements(&self) -> u32 {
        self.as_native_type_ref().num_empty_elements
    }

    pub fn num_full_elements(&self) -> u32 {
        self.as_native_type_ref().num_full_elements
    }

    pub fn empty_list(&self) -> MetaList<Meta> {
         MetaList::<Meta>::new(NonNull::new(self.as_native_type_ref().empty_list).unwrap())
    }

    pub fn full_list(&self) -> MetaList<Meta> {
        MetaList::<Meta>::new(NonNull::new(self.as_native_type_ref().full_list).unwrap())
    }
}

crate::wrapper_impl!(BaseMeta, nvidia_deepstream_sys::NvDsBaseMeta);

impl BaseMeta {
    pub fn batch_meta(&self) -> Option<&BatchMeta> {
        unsafe {
            if self.as_native_type_ref().batch_meta != std::ptr::null_mut() {
                Some(BatchMeta::from_native_type_ref(&*self.as_native_type_ref().batch_meta))
            } else {
                None
            }
        }
    }

    pub fn meta_type(&self) -> MetaType {
        unsafe { std::mem::transmute(self.as_native_type_ref().meta_type) }
    }

    pub unsafe fn user_context(&self) -> *mut () {
        self.as_native_type_ref().uContext as _
    }
}

crate::wrapper_impl!(BatchMeta, nvidia_deepstream_sys::NvDsBatchMeta);

impl BatchMeta {
    pub fn base_meta(&self) -> &BaseMeta {
        BaseMeta::from_native_type_ref(&self.as_native_type_ref().base_meta)
    }

    pub fn max_frames_in_batch(&self) -> u32 {
        self.as_native_type_ref().max_frames_in_batch
    }

    pub fn num_frames_in_batch(&self) -> u32 {
        self.as_native_type_ref().num_frames_in_batch
    }

    pub fn frame_meta_pool(&self) -> &MetaPool {
        unsafe { MetaPool::from_native_type_ref(&*self.as_native_type_ref().frame_meta_pool) }
    }

    pub fn obj_meta_pool(&self) -> &MetaPool {
        unsafe { MetaPool::from_native_type_ref(&*self.as_native_type_ref().obj_meta_pool) }
    }

    pub fn classifier_meta_pool(&self) -> &MetaPool {
        unsafe { MetaPool::from_native_type_ref(&*self.as_native_type_ref().classifier_meta_pool) }
    }

    pub fn display_meta_pool(&self) -> &MetaPool {
        unsafe { MetaPool::from_native_type_ref(&*self.as_native_type_ref().display_meta_pool) }
    }

    pub fn user_meta_pool(&self) -> &MetaPool {
        unsafe { MetaPool::from_native_type_ref(&*self.as_native_type_ref().user_meta_pool) }
    }

    pub fn label_info_meta_pool(&self) -> &MetaPool {
        unsafe { MetaPool::from_native_type_ref(&*self.as_native_type_ref().label_info_meta_pool) }
    }

    pub fn frame_meta_list(&self) -> MetaList<FrameMeta> {
        MetaList::<FrameMeta>::new(NonNull::new(self.as_native_type_ref().frame_meta_list).unwrap())
    }
}

crate::wrapper_impl!(FrameMeta, nvidia_deepstream_sys::NvDsFrameMeta);

impl FrameMeta {
    pub fn base_meta(&self) -> &BaseMeta {
        BaseMeta::from_native_type_ref(&self.as_native_type_ref().base_meta)
    }

    pub fn pad_index(&self) -> u32 {
        self.as_native_type_ref().pad_index
    }

    pub fn batch_id(&self) -> u32 {
        self.as_native_type_ref().batch_id
    }

    pub fn frame_num(&self) -> i32 {
        self.as_native_type_ref().frame_num
    }

    pub fn buf_pts(&self) -> u64 {
        self.as_native_type_ref().buf_pts
    }

    pub fn ntp_timestamp(&self) -> u64 {
        self.as_native_type_ref().ntp_timestamp
    }
    pub fn source_id(&self) -> u32 {
        self.as_native_type_ref().source_id
    }

    pub fn num_surfaces_per_frame(&self) -> i32 {
        self.as_native_type_ref().num_surfaces_per_frame
    }

    pub fn source_frame_width(&self) -> u32 {
        self.as_native_type_ref().source_frame_width
    }

    pub fn source_frame_height(&self) -> u32 {
        self.as_native_type_ref().source_frame_height
    }

    pub fn surface_type(&self) -> u32 {
        self.as_native_type_ref().surface_type
    }

    pub fn surface_index(&self) -> u32 {
        self.as_native_type_ref().surface_index
    }

    pub fn num_obj_meta(&self) -> u32 {
        self.as_native_type_ref().num_obj_meta
    }

    pub fn infer_done(&self) -> bool {
        self.as_native_type_ref().bInferDone != 0
    }

    pub fn obj_meta_list(&self) -> MetaList<ObjectMeta> {
        MetaList::<ObjectMeta>::new(NonNull::new(self.as_native_type_ref().obj_meta_list).unwrap())
    }

    pub fn display_meta_list(&self) -> MetaList<DisplayMeta> {
        MetaList::<DisplayMeta>::new(NonNull::new(self.as_native_type_ref().display_meta_list).unwrap())
    }

    pub fn frame_user_meta_list(&self) -> MetaList<UserMeta> {
        MetaList::<UserMeta>::new(NonNull::new(self.as_native_type_ref().frame_user_meta_list).unwrap())
    }

    pub fn misc_frame_info(&self) -> [i64; 4usize] {
        self.as_native_type_ref().misc_frame_info
    }

    pub fn pipeline_width(&self) -> u32 {
        self.as_native_type_ref().pipeline_width
    }

    pub fn pipeline_height(&self) -> u32 {
        self.as_native_type_ref().pipeline_height
    }
}

crate::wrapper_impl!(ObjectMeta, nvidia_deepstream_sys::NvDsObjectMeta);

impl ObjectMeta {
    pub fn base_meta(&self) -> &BaseMeta {
        BaseMeta::from_native_type_ref(&self.as_native_type_ref().base_meta)
    }

    pub fn parent(&self) -> Option<&ObjectMeta> {
        if self.as_native_type_ref().parent != std::ptr::null_mut() {
            unsafe { Some(ObjectMeta::from_native_type_ref(&*self.as_native_type_ref().parent)) }
        } else {
            None
        }
    }

    pub fn unique_component_id(&self) -> i32 {
        self.as_native_type_ref().unique_component_id
    }

    pub fn class_id(&self) -> i32 {
        self.as_native_type_ref().class_id
    }

    pub fn object_id(&self) -> u64 {
        self.as_native_type_ref().object_id
    }

    pub fn detector_bbox_info(&self) -> &crate::bounding_box::Info {
        crate::bounding_box::Info::from_native_type_ref(&self.as_native_type_ref().detector_bbox_info)
    }

    pub fn tracker_bbox_info(&self) -> &crate::bounding_box::Info {
        crate::bounding_box::Info::from_native_type_ref(&self.as_native_type_ref().tracker_bbox_info)
    }

    pub fn confidence(&self) -> f32 {
        self.as_native_type_ref().confidence
    }

    pub fn tracker_confidence(&self) -> f32 {
        self.as_native_type_ref().tracker_confidence
    }

    pub fn rect_params(&self) -> &crate::osd::RectParams {
        crate::osd::RectParams::from_native_type_ref(&self.as_native_type_ref().rect_params)
    }

    pub fn mask_params(&self) -> &crate::osd::MaskParams {
        crate::osd::MaskParams::from_native_type_ref(&self.as_native_type_ref().mask_params)
    }

    pub fn text_params(&self) -> &crate::osd::TextParams {
        crate::osd::TextParams::from_native_type_ref(&self.as_native_type_ref().text_params)
    }

    pub fn obj_label(&self) -> &str {
        unsafe { CStr::from_ptr(&self.as_native_type_ref().obj_label as _).to_str().unwrap_or_default() }
    }

    pub fn classifier_meta_list(&self) -> MetaList<ClassifierMeta> {
        MetaList::<ClassifierMeta>::new(NonNull::new(self.as_native_type_ref().classifier_meta_list).unwrap())
    }

    pub fn obj_user_meta_list(&self) -> MetaList<UserMeta> {
        MetaList::<UserMeta>::new(NonNull::new(self.as_native_type_ref().obj_user_meta_list).unwrap())
    }

    pub fn misc_obj_info(&self) -> [i64; 4usize] {
        self.as_native_type_ref().misc_obj_info
    }
}

crate::wrapper_impl!(ClassifierMeta, nvidia_deepstream_sys::NvDsClassifierMeta);

impl ClassifierMeta {
    pub fn base_meta(&self) -> &BaseMeta {
        BaseMeta::from_native_type_ref(&self.as_native_type_ref().base_meta)
    }
}

crate::wrapper_impl!(DisplayMeta, nvidia_deepstream_sys::NvDsDisplayMeta);

impl DisplayMeta {
    pub fn base_meta(&self) -> &BaseMeta {
        BaseMeta::from_native_type_ref(&self.as_native_type_ref().base_meta)
    }
}

crate::wrapper_impl!(UserMeta, nvidia_deepstream_sys::NvDsUserMeta);

impl UserMeta {
    pub fn base_meta(&self) -> &BaseMeta {
        BaseMeta::from_native_type_ref(&self.as_native_type_ref().base_meta)
    }
}

