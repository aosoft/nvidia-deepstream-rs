use crate::to_wrapper_ref;
use std::ffi::CStr;

#[repr(u32)]
pub enum Mode {
    CPU = nvidia_deepstream_sys::NvOSD_Mode_MODE_CPU as _,
    GPU = nvidia_deepstream_sys::NvOSD_Mode_MODE_GPU as _,
    Hardware = nvidia_deepstream_sys::NvOSD_Mode_MODE_HW as _,
}

#[repr(u32)]
pub enum ArrowHeadDirection {
    Start = nvidia_deepstream_sys::NvOSD_Arrow_Head_Direction_START_HEAD as _,
    End = nvidia_deepstream_sys::NvOSD_Arrow_Head_Direction_END_HEAD as _,
    Both = nvidia_deepstream_sys::NvOSD_Arrow_Head_Direction_BOTH_HEAD as _,
}

pub struct ColorParams(nvidia_deepstream_sys::NvOSD_ColorParams);

impl ColorParams {
    pub fn red(&self) -> f64 {
        self.0.red
    }

    pub fn green(&self) -> f64 {
        self.0.green
    }

    pub fn blue(&self) -> f64 {
        self.0.blue
    }

    pub fn alpha(&self) -> f64 {
        self.0.alpha
    }
}

pub struct FontParams(nvidia_deepstream_sys::NvOSD_FontParams);

impl FontParams {
    pub fn font_name(&self) -> &str {
        unsafe {
            CStr::from_ptr(self.0.font_name)
                .to_str()
                .unwrap_or_default()
        }
    }

    pub fn font_size(&self) -> u32 {
        self.0.font_size
    }

    pub fn font_color(&self) -> &ColorParams {
        crate::to_wrapper_ref(&self.0.font_color)
    }
}

pub struct TextParams(nvidia_deepstream_sys::NvOSD_TextParams);

impl TextParams {
    pub fn display_text(&self) -> &str {
        unsafe {
            CStr::from_ptr(self.0.display_text)
                .to_str()
                .unwrap_or_default()
        }
    }

    pub fn x_offset(&self) -> u32 {
        self.0.x_offset
    }

    pub fn y_offset(&self) -> u32 {
        self.0.y_offset
    }

    pub fn font_params(&self) -> &FontParams {
        crate::to_wrapper_ref(&self.0.font_params)
    }

    pub fn set_bg_clr(&self) -> i32 {
        self.0.set_bg_clr
    }

    pub fn text_bg_clr(&self) -> &ColorParams {
        crate::to_wrapper_ref(&self.0.text_bg_clr)
    }
}

pub struct ColorInfo(nvidia_deepstream_sys::NvOSD_Color_info);

impl ColorInfo {
    pub fn id(&self) -> i32 {
        self.0.id
    }

    pub fn color(&self) -> &ColorParams {
        crate::to_wrapper_ref(&self.0.color)
    }
}

pub struct RectParams(nvidia_deepstream_sys::_NvOSD_RectParams);

impl RectParams {
    pub fn left(&self) -> f32 {
        self.0.left
    }

    pub fn top(&self) -> f32 {
        self.0.top
    }

    pub fn width(&self) -> f32 {
        self.0.width
    }

    pub fn height(&self) -> f32 {
        self.0.height
    }

    pub fn border_width(&self) -> u32 {
        self.0.border_width
    }

    pub fn border_color(&self) -> &ColorParams {
        to_wrapper_ref(&self.0.border_color)
    }

    pub fn bg_color(&self) -> Option<&ColorParams> {
        if self.0.has_bg_color != 0 {
            Some(to_wrapper_ref(&self.0.bg_color))
        } else {
            None
        }
    }

    pub fn color_id(&self) -> Option<i32> {
        if self.0.has_color_info != 0 {
            Some(self.0.color_id)
        } else {
            None
        }
    }
}

pub struct MaskParams(nvidia_deepstream_sys::NvOSD_MaskParams);

impl MaskParams {
    pub fn data(&self) -> &[f32] {
        if self.0.data != std::ptr::null_mut() && self.0.size > 0 {
            unsafe { std::slice::from_raw_parts(self.0.data, self.0.size as _) }
        } else {
            &[]
        }
    }

    pub fn threshold(&self) -> f32 {
        self.0.threshold
    }

    pub fn width(&self) -> u32 {
        self.0.width
    }

    pub fn height(&self) -> u32 {
        self.0.height
    }
}

pub struct LineParams(nvidia_deepstream_sys::NvOSD_LineParams);

impl LineParams {
    pub fn x1(&self) -> u32 {
        self.0.x1
    }

    pub fn y1(&self) -> u32 {
        self.0.y1
    }

    pub fn x2(&self) -> u32 {
        self.0.x2
    }

    pub fn y2(&self) -> u32 {
        self.0.y2
    }

    pub fn line_width(&self) -> u32 {
        self.0.line_width
    }

    pub fn line_color(&self) -> &ColorParams {
        to_wrapper_ref(&self.0.line_color)
    }
}

pub struct ArrowParams(nvidia_deepstream_sys::NvOSD_ArrowParams);

impl ArrowParams {
    pub fn x1(&self) -> u32 {
        self.0.x1
    }

    pub fn y1(&self) -> u32 {
        self.0.y1
    }

    pub fn x2(&self) -> u32 {
        self.0.x2
    }

    pub fn y2(&self) -> u32 {
        self.0.y2
    }

    pub fn arrow_width(&self) -> u32 {
        self.0.arrow_width
    }

    pub fn arrow_head(&self) -> ArrowHeadDirection {
        unsafe { std::mem::transmute(self.0.arrow_head) }
    }

    pub fn arrow_color(&self) -> &ColorParams {
        to_wrapper_ref(&self.0.arrow_color)
    }
}

pub struct CircleParams(nvidia_deepstream_sys::NvOSD_CircleParams);

impl CircleParams {
    pub fn xc(&self) -> u32 {
        self.0.xc
    }

    pub fn yc(&self) -> u32 {
        self.0.yc
    }

    pub fn radius(&self) -> u32 {
        self.0.radius
    }

    pub fn circle_color(&self) -> &ColorParams {
        to_wrapper_ref(&self.0.circle_color)
    }

    pub fn bg_color(&self) -> Option<&ColorParams> {
        if self.0.has_bg_color != 0 {
            Some(to_wrapper_ref(&self.0.bg_color))
        } else {
            None
        }
    }
}
