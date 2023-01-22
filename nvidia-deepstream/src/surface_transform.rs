use crate::WrapperExt;
use std::ptr::NonNull;

#[repr(u32)]
#[derive(Clone, Copy, PartialEq, Debug)]
pub enum Compute {
    Default =
        nvidia_deepstream_sys::NvBufSurfTransform_Compute_NvBufSurfTransformCompute_Default as _,
    GPU = nvidia_deepstream_sys::NvBufSurfTransform_Compute_NvBufSurfTransformCompute_GPU as _,
    VIC = nvidia_deepstream_sys::NvBufSurfTransform_Compute_NvBufSurfTransformCompute_VIC as _,
}

#[repr(u32)]
#[derive(Clone, Copy, PartialEq, Debug)]
pub enum Flip {
    None = nvidia_deepstream_sys::NvBufSurfTransform_Flip_NvBufSurfTransform_None as _,
    Rotate90 = nvidia_deepstream_sys::NvBufSurfTransform_Flip_NvBufSurfTransform_Rotate90 as _,
    Rotate180 = nvidia_deepstream_sys::NvBufSurfTransform_Flip_NvBufSurfTransform_Rotate180 as _,
    Rotate270 = nvidia_deepstream_sys::NvBufSurfTransform_Flip_NvBufSurfTransform_Rotate270 as _,
    FlipX = nvidia_deepstream_sys::NvBufSurfTransform_Flip_NvBufSurfTransform_FlipX as _,
    FlipY = nvidia_deepstream_sys::NvBufSurfTransform_Flip_NvBufSurfTransform_FlipY as _,
    Transpose = nvidia_deepstream_sys::NvBufSurfTransform_Flip_NvBufSurfTransform_Transpose as _,
    InvTranspose =
        nvidia_deepstream_sys::NvBufSurfTransform_Flip_NvBufSurfTransform_InvTranspose as _,
}

#[repr(u32)]
#[derive(Default, Clone, Copy, PartialEq, Debug)]
pub enum Inter {
    #[default]
    Nearest = nvidia_deepstream_sys::NvBufSurfTransform_Inter_NvBufSurfTransformInter_Nearest as _,
    Bilinear =
        nvidia_deepstream_sys::NvBufSurfTransform_Inter_NvBufSurfTransformInter_Bilinear as _,
    Algo1 = nvidia_deepstream_sys::NvBufSurfTransform_Inter_NvBufSurfTransformInter_Algo1 as _,
    Algo2 = nvidia_deepstream_sys::NvBufSurfTransform_Inter_NvBufSurfTransformInter_Algo2 as _,
    Algo3 = nvidia_deepstream_sys::NvBufSurfTransform_Inter_NvBufSurfTransformInter_Algo3 as _,
    Algo4 = nvidia_deepstream_sys::NvBufSurfTransform_Inter_NvBufSurfTransformInter_Algo4 as _,
    Default = nvidia_deepstream_sys::NvBufSurfTransform_Inter_NvBufSurfTransformInter_Default as _,
}

#[repr(i32)]
#[derive(Clone, Copy, PartialEq, Debug)]
pub enum Error {
    RoiError =
        nvidia_deepstream_sys::NvBufSurfTransform_Error_NvBufSurfTransformError_ROI_Error as _,
    InvalidParams =
        nvidia_deepstream_sys::NvBufSurfTransform_Error_NvBufSurfTransformError_Invalid_Params as _,
    ExecutionError =
        nvidia_deepstream_sys::NvBufSurfTransform_Error_NvBufSurfTransformError_Execution_Error
            as _,
    Unsupported =
        nvidia_deepstream_sys::NvBufSurfTransform_Error_NvBufSurfTransformError_Unsupported as _,
    Success = nvidia_deepstream_sys::NvBufSurfTransform_Error_NvBufSurfTransformError_Success as _,
}

