use std::ffi::CStr;
use std::ptr::NonNull;
use crate::meta::{BaseMeta, ClassifierMeta, MetaList, UserMeta};
use crate::WrapperExt;

#[repr(u32)]
pub enum AudioFormat {
    InvalidFormat = nvidia_deepstream_sys::NvBufAudioFormat_NVBUF_AUDIO_INVALID_FORMAT as _,
    S8 = nvidia_deepstream_sys::NvBufAudioFormat_NVBUF_AUDIO_S8 as _,
    U8 = nvidia_deepstream_sys::NvBufAudioFormat_NVBUF_AUDIO_U8 as _,
    S16LE = nvidia_deepstream_sys::NvBufAudioFormat_NVBUF_AUDIO_S16LE as _,
    S16BE = nvidia_deepstream_sys::NvBufAudioFormat_NVBUF_AUDIO_S16BE as _,
    U16LE = nvidia_deepstream_sys::NvBufAudioFormat_NVBUF_AUDIO_U16LE as _,
    U16BE = nvidia_deepstream_sys::NvBufAudioFormat_NVBUF_AUDIO_U16BE as _,
    S24_32LE = nvidia_deepstream_sys::NvBufAudioFormat_NVBUF_AUDIO_S24_32LE as _,
    S24_32BE = nvidia_deepstream_sys::NvBufAudioFormat_NVBUF_AUDIO_S24_32BE as _,
    U24_32LE = nvidia_deepstream_sys::NvBufAudioFormat_NVBUF_AUDIO_U24_32LE as _,
    U24_32BE = nvidia_deepstream_sys::NvBufAudioFormat_NVBUF_AUDIO_U24_32BE as _,
    S32LE = nvidia_deepstream_sys::NvBufAudioFormat_NVBUF_AUDIO_S32LE as _,
    S32BE = nvidia_deepstream_sys::NvBufAudioFormat_NVBUF_AUDIO_S32BE as _,
    U32LE = nvidia_deepstream_sys::NvBufAudioFormat_NVBUF_AUDIO_U32LE as _,
    U32BE = nvidia_deepstream_sys::NvBufAudioFormat_NVBUF_AUDIO_U32BE as _,
    S24LE = nvidia_deepstream_sys::NvBufAudioFormat_NVBUF_AUDIO_S24LE as _,
    S24BE = nvidia_deepstream_sys::NvBufAudioFormat_NVBUF_AUDIO_S24BE as _,
    U24LE = nvidia_deepstream_sys::NvBufAudioFormat_NVBUF_AUDIO_U24LE as _,
    U24BE = nvidia_deepstream_sys::NvBufAudioFormat_NVBUF_AUDIO_U24BE as _,
    S20LE = nvidia_deepstream_sys::NvBufAudioFormat_NVBUF_AUDIO_S20LE as _,
    S20BE = nvidia_deepstream_sys::NvBufAudioFormat_NVBUF_AUDIO_S20BE as _,
    U20LE = nvidia_deepstream_sys::NvBufAudioFormat_NVBUF_AUDIO_U20LE as _,
    U20BE = nvidia_deepstream_sys::NvBufAudioFormat_NVBUF_AUDIO_U20BE as _,
    S18LE = nvidia_deepstream_sys::NvBufAudioFormat_NVBUF_AUDIO_S18LE as _,
    S18BE = nvidia_deepstream_sys::NvBufAudioFormat_NVBUF_AUDIO_S18BE as _,
    U18LE = nvidia_deepstream_sys::NvBufAudioFormat_NVBUF_AUDIO_U18LE as _,
    U18BE = nvidia_deepstream_sys::NvBufAudioFormat_NVBUF_AUDIO_U18BE as _,
    F32LE = nvidia_deepstream_sys::NvBufAudioFormat_NVBUF_AUDIO_F32LE as _,
    F32BE = nvidia_deepstream_sys::NvBufAudioFormat_NVBUF_AUDIO_F32BE as _,
    F64LE = nvidia_deepstream_sys::NvBufAudioFormat_NVBUF_AUDIO_F64LE as _,
    F64BE = nvidia_deepstream_sys::NvBufAudioFormat_NVBUF_AUDIO_F64BE as _,
}

#[repr(u32)]
pub enum AudioLayout {
    InvalidLayout = nvidia_deepstream_sys::NvBufAudioLayout_NVBUF_AUDIO_INVALID_LAYOUT as _,
    Interleaved = nvidia_deepstream_sys::NvBufAudioLayout_NVBUF_AUDIO_INTERLEAVED as _,
    NonInterleaved = nvidia_deepstream_sys::NvBufAudioLayout_NVBUF_AUDIO_NON_INTERLEAVED as _
}

