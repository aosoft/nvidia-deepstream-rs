#[repr(u32)]
#[derive(Default, Clone, Copy, PartialEq, Debug)]
pub enum Status {
    #[default]
    Success = nvidia_deepstream_sys::NvDsInferStatus_NVDSINFER_SUCCESS as _,
    ConfigFailed = nvidia_deepstream_sys::NvDsInferStatus_NVDSINFER_CONFIG_FAILED as _,
    CustomLibFailed = nvidia_deepstream_sys::NvDsInferStatus_NVDSINFER_CUSTOM_LIB_FAILED as _,
    InvalidParams = nvidia_deepstream_sys::NvDsInferStatus_NVDSINFER_INVALID_PARAMS as _,
    OutputParsingFailed = nvidia_deepstream_sys::NvDsInferStatus_NVDSINFER_OUTPUT_PARSING_FAILED as _,
    CudaError = nvidia_deepstream_sys::NvDsInferStatus_NVDSINFER_CUDA_ERROR as _,
    TensorrtError = nvidia_deepstream_sys::NvDsInferStatus_NVDSINFER_TENSORRT_ERROR as _,
    ResourceError = nvidia_deepstream_sys::NvDsInferStatus_NVDSINFER_RESOURCE_ERROR as _,
    TritonError = nvidia_deepstream_sys::NvDsInferStatus_NVDSINFER_TRITON_ERROR as _,
    UnknownError = nvidia_deepstream_sys::NvDsInferStatus_NVDSINFER_UNKNOWN_ERROR as _,
}

#[repr(u32)]
#[derive(Default, Clone, Copy, PartialEq, Debug)]
pub enum LogLevel {
    #[default]
    Error = nvidia_deepstream_sys::NvDsInferLogLevel_NVDSINFER_LOG_ERROR as _ ,
    Warning = nvidia_deepstream_sys::NvDsInferLogLevel_NVDSINFER_LOG_WARNING as _,
    Info = nvidia_deepstream_sys::NvDsInferLogLevel_NVDSINFER_LOG_INFO as _,
    Debug = nvidia_deepstream_sys::NvDsInferLogLevel_NVDSINFER_LOG_DEBUG as _,
}