#[repr(u32)]
#[derive(Default, Clone, Copy, PartialEq, Debug)]
pub enum TransformFlag {
    #[default]
    None = 0,
    CropSrc =
        nvidia_deepstream_sys::NvBufSurfTransform_Transform_Flag_NVBUFSURF_TRANSFORM_CROP_SRC as _,
    CropDst =
        nvidia_deepstream_sys::NvBufSurfTransform_Transform_Flag_NVBUFSURF_TRANSFORM_CROP_DST as _,
    Filter =
        nvidia_deepstream_sys::NvBufSurfTransform_Transform_Flag_NVBUFSURF_TRANSFORM_FILTER as _,
    Flip = nvidia_deepstream_sys::NvBufSurfTransform_Transform_Flag_NVBUFSURF_TRANSFORM_FLIP as _,
    Normalize =
        nvidia_deepstream_sys::NvBufSurfTransform_Transform_Flag_NVBUFSURF_TRANSFORM_NORMALIZE as _,
}

#[repr(u32)]
#[derive(Default, Clone, Copy, PartialEq, Debug)]
pub enum CompositeFlag {
    #[default]
    None = 0,
    Composite = nvidia_deepstream_sys::NvBufSurfTransform_Composite_Flag_NVBUFSURF_TRANSFORM_COMPOSITE as _,
    Blend = nvidia_deepstream_sys::NvBufSurfTransform_Composite_Flag_NVBUFSURF_TRANSFORM_BLEND as _,
    CompositeFilter = nvidia_deepstream_sys::NvBufSurfTransform_Composite_Flag_NVBUFSURF_TRANSFORM_COMPOSITE_FILTER as _,
}

crate::wrapper_impl!(TransformRect, nvidia_deepstream_sys::NvBufSurfTransformRect);

impl TransformRect {
    pub fn new(left: u32, top: u32, width: u32, height: u32) -> TransformRect {
        TransformRect::from_native_type(nvidia_deepstream_sys::NvBufSurfTransformRect {
            top,
            left,
            width,
            height,
        })
    }

    pub fn top(&self) -> u32 {
        self.as_native_type_ref().top
    }

    pub fn left(&self) -> u32 {
        self.as_native_type_ref().left
    }

    pub fn width(&self) -> u32 {
        self.as_native_type_ref().width
    }

    pub fn height(&self) -> u32 {
        self.as_native_type_ref().height
    }
}

crate::wrapper_impl!(
    ConfigParams,
    nvidia_deepstream_sys::NvBufSurfTransformConfigParams
);

impl ConfigParams {
    pub fn new(compute_mode: Compute, gpu_id: u32, cuda_stream: *mut ()) -> ConfigParams {
        ConfigParams::from_native_type(nvidia_deepstream_sys::NvBufSurfTransformConfigParams {
            compute_mode: compute_mode as _,
            gpu_id: gpu_id as _,
            cuda_stream: cuda_stream as _,
        })
    }

    pub fn compute_mode(&self) -> Compute {
        unsafe { std::mem::transmute(self.as_native_type_ref().compute_mode) }
    }

    pub fn gpu_id(&self) -> u32 {
        self.as_native_type_ref().gpu_id as _
    }

    pub fn cuda_stream(&self) -> *mut () {
        self.as_native_type_ref().cuda_stream as _
    }
}

crate::wrapper_impl!(
    TransformParams,
    nvidia_deepstream_sys::_NvBufSurfaceTransformParams
);

impl TransformParams {
    pub fn transform_flag(&self) -> TransformFlag {
        unsafe { std::mem::transmute(self.as_native_type_ref().transform_flag) }
    }

    pub fn transform_flip(&self) -> Flip {
        unsafe { std::mem::transmute(self.as_native_type_ref().transform_flip) }
    }

    pub fn transform_filter(&self) -> Inter {
        unsafe { std::mem::transmute(self.as_native_type_ref().transform_filter) }
    }

    pub fn src_rect(&self) -> Option<&TransformRect> {
        NonNull::new(self.as_native_type_ref().src_rect)
            .map(|p| unsafe { TransformRect::from_native_type_ref(p.as_ref()) })
    }

    pub fn dst_rect(&self) -> Option<&TransformRect> {
        NonNull::new(self.as_native_type_ref().dst_rect)
            .map(|p| unsafe { TransformRect::from_native_type_ref(p.as_ref()) })
    }
}