crate::wrapper_impl!(AudioParams, nvidia_deepstream_sys::NvBufAudioParams);

impl AudioParams {
    pub fn layout(&self) -> AudioLayout {
        unsafe { std::mem::transmute(self.as_native_type_ref().layout) }
    }

    pub fn format(&self) -> AudioFormat {
        unsafe { std::mem::transmute(self.as_native_type_ref().format) }
    }

    pub fn bpf(&self) -> u32 {
        self.as_native_type_ref().bpf
    }

    pub fn channels(&self) -> u32 {
        self.as_native_type_ref().channels
    }

    pub fn rate(&self) -> u32 {
        self.as_native_type_ref().rate
    }

    pub fn data_size(&self) -> u32 {
        self.as_native_type_ref().dataSize
    }

    pub fn data_ptr(&self) -> *mut ::std::os::raw::c_void  {
        self.as_native_type_ref().dataPtr
    }

    pub fn data<T: Sized>(&self) -> &[T] {
        unsafe{ std::slice::from_raw_parts(self.data_ptr() as *const T, self.data_size() as usize / std::mem::size_of::<T>()) }
    }

    pub fn pad_id(&self) -> u32 {
        self.as_native_type_ref().padId
    }

    pub fn source_id(&self) -> u32 {
        self.as_native_type_ref().sourceId
    }

    pub fn ntp_timestamp(&self) -> u64 {
        self.as_native_type_ref().ntpTimestamp
    }

    pub fn buf_pts(&self) -> u64 {
        self.as_native_type_ref().bufPts
    }

    pub fn duration(&self) -> u64 {
        self.as_native_type_ref().duration
    }
}

crate::wrapper_impl!(AudioFrameMeta, nvidia_deepstream_sys::NvDsAudioFrameMeta);

impl AudioFrameMeta {
    pub fn base_meta(&self) -> &BaseMeta {
        BaseMeta::from_native_type_ref(&self.as_native_type_ref().base_meta)
    }

    pub fn base_meta_mut(&mut self) -> &mut BaseMeta {
        BaseMeta::from_native_type_mut(&mut self.as_native_type_mut().base_meta)
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

    pub fn num_samples_per_frame(&self) -> i32 {
        self.as_native_type_ref().num_samples_per_frame
    }

    pub fn sample_rate(&self) -> u32 {
        self.as_native_type_ref().sample_rate
    }

    pub fn num_channels(&self) -> u32 {
        self.as_native_type_ref().num_channels
    }

    pub fn format(&self) -> AudioFormat {
        unsafe { std::mem::transmute(self.as_native_type_ref().format) }
    }

    pub fn layout(&self) -> AudioLayout {
        unsafe { std::mem::transmute(self.as_native_type_ref().layout) }
    }

    pub fn infer_done(&self) -> bool {
        self.as_native_type_ref().bInferDone != 0
    }

    pub fn class_id(&self) -> i32 {
        self.as_native_type_ref().class_id
    }

    pub fn confidence(&self) -> f32 {
        self.as_native_type_ref().confidence
    }

    pub fn class_label(&self) -> &CStr {
        unsafe { CStr::from_ptr(self.as_native_type_ref().class_label.as_ptr()) }
    }

    pub fn classifier_meta_list(&self) -> MetaList<ClassifierMeta> {
        MetaList::<ClassifierMeta>::new(NonNull::new(self.as_native_type_ref().classifier_meta_list).unwrap())
    }

    pub fn frame_user_meta_list(&self) -> MetaList<UserMeta> {
        MetaList::<UserMeta>::new(NonNull::new(self.as_native_type_ref().frame_user_meta_list).unwrap())
    }

    pub fn misc_frame_info(&self) -> [i64; 4usize] {
        self.as_native_type_ref().misc_frame_info
    }
}

crate::wrapper_impl!(AudioBatchMeta, nvidia_deepstream_sys::NvDsBatchMeta);

impl crate::mem::NvdsDrop for AudioBatchMeta {
    fn drop(p: NonNull<Self::NativeType>) {
        unsafe {
            nvidia_deepstream_sys::nvds_destroy_audio_batch_meta(p.as_ptr());
        }
    }
}

impl AudioBatchMeta {
    pub fn create(max_batch_size: u32) -> Option<crate::mem::NvdsBox<AudioBatchMeta>> {
        crate::mem::NvdsBox::new(|| unsafe {
            NonNull::new(nvidia_deepstream_sys::nvds_create_audio_batch_meta(
                max_batch_size,
            ))
        })
    }
}
