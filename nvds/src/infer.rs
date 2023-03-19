use crate::WrapperExt;
use std::ptr::NonNull;
use gstreamer::glib::GStr;

#[repr(u32)]
#[derive(Default, Clone, Copy, PartialEq, Debug)]
pub enum DataType {
    #[default]
    Float = nvidia_deepstream_sys::NvDsInferDataType_FLOAT as _,
    Half = nvidia_deepstream_sys::NvDsInferDataType_HALF as _,
    Int8 = nvidia_deepstream_sys::NvDsInferDataType_INT8 as _,
    Int32 = nvidia_deepstream_sys::NvDsInferDataType_INT32 as _,
}

#[repr(u32)]
#[derive(Default, Clone, Copy, PartialEq, Debug)]
pub enum Status {
    #[default]
    Success = nvidia_deepstream_sys::NvDsInferStatus_NVDSINFER_SUCCESS as _,
    ConfigFailed = nvidia_deepstream_sys::NvDsInferStatus_NVDSINFER_CONFIG_FAILED as _,
    CustomLibFailed = nvidia_deepstream_sys::NvDsInferStatus_NVDSINFER_CUSTOM_LIB_FAILED as _,
    InvalidParams = nvidia_deepstream_sys::NvDsInferStatus_NVDSINFER_INVALID_PARAMS as _,
    OutputParsingFailed =
        nvidia_deepstream_sys::NvDsInferStatus_NVDSINFER_OUTPUT_PARSING_FAILED as _,
    CudaError = nvidia_deepstream_sys::NvDsInferStatus_NVDSINFER_CUDA_ERROR as _,
    TensorrtError = nvidia_deepstream_sys::NvDsInferStatus_NVDSINFER_TENSORRT_ERROR as _,
    ResourceError = nvidia_deepstream_sys::NvDsInferStatus_NVDSINFER_RESOURCE_ERROR as _,
    TritonError = nvidia_deepstream_sys::NvDsInferStatus_NVDSINFER_TRITON_ERROR as _,
    UnknownError = nvidia_deepstream_sys::NvDsInferStatus_NVDSINFER_UNKNOWN_ERROR as _,
}

impl Status {
    pub fn to_str(&self) -> &GStr {
        unsafe {
            NonNull::new(nvidia_deepstream_sys::NvDsInferStatus2Str(*self as _) as _)
                .map(|x| GStr::from_ptr(x.as_ptr()))
                .unwrap_or_default()
        }
    }
}

#[repr(u32)]
#[derive(Default, Clone, Copy, PartialEq, Debug)]
pub enum LogLevel {
    #[default]
    Error = nvidia_deepstream_sys::NvDsInferLogLevel_NVDSINFER_LOG_ERROR as _,
    Warning = nvidia_deepstream_sys::NvDsInferLogLevel_NVDSINFER_LOG_WARNING as _,
    Info = nvidia_deepstream_sys::NvDsInferLogLevel_NVDSINFER_LOG_INFO as _,
    Debug = nvidia_deepstream_sys::NvDsInferLogLevel_NVDSINFER_LOG_DEBUG as _,
}

crate::wrapper_impl_ref_type!(Dims, nvidia_deepstream_sys::NvDsInferDims);

impl Dims {
    pub fn d(&self) -> &[u32] {
        unsafe {
            std::slice::from_raw_parts(
                self.as_native_type_ref().d.as_ptr(),
                self.as_native_type_ref().numDims as _,
            )
        }
    }

    pub fn num_elements(&self) -> u32 {
        self.as_native_type_ref().numElements
    }

    pub fn to_chw(&self) -> Option<DimsChw> {
        let d = self.d();
        if d.len() >= 3 {
            Some(DimsChw::from_native_type(
                nvidia_deepstream_sys::NvDsInferDimsCHW {
                    c: d[0],
                    h: d[1],
                    w: d[2],
                },
            ))
        } else {
            None
        }
    }

    pub fn to_hwc(&self) -> Option<DimsChw> {
        let d = self.d();
        if d.len() >= 3 {
            Some(DimsChw::from_native_type(
                nvidia_deepstream_sys::NvDsInferDimsCHW {
                    c: d[2],
                    h: d[0],
                    w: d[1],
                },
            ))
        } else {
            None
        }
    }
}

