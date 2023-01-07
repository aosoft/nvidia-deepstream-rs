#![allow(non_camel_case_types)]

#[repr(u32)]
#[derive(Clone, Copy, PartialEq, Debug)]
pub enum MemMapFlags {
    Read = nvidia_deepstream_sys::NvBufSurfaceMemMapFlags_NVBUF_MAP_READ as _,
    Write = nvidia_deepstream_sys::NvBufSurfaceMemMapFlags_NVBUF_MAP_WRITE as _,
    ReadWrite = nvidia_deepstream_sys::NvBufSurfaceMemMapFlags_NVBUF_MAP_READ_WRITE as _,
}

#[repr(u32)]
#[derive(Clone, Copy, PartialEq, Debug)]
pub enum Tag {
    None = nvidia_deepstream_sys::NvBufSurfaceTag_NvBufSurfaceTag_NONE as _,
    Camera = nvidia_deepstream_sys::NvBufSurfaceTag_NvBufSurfaceTag_CAMERA as _,
    JPEG = nvidia_deepstream_sys::NvBufSurfaceTag_NvBufSurfaceTag_JPEG as _,
    Protected = nvidia_deepstream_sys::NvBufSurfaceTag_NvBufSurfaceTag_PROTECTED as _,
    VideoEnc = nvidia_deepstream_sys::NvBufSurfaceTag_NvBufSurfaceTag_VIDEO_ENC as _,
    VideoDec = nvidia_deepstream_sys::NvBufSurfaceTag_NvBufSurfaceTag_VIDEO_DEC as _,
    VideoConvert = nvidia_deepstream_sys::NvBufSurfaceTag_NvBufSurfaceTag_VIDEO_CONVERT as _,
}

