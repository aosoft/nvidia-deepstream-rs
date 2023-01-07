#![allow(non_camel_case_types)]

use crate::WrapperExt;

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

pub struct PlaneParamEx {
    pub scanformat: DisplayScanFormat,
    pub secondfieldoffset: u32,
    pub blockheightlog2: u32,
    pub physicaladdress: u32,
    pub flags: u64,
}

impl PlaneParamsEx {
    pub fn param(&self, index: usize) -> Option<PlaneParamEx> {
        let p = self.as_native_type_ref();
        if index < p.scanformat.len() {
            Some(PlaneParamEx {
                scanformat: unsafe { std::mem::transmute(p.scanformat[index]) },
                secondfieldoffset: p.secondfieldoffset[index],
                blockheightlog2: p.blockheightlog2[index],
                physicaladdress: p.physicaladdress[index],
                flags: p.flags[index]
            })
        } else {
            None
        }
    }
}

crate::wrapper_impl!(PlaneParams, nvidia_deepstream_sys::NvBufSurfacePlaneParams);

pub struct PlaneParam {
    pub width: u32,
    pub height: u32,
    pub pitch: u32,
    pub offset: u32,
    pub psize: u32,
    pub bytes_per_pix: u32,
}

impl PlaneParams {
    pub fn param(&self, index: usize) -> Option<PlaneParam> {
        let p = self.as_native_type_ref();
        if index < p.num_planes as usize {
            Some(PlaneParam {
                width: p.width[index],
                height: p.height[index],
                pitch: p.pitch[index],
                offset: p.offset[index],
                psize: p.psize[index],
                bytes_per_pix: p.bytesPerPix[index],
            })
        } else {
            None
        }
    }
}

crate::wrapper_impl!(ChromaSubsamplingParams, nvidia_deepstream_sys::NvBufSurfaceChromaSubsamplingParams);

impl ChromaSubsamplingParams {
    pub fn chroma_loc_horiz(&self) -> u8 {
        self.as_native_type_ref().chromaLocHoriz
    }

    pub fn chroma_loc_vert(&self) -> u8 {
        self.as_native_type_ref().chromaLocVert
    }
}


crate::wrapper_impl!(CreateParams, nvidia_deepstream_sys::NvBufSurfaceCreateParams);

impl CreateParams {
    pub fn gpu_id(&self) -> u32 {
        self.as_native_type_ref().gpuId
    }

    pub fn width(&self) -> u32 {
        self.as_native_type_ref().width
    }

    pub fn height(&self) -> u32 {
        self.as_native_type_ref().height
    }

    pub fn size(&self) -> u32 {
        self.as_native_type_ref().size
    }

    pub fn is_contiguous(&self) -> bool {
        self.as_native_type_ref().isContiguous
    }

    pub fn color_format(&self) -> ColorFormat {
        unsafe { std::mem::transmute(self.as_native_type_ref().colorFormat) }
    }

    pub fn layout(&self) -> Layout {
        unsafe { std::mem::transmute(self.as_native_type_ref().layout) }
    }

    pub fn mem_type(&self) -> MemType {
        unsafe { std::mem::transmute(self.as_native_type_ref().memType) }
    }
}


crate::wrapper_impl!(AllocateParams, nvidia_deepstream_sys::NvBufSurfaceAllocateParams);

impl AllocateParams {
    pub fn params(&self) -> &CreateParams {
        CreateParams::from_native_type_ref(&self.as_native_type_ref().params)
    }

    pub fn displayscanformat(&self) -> DisplayScanFormat {
        unsafe { std::mem::transmute(self.as_native_type_ref().displayscanformat) }
    }

    pub fn chroma_subsampling(&self) -> ChromaSubsamplingParams {
        unsafe { std::mem::transmute(self.as_native_type_ref().chromaSubsampling) }
    }

    pub fn memtag(&self) -> Tag {
        unsafe { std::mem::transmute(self.as_native_type_ref().memtag) }
    }
}


crate::wrapper_impl!(MappedAddr, nvidia_deepstream_sys::NvBufSurfaceMappedAddr);

impl MappedAddr {
    pub fn addr(&self, index: usize) -> Option<*mut ()> {
        if index < self.as_native_type_ref().addr.len() {
            Some(self.as_native_type_ref().addr[index] as _)
        } else {
            None
        }
    }

    pub fn egl_image(&self) -> *mut () {
        self.as_native_type_ref().eglImage as _
    }
}


crate::wrapper_impl!(SurfaceParamsEx, nvidia_deepstream_sys::NvBufSurfaceParamsEx);

impl SurfaceParamsEx {
    pub fn start_of_valid_data(&self) -> i32 {
        self.as_native_type_ref().startofvaliddata
    }

    pub fn size_of_valid_data_in_bytes(&self) -> i32 {
        self.as_native_type_ref().sizeofvaliddatainbytes
    }

    pub fn chroma_subsampling(&self) -> &ChromaSubsamplingParams {
        ChromaSubsamplingParams::from_native_type_ref(&self.as_native_type_ref().chromaSubsampling)
    }

    pub fn is_protected(&self) -> bool {
        self.as_native_type_ref().is_protected
    }

    pub fn plane_params_ex(&self) -> &PlaneParamsEx {
        PlaneParamsEx::from_native_type_ref(&self.as_native_type_ref().planeParamsex)
    }
}

crate::wrapper_impl!(SurfaceParams, nvidia_deepstream_sys::NvBufSurfaceParams);

impl SurfaceParams {
    pub fn width(&self) -> u32 {
        self.as_native_type_ref().width
    }

    pub fn height(&self) -> u32 {
        self.as_native_type_ref().height
    }

    pub fn pitch(&self) -> u32 {
        self.as_native_type_ref().pitch
    }

    pub fn color_format(&self) -> ColorFormat {
        unsafe { std::mem::transmute(self.as_native_type_ref().colorFormat) }
    }

    pub fn layout(&self) -> Layout {
        unsafe { std::mem::transmute(self.as_native_type_ref().layout) }
    }

    pub fn buffer_desc(&self) -> u64 {
        self.as_native_type_ref().bufferDesc
    }

    pub fn data_size(&self) -> u32 {
        self.as_native_type_ref().dataSize
    }

    pub fn data_ptr(&self) -> *mut () {
        self.as_native_type_ref().dataPtr as _
    }

    pub fn plane_params(&self) -> &PlaneParams {
        PlaneParams::from_native_type_ref(&self.as_native_type_ref().planeParams)
    }

    pub fn mapped_addr(&self) -> &MappedAddr {
        MappedAddr::from_native_type_ref(&self.as_native_type_ref().mappedAddr)
    }

    pub fn paramex(&self) -> Option<&SurfaceParamsEx> {
        let p = self.as_native_type_ref().paramex;
        if p != std::ptr::null_mut() {
            Some(SurfaceParamsEx::from_native_type_ref(unsafe { &*p }))
        } else {
            None
        }
    }
}
