use crate::meta::UserMeta;
use crate::WrapperExt;
use std::ptr::NonNull;
use gstreamer::glib::GStr;

crate::wrapper_impl_ref_type!(
    MetaSubCompLatency,
    nvidia_deepstream_sys::NvDsMetaSubCompLatency
);

impl MetaSubCompLatency {
    pub fn sub_comp_name(&self) -> &GStr {
        unsafe { GStr::from_ptr(self.as_native_type_ref().sub_comp_name.as_ptr()) }
    }

    pub fn in_system_timestamp(&self) -> f64 {
        self.as_native_type_ref().in_system_timestamp
    }

    pub fn out_system_timestamp(&self) -> f64 {
        self.as_native_type_ref().out_system_timestamp
    }
}

crate::wrapper_impl_ref_type!(MetaCompLatency, nvidia_deepstream_sys::NvDsMetaCompLatency);

impl MetaCompLatency {
    pub fn component_name(&self) -> &GStr {
        unsafe { GStr::from_ptr(self.as_native_type_ref().component_name.as_ptr()) }
    }

    pub fn in_system_timestamp(&self) -> f64 {
        self.as_native_type_ref().in_system_timestamp
    }

    pub fn out_system_timestamp(&self) -> f64 {
        self.as_native_type_ref().out_system_timestamp
    }

    pub fn source_id(&self) -> u32 {
        self.as_native_type_ref().source_id
    }

    pub fn frame_num(&self) -> u32 {
        self.as_native_type_ref().frame_num
    }

    pub fn pad_index(&self) -> u32 {
        self.as_native_type_ref().pad_index
    }

    pub fn sub_comp_latencies(&self, index: usize) -> Option<&MetaSubCompLatency> {
        if index < self.as_native_type_ref().sub_comp_latencies.len() {
            Some(MetaSubCompLatency::from_native_type_ref(
                &self.as_native_type_ref().sub_comp_latencies[index],
            ))
        } else {
            None
        }
    }

    pub fn num_sub_comps(&self) -> u32 {
        self.as_native_type_ref().num_sub_comps
    }
}

crate::wrapper_impl_ref_type!(
    FrameLatencyInfo,
    nvidia_deepstream_sys::NvDsFrameLatencyInfo
);

impl FrameLatencyInfo {
    pub fn source_id(&self) -> u32 {
        self.as_native_type_ref().source_id
    }

    pub fn frame_num(&self) -> u32 {
        self.as_native_type_ref().frame_num
    }

    pub fn comp_in_timestamp(&self) -> f64 {
        self.as_native_type_ref().comp_in_timestamp
    }

    pub fn latency(&self) -> f64 {
        self.as_native_type_ref().latency
    }
}

pub trait BufferExt {
    fn set_input_system_timestamp(&self, element_name: &GStr) -> Option<&UserMeta>;
    fn set_output_system_timestamp(&self, element_name: &GStr) -> Result<(), bool>;
    fn measure_buffer_latency(&self, latency_info: &FrameLatencyInfo) -> u32;
    fn add_reference_timestamp_meta(&self, element_name: &GStr, frame_id: u32);
}

impl BufferExt for gstreamer::Buffer {
    fn set_input_system_timestamp(&self, element_name: &GStr) -> Option<&UserMeta> {
        unsafe {
            NonNull::new(nvidia_deepstream_sys::nvds_set_input_system_timestamp(
                self.as_ptr() as _,
                element_name.as_ptr() as _,
            ))
            .map(|p| UserMeta::from_native_type_ref(p.as_ref()))
        }
    }

    fn set_output_system_timestamp(&self, element_name: &GStr) -> Result<(), bool> {
        unsafe {
            if nvidia_deepstream_sys::nvds_set_output_system_timestamp(
                self.as_ptr() as _,
                element_name.as_ptr() as _,
            ) != 0
            {
                Ok(())
            } else {
                Err(false)
            }
        }
    }

    fn measure_buffer_latency(&self, latency_info: &FrameLatencyInfo) -> u32 {
        unsafe {
            nvidia_deepstream_sys::nvds_measure_buffer_latency(
                self.as_ptr() as _,
                latency_info.as_native_type_ptr(),
            )
        }
    }

    fn add_reference_timestamp_meta(&self, element_name: &GStr, frame_id: u32) {
        unsafe {
            nvidia_deepstream_sys::nvds_add_reference_timestamp_meta(
                self.as_ptr() as _,
                element_name.as_ptr() as _,
                frame_id,
            )
        }
    }
}