#[repr(u32)]
#[derive(Clone, Copy, PartialEq, Debug)]
pub enum ColorFormat {
    Invalid = nvidia_deepstream_sys::NvBufSurfaceColorFormat_NVBUF_COLOR_FORMAT_INVALID as _,
    Gray8 = nvidia_deepstream_sys::NvBufSurfaceColorFormat_NVBUF_COLOR_FORMAT_GRAY8 as _,
    YUV420 = nvidia_deepstream_sys::NvBufSurfaceColorFormat_NVBUF_COLOR_FORMAT_YUV420 as _,
    YVU420 = nvidia_deepstream_sys::NvBufSurfaceColorFormat_NVBUF_COLOR_FORMAT_YVU420 as _,
    YUV420_ER = nvidia_deepstream_sys::NvBufSurfaceColorFormat_NVBUF_COLOR_FORMAT_YUV420_ER as _,
    YVU420_ER = nvidia_deepstream_sys::NvBufSurfaceColorFormat_NVBUF_COLOR_FORMAT_YVU420_ER as _,
    NV12 = nvidia_deepstream_sys::NvBufSurfaceColorFormat_NVBUF_COLOR_FORMAT_NV12 as _,
    NV12_ER = nvidia_deepstream_sys::NvBufSurfaceColorFormat_NVBUF_COLOR_FORMAT_NV12_ER as _,
    NV21 = nvidia_deepstream_sys::NvBufSurfaceColorFormat_NVBUF_COLOR_FORMAT_NV21 as _,
    NV21_ER = nvidia_deepstream_sys::NvBufSurfaceColorFormat_NVBUF_COLOR_FORMAT_NV21_ER as _,
    UYVY = nvidia_deepstream_sys::NvBufSurfaceColorFormat_NVBUF_COLOR_FORMAT_UYVY as _,
    UYVY_ER = nvidia_deepstream_sys::NvBufSurfaceColorFormat_NVBUF_COLOR_FORMAT_UYVY_ER as _,
    VYUY = nvidia_deepstream_sys::NvBufSurfaceColorFormat_NVBUF_COLOR_FORMAT_VYUY as _,
    VYUY_ER = nvidia_deepstream_sys::NvBufSurfaceColorFormat_NVBUF_COLOR_FORMAT_VYUY_ER as _,
    YUYV = nvidia_deepstream_sys::NvBufSurfaceColorFormat_NVBUF_COLOR_FORMAT_YUYV as _,
    YUYV_ER = nvidia_deepstream_sys::NvBufSurfaceColorFormat_NVBUF_COLOR_FORMAT_YUYV_ER as _,
    YVYU = nvidia_deepstream_sys::NvBufSurfaceColorFormat_NVBUF_COLOR_FORMAT_YVYU as _,
    YVYU_ER = nvidia_deepstream_sys::NvBufSurfaceColorFormat_NVBUF_COLOR_FORMAT_YVYU_ER as _,
    YUV444 = nvidia_deepstream_sys::NvBufSurfaceColorFormat_NVBUF_COLOR_FORMAT_YUV444 as _,
    RGBA = nvidia_deepstream_sys::NvBufSurfaceColorFormat_NVBUF_COLOR_FORMAT_RGBA as _,
    BGRA = nvidia_deepstream_sys::NvBufSurfaceColorFormat_NVBUF_COLOR_FORMAT_BGRA as _,
    ARGB = nvidia_deepstream_sys::NvBufSurfaceColorFormat_NVBUF_COLOR_FORMAT_ARGB as _,
    ABGR = nvidia_deepstream_sys::NvBufSurfaceColorFormat_NVBUF_COLOR_FORMAT_ABGR as _,
    RGBx = nvidia_deepstream_sys::NvBufSurfaceColorFormat_NVBUF_COLOR_FORMAT_RGBx as _,
    BGRx = nvidia_deepstream_sys::NvBufSurfaceColorFormat_NVBUF_COLOR_FORMAT_BGRx as _,
    xRGB = nvidia_deepstream_sys::NvBufSurfaceColorFormat_NVBUF_COLOR_FORMAT_xRGB as _,
    xBGR = nvidia_deepstream_sys::NvBufSurfaceColorFormat_NVBUF_COLOR_FORMAT_xBGR as _,
    RGB = nvidia_deepstream_sys::NvBufSurfaceColorFormat_NVBUF_COLOR_FORMAT_RGB as _,
    BGR = nvidia_deepstream_sys::NvBufSurfaceColorFormat_NVBUF_COLOR_FORMAT_BGR as _,
    NV12_10LE = nvidia_deepstream_sys::NvBufSurfaceColorFormat_NVBUF_COLOR_FORMAT_NV12_10LE as _,
    NV12_12LE = nvidia_deepstream_sys::NvBufSurfaceColorFormat_NVBUF_COLOR_FORMAT_NV12_12LE as _,
    YUV420_709 = nvidia_deepstream_sys::NvBufSurfaceColorFormat_NVBUF_COLOR_FORMAT_YUV420_709 as _,
    YUV420_709_ER =
        nvidia_deepstream_sys::NvBufSurfaceColorFormat_NVBUF_COLOR_FORMAT_YUV420_709_ER as _,
    NV12_709 = nvidia_deepstream_sys::NvBufSurfaceColorFormat_NVBUF_COLOR_FORMAT_NV12_709 as _,
    NV12_709_ER =
        nvidia_deepstream_sys::NvBufSurfaceColorFormat_NVBUF_COLOR_FORMAT_NV12_709_ER as _,
    YUV420_2020 =
        nvidia_deepstream_sys::NvBufSurfaceColorFormat_NVBUF_COLOR_FORMAT_YUV420_2020 as _,
    NV12_2020 = nvidia_deepstream_sys::NvBufSurfaceColorFormat_NVBUF_COLOR_FORMAT_NV12_2020 as _,
    NV12_10LE_ER =
        nvidia_deepstream_sys::NvBufSurfaceColorFormat_NVBUF_COLOR_FORMAT_NV12_10LE_ER as _,
    NV12_10LE_709 =
        nvidia_deepstream_sys::NvBufSurfaceColorFormat_NVBUF_COLOR_FORMAT_NV12_10LE_709 as _,
    NV12_10LE_709_ER =
        nvidia_deepstream_sys::NvBufSurfaceColorFormat_NVBUF_COLOR_FORMAT_NV12_10LE_709_ER as _,
    NV12_10LE_2020 =
        nvidia_deepstream_sys::NvBufSurfaceColorFormat_NVBUF_COLOR_FORMAT_NV12_10LE_2020 as _,
    Signed_R16G16 =
        nvidia_deepstream_sys::NvBufSurfaceColorFormat_NVBUF_COLOR_FORMAT_SIGNED_R16G16 as _,
    R8_G8_B8 = nvidia_deepstream_sys::NvBufSurfaceColorFormat_NVBUF_COLOR_FORMAT_R8_G8_B8 as _,
    B8_G8_R8 = nvidia_deepstream_sys::NvBufSurfaceColorFormat_NVBUF_COLOR_FORMAT_B8_G8_R8 as _,
    R32F_G32F_B32F =
        nvidia_deepstream_sys::NvBufSurfaceColorFormat_NVBUF_COLOR_FORMAT_R32F_G32F_B32F as _,
    B32F_G32F_R32F =
        nvidia_deepstream_sys::NvBufSurfaceColorFormat_NVBUF_COLOR_FORMAT_B32F_G32F_R32F as _,
    YUV422 = nvidia_deepstream_sys::NvBufSurfaceColorFormat_NVBUF_COLOR_FORMAT_YUV422 as _,
    NV21_10LE = nvidia_deepstream_sys::NvBufSurfaceColorFormat_NVBUF_COLOR_FORMAT_NV21_10LE as _,
    NV21_12LE = nvidia_deepstream_sys::NvBufSurfaceColorFormat_NVBUF_COLOR_FORMAT_NV21_12LE as _,
    NV12_12LE_2020 =
        nvidia_deepstream_sys::NvBufSurfaceColorFormat_NVBUF_COLOR_FORMAT_NV12_12LE_2020 as _,
    NV16 = nvidia_deepstream_sys::NvBufSurfaceColorFormat_NVBUF_COLOR_FORMAT_NV16 as _,
    NV16_10LE = nvidia_deepstream_sys::NvBufSurfaceColorFormat_NVBUF_COLOR_FORMAT_NV16_10LE as _,
    NV24 = nvidia_deepstream_sys::NvBufSurfaceColorFormat_NVBUF_COLOR_FORMAT_NV24 as _,
    NV24_10LE = nvidia_deepstream_sys::NvBufSurfaceColorFormat_NVBUF_COLOR_FORMAT_NV24_10LE as _,
    NV16_ER = nvidia_deepstream_sys::NvBufSurfaceColorFormat_NVBUF_COLOR_FORMAT_NV16_ER as _,
    NV24_ER = nvidia_deepstream_sys::NvBufSurfaceColorFormat_NVBUF_COLOR_FORMAT_NV24_ER as _,
    NV16_709 = nvidia_deepstream_sys::NvBufSurfaceColorFormat_NVBUF_COLOR_FORMAT_NV16_709 as _,
    NV24_709 = nvidia_deepstream_sys::NvBufSurfaceColorFormat_NVBUF_COLOR_FORMAT_NV24_709 as _,
    NV16_709_ER =
        nvidia_deepstream_sys::NvBufSurfaceColorFormat_NVBUF_COLOR_FORMAT_NV16_709_ER as _,
    NV24_709_ER =
        nvidia_deepstream_sys::NvBufSurfaceColorFormat_NVBUF_COLOR_FORMAT_NV24_709_ER as _,
    NV24_10LE_709 =
        nvidia_deepstream_sys::NvBufSurfaceColorFormat_NVBUF_COLOR_FORMAT_NV24_10LE_709 as _,
    NV24_10LE_709_ER =
        nvidia_deepstream_sys::NvBufSurfaceColorFormat_NVBUF_COLOR_FORMAT_NV24_10LE_709_ER as _,
    NV24_10LE_2020 =
        nvidia_deepstream_sys::NvBufSurfaceColorFormat_NVBUF_COLOR_FORMAT_NV24_10LE_2020 as _,
    NV24_12LE_2020 =
        nvidia_deepstream_sys::NvBufSurfaceColorFormat_NVBUF_COLOR_FORMAT_NV24_12LE_2020 as _,
    RGBA_10_10_10_2_709 =
        nvidia_deepstream_sys::NvBufSurfaceColorFormat_NVBUF_COLOR_FORMAT_RGBA_10_10_10_2_709 as _,
    RGBA_10_10_10_2_2020 =
        nvidia_deepstream_sys::NvBufSurfaceColorFormat_NVBUF_COLOR_FORMAT_RGBA_10_10_10_2_2020 as _,
    BGRA_10_10_10_2_709 =
        nvidia_deepstream_sys::NvBufSurfaceColorFormat_NVBUF_COLOR_FORMAT_BGRA_10_10_10_2_709 as _,
    BGRA_10_10_10_2_2020 =
        nvidia_deepstream_sys::NvBufSurfaceColorFormat_NVBUF_COLOR_FORMAT_BGRA_10_10_10_2_2020 as _,
    A32 = nvidia_deepstream_sys::NvBufSurfaceColorFormat_NVBUF_COLOR_FORMAT_A32 as _,
    UYVP = nvidia_deepstream_sys::NvBufSurfaceColorFormat_NVBUF_COLOR_FORMAT_UYVP as _,
    UYVP_ER = nvidia_deepstream_sys::NvBufSurfaceColorFormat_NVBUF_COLOR_FORMAT_UYVP_ER as _,
    LAST = nvidia_deepstream_sys::NvBufSurfaceColorFormat_NVBUF_COLOR_FORMAT_LAST as _,
}