crate::wrapper_impl_ref_type!(DimsChw, nvidia_deepstream_sys::NvDsInferDimsCHW);

impl DimsChw {
    pub fn c(&self) -> u32 {
        self.as_native_type_ref().c
    }

    pub fn h(&self) -> u32 {
        self.as_native_type_ref().h
    }

    pub fn w(&self) -> u32 {
        self.as_native_type_ref().w
    }
}

crate::wrapper_impl_ref_type!(LayerInfo, nvidia_deepstream_sys::NvDsInferLayerInfo);

impl LayerInfo {
    pub fn data_type(&self) -> DataType {
        unsafe { std::mem::transmute(self.as_native_type_ref().dataType) }
    }

    pub fn infer_dims(&self) -> Dims {
        unsafe { Dims::from_native_type(self.as_native_type_ref().__bindgen_anon_1.inferDims) }
    }

    pub fn binding_index(&self) -> i32 {
        self.as_native_type_ref().bindingIndex
    }

    pub fn layer_name(&self) -> &GStr {
        unsafe { GStr::from_ptr(self.as_native_type_ref().layerName) }
    }

    pub unsafe fn buffer(&self) -> *mut () {
        self.as_native_type_ref().buffer as _
    }

    pub fn is_input(&self) -> bool {
        self.as_native_type_ref().isInput != 0
    }
}

crate::wrapper_impl_ref_type!(NetworkInfo, nvidia_deepstream_sys::NvDsInferNetworkInfo);

impl NetworkInfo {
    pub fn width(&self) -> u32 {
        self.as_native_type_ref().width
    }

    pub fn height(&self) -> u32 {
        self.as_native_type_ref().height
    }

    pub fn channels(&self) -> u32 {
        self.as_native_type_ref().channels
    }
}

crate::wrapper_impl_ref_type!(
    ObjectDetectionInfo,
    nvidia_deepstream_sys::NvDsInferObjectDetectionInfo
);

impl ObjectDetectionInfo {
    pub fn class_id(&self) -> u32 {
        self.as_native_type_ref().classId
    }

    pub fn left(&self) -> f32 {
        self.as_native_type_ref().left
    }

    pub fn top(&self) -> f32 {
        self.as_native_type_ref().top
    }

    pub fn width(&self) -> f32 {
        self.as_native_type_ref().width
    }

    pub fn height(&self) -> f32 {
        self.as_native_type_ref().height
    }

    pub fn detection_confidence(&self) -> f32 {
        self.as_native_type_ref().detectionConfidence
    }
}

crate::wrapper_impl_ref_type!(
    InstanceMaskInfo,
    nvidia_deepstream_sys::NvDsInferInstanceMaskInfo
);

impl InstanceMaskInfo {
    pub fn class_id(&self) -> u32 {
        self.as_native_type_ref().classId
    }

    pub fn left(&self) -> f32 {
        self.as_native_type_ref().left
    }

    pub fn top(&self) -> f32 {
        self.as_native_type_ref().top
    }

    pub fn width(&self) -> f32 {
        self.as_native_type_ref().width
    }

    pub fn height(&self) -> f32 {
        self.as_native_type_ref().height
    }

    pub fn detection_confidence(&self) -> f32 {
        self.as_native_type_ref().detectionConfidence
    }

    pub fn mask(&self) -> &[f32] {
        unsafe {
            std::slice::from_raw_parts(
                self.as_native_type_ref().mask,
                self.as_native_type_ref().mask_size as usize / std::mem::size_of::<f32>(),
            )
        }
    }

    pub fn mask_width(&self) -> u32 {
        self.as_native_type_ref().mask_width
    }

    pub fn mask_height(&self) -> u32 {
        self.as_native_type_ref().mask_height
    }
}

crate::wrapper_impl_ref_type!(Attribute, nvidia_deepstream_sys::NvDsInferAttribute);

impl Attribute {
    pub fn attribute_index(&self) -> u32 {
        self.as_native_type_ref().attributeIndex
    }

    pub fn attribute_value(&self) -> u32 {
        self.as_native_type_ref().attributeValue
    }

    pub fn attribute_confidence(&self) -> f32 {
        self.as_native_type_ref().attributeConfidence
    }

    pub fn attribute_label(&self) -> &GStr {
        unsafe { GStr::from_ptr(self.as_native_type_ref().attributeLabel) }
    }
}