pub struct CompositeParams<'a>(
    nvidia_deepstream_sys::NvBufSurfTransformCompositeParams,
    Option<&'a TransformRect>,
    Option<&'a TransformRect>,
);
crate::wrapper_impl_with_lifetime_body!(
    CompositeParams,
    nvidia_deepstream_sys::NvBufSurfTransformCompositeParams
);

impl<'a> CompositeParams<'a> {
    pub fn new(
        composite_flag: CompositeFlag,
        input_buf_count: u32,
        src_comp_rect: Option<&'a TransformRect>,
        dst_comp_rect: Option<&'a TransformRect>,
        composite_filter: Inter,
    ) -> CompositeParams<'a> {
        CompositeParams(
            nvidia_deepstream_sys::NvBufSurfTransformCompositeParams {
                composite_flag: composite_flag as _,
                input_buf_count,
                src_comp_rect: src_comp_rect.map_or(std::ptr::null_mut(), |p| {
                    p.as_native_type_ref() as *const _ as _
                }),
                dst_comp_rect: dst_comp_rect.map_or(std::ptr::null_mut(), |p| {
                    p.as_native_type_ref() as *const _ as _
                }),
                composite_filter: composite_filter as _,
            },
            src_comp_rect,
            dst_comp_rect,
        )
    }

    pub fn composite_flag(&self) -> CompositeFlag {
        unsafe { std::mem::transmute(self.as_native_type_ref().composite_flag) }
    }

    pub fn input_buf_count(&self) -> u32 {
        self.as_native_type_ref().input_buf_count
    }

    pub fn src_comp_rect(&self) -> Option<&TransformRect> {
        NonNull::new(self.as_native_type_ref().src_comp_rect)
            .map(|p| unsafe { TransformRect::from_native_type_ref(p.as_ref()) })
    }

    pub fn dst_comp_rect(&self) -> Option<&TransformRect> {
        NonNull::new(self.as_native_type_ref().dst_comp_rect)
            .map(|p| unsafe { TransformRect::from_native_type_ref(p.as_ref()) })
    }

    pub fn composite_filter(&self) -> Inter {
        unsafe { std::mem::transmute(self.as_native_type_ref().composite_filter) }
    }
}

crate::wrapper_impl!(
    ColorParams,
    nvidia_deepstream_sys::NvBufSurfTransform_ColorParams
);

impl ColorParams {
    pub fn new(red: f64, green: f64, blue: f64, alpha: f64) -> ColorParams {
        ColorParams::from_native_type(nvidia_deepstream_sys::NvBufSurfTransform_ColorParams {
            red,
            green,
            blue,
            alpha,
        })
    }

    pub fn red(&self) -> f64 {
        self.as_native_type_ref().red
    }

    pub fn green(&self) -> f64 {
        self.as_native_type_ref().green
    }

    pub fn blue(&self) -> f64 {
        self.as_native_type_ref().blue
    }

    pub fn alpha(&self) -> f64 {
        self.as_native_type_ref().alpha
    }

    pub fn black() -> ColorParams {
        ColorParams::new(0.0, 0.0, 0.0, 1.0)
    }

    pub fn white() -> ColorParams {
        ColorParams::new(1.0, 1.0, 1.0, 1.0)
    }
}

#[derive(Clone, Copy, PartialEq, Debug)]
pub struct PerformBlendingFlags<'a>(&'a [u32]);

impl PerformBlendingFlags<'_> {
    pub fn len(&self) -> usize {
        self.0.len()
    }

    pub fn flag(&self, index: usize) -> bool {
        if index < self.len() {
            self.0[index] != 0
        } else {
            false
        }
    }
}

crate::wrapper_impl_with_lifetime!(
    CompositeBlendParams,
    nvidia_deepstream_sys::NvBufSurfTransformCompositeBlendParams
);

impl<'a> CompositeBlendParams<'a> {
    pub fn composite_blend_flag(&self) -> CompositeFlag {
        unsafe { std::mem::transmute(self.as_native_type_ref().composite_blend_flag) }
    }

    pub fn input_buf_count(&self) -> u32 {
        self.as_native_type_ref().input_buf_count
    }

    pub fn composite_blend_filter(&self) -> Inter {
        unsafe { std::mem::transmute(self.as_native_type_ref().composite_blend_filter) }
    }

