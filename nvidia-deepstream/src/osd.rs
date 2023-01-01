use crate::WrapperExt;
use gstreamer::glib;
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

crate::wrapper_impl!(ColorParams, nvidia_deepstream_sys::NvOSD_ColorParams);

impl ColorParams {
    pub fn new(red: f64, green: f64, blue: f64, alpha: f64) -> ColorParams {
        ColorParams::from_native_type(nvidia_deepstream_sys::NvOSD_ColorParams {
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

    pub fn black() -> ColorParams { ColorParams::new(0.0, 0.0, 0.0, 1.0) }

    pub fn white() -> ColorParams { ColorParams::new(1.0, 1.0, 1.0, 1.0) }
}

crate::wrapper_impl!(FontParams, nvidia_deepstream_sys::NvOSD_FontParams);

impl FontParams {
    pub fn font_name(&self) -> &CStr {
        unsafe {
            CStr::from_ptr(self.as_native_type_ref().font_name)
        }
    }

    pub fn font_size(&self) -> u32 {
        self.as_native_type_ref().font_size
    }

    pub fn font_color(&self) -> &ColorParams {
        ColorParams::from_native_type_ref(&self.as_native_type_ref().font_color)
    }
}

pub struct FontParamsBuilder {
    font_name: Option<&'static CStr>,
    font_size: Option<u32>,
    font_color: Option<ColorParams>
}

impl FontParamsBuilder {
    pub fn new() -> FontParamsBuilder {
        FontParamsBuilder {
            font_name: None,
            font_size: None,
            font_color: None,
        }
    }

    pub fn font_name(mut self, name: &'static CStr) -> Self {
        self.font_name = Some(name);
        self
    }

    pub fn font_size(mut self, size: u32) -> Self {
        self.font_size = Some(size);
        self
    }

    pub fn font_color(mut self, color: ColorParams) -> Self {
        self.font_color = Some(color);
        self
    }

    pub fn build(self) -> FontParams {
        FontParams::from_native_type(nvidia_deepstream_sys::NvOSD_FontParams {
            font_name: self.font_name.unwrap_or_default().as_ptr() as _,
            font_size: self.font_size.unwrap_or(8),
            font_color: *self.font_color.unwrap_or(ColorParams::black()).as_native_type_ref(),
        })
    }
}

crate::wrapper_impl!(TextParams, nvidia_deepstream_sys::NvOSD_TextParams);

impl TextParams {
    pub fn display_text(&self) -> &CStr {
        unsafe {
            CStr::from_ptr(self.as_native_type_ref().display_text)
        }
    }

    pub fn x_offset(&self) -> u32 {
        self.as_native_type_ref().x_offset
    }

    pub fn y_offset(&self) -> u32 {
        self.as_native_type_ref().y_offset
    }

    pub fn font_params(&self) -> &FontParams {
        FontParams::from_native_type_ref(&self.as_native_type_ref().font_params)
    }

    pub fn text_bg_clr(&self) -> Option<&ColorParams> {
        if self.as_native_type_ref().set_bg_clr != 0 {
            Some(ColorParams::from_native_type_ref(&self.as_native_type_ref().text_bg_clr))
        } else {
            None
        }
    }
}

pub struct TextParamsBuilder {
    display_text: Option<&'static CStr>,
    x_offset: Option<u32>,
    y_offset: Option<u32>,
    font_params: Option<FontParams>,
    text_bg_clr: Option<ColorParams>
}

impl TextParamsBuilder {
    pub fn new() -> TextParamsBuilder {
        TextParamsBuilder {
            display_text: None,
            x_offset: None,
            y_offset: None,
            font_params: None,
            text_bg_clr: None,
        }
    }

    pub fn display_text(mut self, text: &'static CStr) -> Self {
        self.display_text = Some(text);
        self
    }

    pub fn x_offset(mut self, offset: u32) -> Self {
        self.x_offset = Some(offset);
        self
    }

    pub fn y_offset(mut self, offset: u32) -> Self {
        self.y_offset = Some(offset);
        self
    }

    pub fn font_params(mut self, params: FontParams) -> Self {
        self.font_params = Some(params);
        self
    }

    pub fn text_bg_clr(mut self, params: ColorParams) -> Self {
        self.text_bg_clr = Some(params);
        self
    }

    pub fn build(self) -> TextParams {
        TextParams::from_native_type(nvidia_deepstream_sys::NvOSD_TextParams {
            display_text: self.display_text.unwrap_or_default().as_ptr() as _,
            x_offset: self.x_offset.unwrap_or_default(),
            y_offset: self.y_offset.unwrap_or_default(),
            font_params: self.font_params.unwrap_or_default().as_native_type(),
            set_bg_clr: if self.text_bg_clr.is_some() { 1 } else { 0},
            text_bg_clr: self.text_bg_clr.unwrap_or_default().as_native_type(),
        })
    }
}

crate::wrapper_impl!(ColorInfo, nvidia_deepstream_sys::NvOSD_Color_info);

impl ColorInfo {
    pub fn new(id: i32, color: ColorParams) -> ColorInfo {
        ColorInfo::from_native_type(nvidia_deepstream_sys::NvOSD_Color_info {
            id: id, color: color.as_native_type()
        })
    }

    pub fn id(&self) -> i32 {
        self.as_native_type_ref().id
    }

    pub fn color(&self) -> &ColorParams {
        ColorParams::from_native_type_ref(&self.as_native_type_ref().color)
    }
}

crate::wrapper_impl!(RectParams, nvidia_deepstream_sys::_NvOSD_RectParams);

impl RectParams {
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

    pub fn border_width(&self) -> u32 {
        self.as_native_type_ref().border_width
    }

    pub fn border_color(&self) -> &ColorParams {
        ColorParams::from_native_type_ref(&self.as_native_type_ref().border_color)
    }

    pub fn bg_color(&self) -> Option<&ColorParams> {
        if self.as_native_type_ref().has_bg_color != 0 {
            Some(ColorParams::from_native_type_ref(
                &self.as_native_type_ref().bg_color,
            ))
        } else {
            None
        }
    }

    pub fn color_id(&self) -> Option<i32> {
        if self.as_native_type_ref().has_color_info != 0 {
            Some(self.as_native_type_ref().color_id)
        } else {
            None
        }
    }
}

crate::wrapper_impl!(MaskParams, nvidia_deepstream_sys::NvOSD_MaskParams);

impl MaskParams {
    pub fn data(&self) -> &[f32] {
        if self.as_native_type_ref().data != std::ptr::null_mut()
            && self.as_native_type_ref().size > 0
        {
            unsafe {
                std::slice::from_raw_parts(
                    self.as_native_type_ref().data,
                    self.as_native_type_ref().size as _,
                )
            }
        } else {
            &[]
        }
    }

    pub fn threshold(&self) -> f32 {
        self.as_native_type_ref().threshold
    }

    pub fn width(&self) -> u32 {
        self.as_native_type_ref().width
    }

    pub fn height(&self) -> u32 {
        self.as_native_type_ref().height
    }
}

crate::wrapper_impl!(LineParams, nvidia_deepstream_sys::NvOSD_LineParams);

impl LineParams {
    pub fn x1(&self) -> u32 {
        self.as_native_type_ref().x1
    }

    pub fn y1(&self) -> u32 {
        self.as_native_type_ref().y1
    }

    pub fn x2(&self) -> u32 {
        self.as_native_type_ref().x2
    }

    pub fn y2(&self) -> u32 {
        self.as_native_type_ref().y2
    }

    pub fn line_width(&self) -> u32 {
        self.as_native_type_ref().line_width
    }

    pub fn line_color(&self) -> &ColorParams {
        ColorParams::from_native_type_ref(&self.as_native_type_ref().line_color)
    }
}

crate::wrapper_impl!(ArrowParams, nvidia_deepstream_sys::NvOSD_ArrowParams);

impl ArrowParams {
    pub fn x1(&self) -> u32 {
        self.as_native_type_ref().x1
    }

    pub fn y1(&self) -> u32 {
        self.as_native_type_ref().y1
    }

    pub fn x2(&self) -> u32 {
        self.as_native_type_ref().x2
    }

    pub fn y2(&self) -> u32 {
        self.as_native_type_ref().y2
    }

    pub fn arrow_width(&self) -> u32 {
        self.as_native_type_ref().arrow_width
    }

    pub fn arrow_head(&self) -> ArrowHeadDirection {
        unsafe { std::mem::transmute(self.as_native_type_ref().arrow_head) }
    }

    pub fn arrow_color(&self) -> &ColorParams {
        ColorParams::from_native_type_ref(&self.as_native_type_ref().arrow_color)
    }
}

crate::wrapper_impl!(CircleParams, nvidia_deepstream_sys::NvOSD_CircleParams);

impl CircleParams {
    pub fn xc(&self) -> u32 {
        self.as_native_type_ref().xc
    }

    pub fn yc(&self) -> u32 {
        self.as_native_type_ref().yc
    }

    pub fn radius(&self) -> u32 {
        self.as_native_type_ref().radius
    }

    pub fn circle_color(&self) -> &ColorParams {
        ColorParams::from_native_type_ref(&self.as_native_type_ref().circle_color)
    }

    pub fn bg_color(&self) -> Option<&ColorParams> {
        if self.as_native_type_ref().has_bg_color != 0 {
            Some(ColorParams::from_native_type_ref(
                &self.as_native_type_ref().bg_color,
            ))
        } else {
            None
        }
    }
}
