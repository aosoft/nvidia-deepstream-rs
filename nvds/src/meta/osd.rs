use crate::WrapperExt;
use gstreamer::glib;
use gstreamer::glib::translate::ToGlibPtr;
use std::ptr::null_mut;
use gstreamer::glib::{GStr, GString};

#[repr(u32)]
#[derive(Default, Clone, Copy, PartialEq, Debug)]
pub enum Mode {
    #[default]
    CPU = nvidia_deepstream_sys::NvOSD_Mode_MODE_CPU as _,
    GPU = nvidia_deepstream_sys::NvOSD_Mode_MODE_GPU as _,
    Hardware = nvidia_deepstream_sys::NvOSD_Mode_MODE_HW as _,
}

#[repr(u32)]
#[derive(Default, Clone, Copy, PartialEq, Debug)]
pub enum ArrowHeadDirection {
    #[default]
    Start = nvidia_deepstream_sys::NvOSD_Arrow_Head_Direction_START_HEAD as _,
    End = nvidia_deepstream_sys::NvOSD_Arrow_Head_Direction_END_HEAD as _,
    Both = nvidia_deepstream_sys::NvOSD_Arrow_Head_Direction_BOTH_HEAD as _,
}

crate::wrapper_impl_value_type!(ColorParams, nvidia_deepstream_sys::NvOSD_ColorParams);

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

    pub fn black() -> ColorParams {
        ColorParams::new(0.0, 0.0, 0.0, 1.0)
    }

    pub fn white() -> ColorParams {
        ColorParams::new(1.0, 1.0, 1.0, 1.0)
    }
}

crate::wrapper_impl_ref_type!(FontParams, nvidia_deepstream_sys::NvOSD_FontParams);

impl FontParams {
    pub fn font_name(&self) -> &GStr {
        unsafe { GStr::from_ptr(self.as_native_type_ref().font_name) }
    }

    pub fn font_size(&self) -> u32 {
        self.as_native_type_ref().font_size
    }

    pub fn font_color(&self) -> &ColorParams {
        ColorParams::from_native_type_ref(&self.as_native_type_ref().font_color)
    }
}

pub struct FontParamsBuilder<'a> {
    font_name: Option<&'a GStr>,
    font_size: Option<u32>,
    font_color: Option<ColorParams>,
}

impl<'a> FontParamsBuilder<'a> {
    pub fn new() -> Self {
        FontParamsBuilder {
            font_name: None,
            font_size: None,
            font_color: None,
        }
    }

    pub fn font_name(mut self, name: &'a GStr) -> Self {
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
            font_color: *self
                .font_color
                .unwrap_or(ColorParams::black())
                .as_native_type_ref(),
        })
    }
}

crate::wrapper_impl_ref_type!(TextParams, nvidia_deepstream_sys::NvOSD_TextParams);

impl TextParams {
    pub fn display_text(&self) -> &GStr {
        unsafe { GStr::from_ptr(self.as_native_type_ref().display_text) }
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
            Some(ColorParams::from_native_type_ref(
                &self.as_native_type_ref().text_bg_clr,
            ))
        } else {
            None
        }
    }

    pub fn to_glib_full(mut self) -> nvidia_deepstream_sys::NvOSD_TextParams {
        let ret = *self.as_native_type_ref();
        self.as_native_type_mut().display_text = null_mut();
        ret
    }

    pub(crate) fn drop_ref(&mut self) {
        unsafe {
            glib::ffi::g_free(self.as_native_type_ref().display_text as _);
        }
    }
}

impl Drop for TextParams {
    fn drop(&mut self) {
        self.drop_ref();
    }
}

pub struct TextParamsBuilder {
    display_text: Option<GString>,
    x_offset: Option<u32>,
    y_offset: Option<u32>,
    font_params: Option<FontParams>,
    text_bg_clr: Option<ColorParams>,
}

impl TextParamsBuilder {
    pub fn new() -> Self {
        TextParamsBuilder {
            display_text: None,
            x_offset: None,
            y_offset: None,
            font_params: None,
            text_bg_clr: None,
        }
    }

    pub fn display_text(mut self, text: GString) -> Self {
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

    /// Build TextParams.
    pub fn build(self) -> TextParams {
        TextParams::from_native_type(nvidia_deepstream_sys::NvOSD_TextParams {
            display_text: if let Some(text) = self.display_text {
                text.to_glib_full()
            } else {
                std::ptr::null_mut()
            } as _,
            x_offset: self.x_offset.unwrap_or_default(),
            y_offset: self.y_offset.unwrap_or_default(),
            font_params: self.font_params.unwrap_or_default().as_native_type(),
            set_bg_clr: if self.text_bg_clr.is_some() { 1 } else { 0 },
            text_bg_clr: self.text_bg_clr.unwrap_or_default().as_native_type(),
        })
    }
}

crate::wrapper_impl_value_type!(ColorInfo, nvidia_deepstream_sys::NvOSD_Color_info);

impl ColorInfo {
    pub fn new(id: i32, color: ColorParams) -> ColorInfo {
        ColorInfo::from_native_type(nvidia_deepstream_sys::NvOSD_Color_info {
            id: id,
            color: color.as_native_type(),
        })
    }

