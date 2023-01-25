use crate::WrapperExt;
use std::ffi::CStr;
use std::ptr::NonNull;

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
    pub fn to_str(&self) -> &CStr {
        unsafe {
            NonNull::new(nvidia_deepstream_sys::NvDsInferStatus2Str(*self as _) as _)
                .map(|x| CStr::from_ptr(x.as_ptr()))
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

crate::wrapper_impl!(Dims, nvidia_deepstream_sys::NvDsInferDims);

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
}

crate::wrapper_impl!(DimsChw, nvidia_deepstream_sys::NvDsInferDimsCHW);

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

crate::wrapper_impl!(LayerInfo, nvidia_deepstream_sys::NvDsInferLayerInfo);

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

    pub fn layer_name(&self) -> &CStr {
        unsafe { CStr::from_ptr(self.as_native_type_ref().layerName) }
    }

    pub unsafe fn buffer(&self) -> *mut () {
        self.as_native_type_ref().buffer as _
    }

    pub fn is_input(&self) -> bool {
        self.as_native_type_ref().isInput != 0
    }
}

crate::wrapper_impl!(NetworkInfo, nvidia_deepstream_sys::NvDsInferNetworkInfo);

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
