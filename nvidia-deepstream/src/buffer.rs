use crate::meta::BatchMeta;
use crate::Wrapper;

pub trait BufferNvdsExt: 'static {
    fn get_nvds_batch_meta(&self) -> Option<&BatchMeta>;
}

impl BufferNvdsExt for gstreamer::Buffer {
    fn get_nvds_batch_meta(&self) -> Option<&BatchMeta> {
        unsafe {
            let batch_meta = nvidia_deepstream_sys::gst_buffer_get_nvds_batch_meta(
                self.as_mut_ptr() as *mut nvidia_deepstream_sys::GstBuffer,
            );

            if batch_meta != std::ptr::null_mut() {
                Some(BatchMeta::from_native_type_ref(&*batch_meta))
            } else {
                None
            }
        }
    }
}