    pub fn color_bg(&self) -> Option<&ColorParams> {
        NonNull::new(self.as_native_type_ref().color_bg)
            .map(|p| unsafe { ColorParams::from_native_type_ref(p.as_ref()) })
    }

    pub fn perform_blending(&self) -> Option<PerformBlendingFlags> {
        NonNull::new(self.as_native_type_ref().perform_blending).map(|p| unsafe {
            PerformBlendingFlags(std::slice::from_raw_parts(
                p.as_ptr(),
                self.input_buf_count() as _,
            ))
        })
    }
}

pub struct CompositeBlendParamsBuilder<'a> {
    composite_blend_flag: Option<CompositeFlag>,
    input_buf_count: Option<u32>,
    composite_blend_filter: Option<Inter>,
    color_bg: Option<&'a ColorParams>,
    perform_blending: Option<&'a [u32]>,
}

impl<'a> CompositeBlendParamsBuilder<'a> {
    pub fn new() -> CompositeBlendParamsBuilder<'a> {
        CompositeBlendParamsBuilder {
            composite_blend_flag: None,
            input_buf_count: None,
            composite_blend_filter: None,
            color_bg: None,
            perform_blending: None,
        }
    }

    pub fn composite_blend_flag(mut self, flag: CompositeFlag) -> Self {
        self.composite_blend_flag = Some(flag);
        self
    }

    pub fn input_buf_count(mut self, count: u32) -> Self {
        self.input_buf_count = Some(count);
        self.perform_blending = None;
        self
    }

    pub fn composite_blend_filter(mut self, filter: Inter) -> Self {
        self.composite_blend_filter = Some(filter);
        self
    }

    pub fn color_bg(mut self, color_bg: Option<&'a ColorParams>) -> Self {
        self.color_bg = color_bg;
        self
    }

    pub fn perform_blending(mut self, perform_blending: Option<PerformBlendingFlags<'a>>) -> Self {
        if let Some(x) = perform_blending {
            self.input_buf_count = Some(x.len() as _);
        }
        self.perform_blending = perform_blending.map(|x| x.0);
        self
    }

    pub fn build(self) -> CompositeBlendParams<'a> {
        CompositeBlendParams(
            nvidia_deepstream_sys::NvBufSurfTransformCompositeBlendParams {
                composite_blend_flag: self.composite_blend_flag.unwrap_or_default() as _,
                input_buf_count: self.input_buf_count.unwrap_or_default(),
                composite_blend_filter: self.composite_blend_filter.unwrap_or_default() as _,
                color_bg: self.color_bg.map_or(std::ptr::null_mut(), |p| {
                    p.as_native_type_ref() as *const _ as _
                }),
                perform_blending: self
                    .perform_blending
                    .map_or_else(|| std::ptr::null_mut(), |x| x.as_ptr() as _),
            },
            Default::default(),
        )
    }
}

crate::wrapper_impl_with_lifetime!(
    CompositeBlendParamsEx,
    nvidia_deepstream_sys::NvBufSurfTransformCompositeBlendParamsEx
);

impl<'a> CompositeBlendParamsEx<'a> {
    pub fn params(&self) -> &CompositeBlendParams {
        CompositeBlendParams::from_native_type_ref(&self.as_native_type_ref().params)
    }

    pub fn src_comp_rect(&self) -> &'a [TransformRect] {
        unsafe {
            std::slice::from_raw_parts(self.as_native_type_ref().src_comp_rect as _, self.params().input_buf_count() as _)
        }
    }

    pub fn dst_comp_rect(&self) -> &'a [TransformRect] {
        unsafe {
            std::slice::from_raw_parts(self.as_native_type_ref().dst_comp_rect as _, self.params().input_buf_count() as _)
        }
    }

    pub fn composite_blend_filter(&self) -> &'a [Inter] {
        unsafe {
            std::slice::from_raw_parts(self.as_native_type_ref().composite_blend_filter as _, self.params().input_buf_count() as _)
        }
    }

    pub fn alpha(&self) -> &'a [f32] {
        unsafe {
            std::slice::from_raw_parts(self.as_native_type_ref().alpha as _, self.params().input_buf_count() as _)
        }
    }
}