    pub fn id(&self) -> i32 {
        self.as_native_type_ref().id
    }

    pub fn color(&self) -> &ColorParams {
        ColorParams::from_native_type_ref(&self.as_native_type_ref().color)
    }
}

crate::wrapper_impl_ref_type!(RectParams, nvidia_deepstream_sys::_NvOSD_RectParams);

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

pub struct RectParamsBuilder {
    pub left: Option<f32>,
    pub top: Option<f32>,
    pub width: Option<f32>,
    pub height: Option<f32>,
    pub border_width: Option<u32>,
    pub border_color: Option<ColorParams>,
    pub bg_color: Option<ColorParams>,
    pub color_id: Option<i32>,
}

impl RectParamsBuilder {
    pub fn new() -> Self {
        RectParamsBuilder {
            left: None,
            top: None,
            width: None,
            height: None,
            border_width: None,
            border_color: None,
            bg_color: None,
            color_id: None,
        }
    }

    pub fn left(mut self, left: f32) -> Self {
        self.left = Some(left);
        self
    }

    pub fn top(mut self, top: f32) -> Self {
        self.top = Some(top);
        self
    }

    pub fn width(mut self, width: f32) -> Self {
        self.width = Some(width);
        self
    }

    pub fn height(mut self, height: f32) -> Self {
        self.height = Some(height);
        self
    }

    pub fn border_width(mut self, width: u32) -> Self {
        self.border_width = Some(width);
        self
    }

    pub fn border_color(mut self, params: ColorParams) -> Self {
        self.border_color = Some(params);
        self
    }

    pub fn bg_color(mut self, params: Option<ColorParams>) -> Self {
        self.bg_color = params;
        self
    }

    pub fn color_id(mut self, id: Option<i32>) -> Self {
        self.color_id = id;
        self
    }

    pub fn build(self) -> RectParams {
        RectParams::from_native_type(nvidia_deepstream_sys::_NvOSD_RectParams {
            left: self.left.unwrap_or_default(),
            top: self.top.unwrap_or_default(),
            width: self.width.unwrap_or_default(),
            height: self.height.unwrap_or_default(),
            border_width: self.border_width.unwrap_or_default(),
            border_color: self.border_color.unwrap_or_default().as_native_type(),
            has_bg_color: if self.bg_color.is_some() { 1 } else { 0 },
            reserved: 0,
            bg_color: self.bg_color.unwrap_or_default().as_native_type(),
            has_color_info: if self.color_id.is_some() { 1 } else { 0 },
            color_id: self.color_id.unwrap_or_default(),
        })
    }
}

crate::wrapper_impl_ref_type!(MaskParams, nvidia_deepstream_sys::NvOSD_MaskParams);

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

pub struct MaskParamsBuilder {
    data: Option<&'static [f32]>,
    threshold: Option<f32>,
    width: Option<u32>,
    height: Option<u32>,
}

impl MaskParamsBuilder {
    pub fn new() -> Self {
        MaskParamsBuilder {
            data: None,
            threshold: None,
            width: None,
            height: None,
        }
    }

    pub fn data(mut self, data: &'static [f32]) -> Self {
        self.data = Some(data);
        self
    }

    pub fn threshold(mut self, threshold: f32) -> Self {
        self.threshold = Some(threshold);
        self
    }

    pub fn width(mut self, width: u32) -> Self {
        self.width = Some(width);
        self
    }

    pub fn height(mut self, height: u32) -> Self {
        self.height = Some(height);
        self
    }

    pub fn build(self) -> MaskParams {
        MaskParams::from_native_type(nvidia_deepstream_sys::NvOSD_MaskParams {
            data: self.data.map_or(std::ptr::null_mut(), |v| v.as_ptr() as _),
            size: self.data.map_or(0, |v| v.len() as _),
            threshold: self.threshold.unwrap_or_default(),
            width: self.width.unwrap_or_default(),
            height: self.height.unwrap_or_default(),
        })
    }
}

crate::wrapper_impl_ref_type!(LineParams, nvidia_deepstream_sys::NvOSD_LineParams);

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

pub struct LineParamsBuilder {
    x1: Option<u32>,
    y1: Option<u32>,
    x2: Option<u32>,
    y2: Option<u32>,
    line_width: Option<u32>,
    line_color: Option<ColorParams>,
}

impl LineParamsBuilder {
    pub fn new() -> Self {
        LineParamsBuilder {
            x1: None,
            y1: None,
            x2: None,
            y2: None,
            line_width: None,
            line_color: None,
        }
    }

    pub fn x1(mut self, value: u32) -> Self {
        self.x1 = Some(value);
        self
    }