#[repr(u32)]
#[derive(Clone, Copy, PartialEq, Debug)]
pub enum Layout {
    Pitch = nvidia_deepstream_sys::NvBufSurfaceLayout_NVBUF_LAYOUT_PITCH as _,
    BlockLinear = nvidia_deepstream_sys::NvBufSurfaceLayout_NVBUF_LAYOUT_BLOCK_LINEAR as _,
}

#[repr(u32)]
#[derive(Clone, Copy, PartialEq, Debug)]
pub enum MemType {
    Default = nvidia_deepstream_sys::NvBufSurfaceMemType_NVBUF_MEM_DEFAULT as _,
    CUDA_Pinned = nvidia_deepstream_sys::NvBufSurfaceMemType_NVBUF_MEM_CUDA_PINNED as _,
    CUDA_Device = nvidia_deepstream_sys::NvBufSurfaceMemType_NVBUF_MEM_CUDA_DEVICE as _,
    CUDA_Unified = nvidia_deepstream_sys::NvBufSurfaceMemType_NVBUF_MEM_CUDA_UNIFIED as _,
    SurfaceArray = nvidia_deepstream_sys::NvBufSurfaceMemType_NVBUF_MEM_SURFACE_ARRAY as _,
    Handle = nvidia_deepstream_sys::NvBufSurfaceMemType_NVBUF_MEM_HANDLE as _,
    System = nvidia_deepstream_sys::NvBufSurfaceMemType_NVBUF_MEM_SYSTEM as _,
}

#[repr(u32)]
#[derive(Clone, Copy, PartialEq, Debug)]
pub enum DisplayScanFormat {
    Progressive = nvidia_deepstream_sys::NvBufSurfaceDisplayScanFormat_NVBUF_DISPLAYSCANFORMAT_PROGRESSIVE,
    Interlaced = nvidia_deepstream_sys::NvBufSurfaceDisplayScanFormat_NVBUF_DISPLAYSCANFORMAT_INTERLACED,
}

crate::wrapper_impl!(PlaneParamsEx, nvidia_deepstream_sys::NvBufSurfacePlaneParamsEx);