    pub fn y1(mut self, value: u32) -> Self {
        self.y1 = Some(value);
        self
    }

    pub fn x2(mut self, value: u32) -> Self {
        self.x2 = Some(value);
        self
    }

    pub fn y2(mut self, value: u32) -> Self {
        self.y2 = Some(value);
        self
    }

    pub fn line_width(mut self, width: u32) -> Self {
        self.line_width = Some(width);
        self
    }

    pub fn line_color(mut self, color: ColorParams) -> Self {
        self.line_color = Some(color);
        self
    }

    pub fn build(self) -> LineParams {
        LineParams::from_native_type(nvidia_deepstream_sys::NvOSD_LineParams {
            x1: self.x1.unwrap_or_default(),
            y1: self.y1.unwrap_or_default(),
            x2: self.x2.unwrap_or_default(),
            y2: self.y2.unwrap_or_default(),
            line_width: self.line_width.unwrap_or_default(),
            line_color: self
                .line_color
                .unwrap_or(ColorParams::white())
                .as_native_type(),
        })
    }
}

crate::wrapper_impl_ref_type!(ArrowParams, nvidia_deepstream_sys::NvOSD_ArrowParams);

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

pub struct ArrowParamsBuilder {
    x1: Option<u32>,
    y1: Option<u32>,
    x2: Option<u32>,
    y2: Option<u32>,
    arrow_width: Option<u32>,
    arrow_head: Option<ArrowHeadDirection>,
    arrow_color: Option<ColorParams>,
}

impl ArrowParamsBuilder {
    pub fn new() -> Self {
        ArrowParamsBuilder {
            x1: None,
            y1: None,
            x2: None,
            y2: None,
            arrow_width: None,
            arrow_head: None,
            arrow_color: None,
        }
    }

    pub fn x1(mut self, value: u32) -> Self {
        self.x1 = Some(value);
        self
    }

    pub fn y1(mut self, value: u32) -> Self {
        self.y1 = Some(value);
        self
    }

    pub fn x2(mut self, value: u32) -> Self {
        self.x2 = Some(value);
        self
    }

    pub fn y2(mut self, value: u32) -> Self {
        self.y2 = Some(value);
        self
    }

    pub fn arrow_width(mut self, width: u32) -> Self {
        self.arrow_width = Some(width);
        self
    }

    pub fn arrow_head(mut self, head: ArrowHeadDirection) -> Self {
        self.arrow_head = Some(head);
        self
    }

    pub fn arrow_color(mut self, color: ColorParams) -> Self {
        self.arrow_color = Some(color);
        self
    }

    pub fn build(self) -> ArrowParams {
        ArrowParams::from_native_type(nvidia_deepstream_sys::NvOSD_ArrowParams {
            x1: self.x1.unwrap_or_default(),
            y1: self.y1.unwrap_or_default(),
            x2: self.x2.unwrap_or_default(),
            y2: self.y2.unwrap_or_default(),
            arrow_width: self.arrow_width.unwrap_or_default(),
            arrow_head: unsafe { std::mem::transmute(self.arrow_head.unwrap_or_default()) },
            arrow_color: self
                .arrow_color
                .unwrap_or(ColorParams::white())
                .as_native_type(),
            reserved: 0,
        })
    }
}

crate::wrapper_impl_ref_type!(CircleParams, nvidia_deepstream_sys::NvOSD_CircleParams);

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

pub struct CircleParamsBuilder {
    xc: Option<u32>,
    yc: Option<u32>,
    radius: Option<u32>,
    circle_color: Option<ColorParams>,
    bg_color: Option<ColorParams>,
}

impl CircleParamsBuilder {
    pub fn new() -> Self {
        CircleParamsBuilder {
            xc: None,
            yc: None,
            radius: None,
            circle_color: None,
            bg_color: None,
        }
    }

    pub fn xc(mut self, value: u32) -> Self {
        self.xc = Some(value);
        self
    }

    pub fn yc(mut self, value: u32) -> Self {
        self.yc = Some(value);
        self
    }

    pub fn radius(mut self, radius: u32) -> Self {
        self.radius = Some(radius);
        self
    }

    pub fn circle_color(mut self, color: ColorParams) -> Self {
        self.circle_color = Some(color);
        self
    }

    pub fn bg_color(mut self, color: Option<ColorParams>) -> Self {
        self.bg_color = color;
        self
    }

    pub fn build(self) -> CircleParams {
        CircleParams::from_native_type(nvidia_deepstream_sys::NvOSD_CircleParams {
            xc: self.xc.unwrap_or_default(),
            yc: self.yc.unwrap_or_default(),
            radius: self.radius.unwrap_or_default(),
            circle_color: self
                .circle_color
                .unwrap_or(ColorParams::white())
                .as_native_type(),
            has_bg_color: if self.bg_color.is_some() { 1 } else { 0 },
            bg_color: self.bg_color.unwrap_or_default().as_native_type(),
            reserved: 0,
        })
    }
}
