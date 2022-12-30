pub struct Coords(nvidia_deepstream_sys::NvBbox_Coords);

impl Coords {
    pub fn new(left: f32, top: f32, width: f32, height: f32) -> Coords {
        Coords(nvidia_deepstream_sys::NvBbox_Coords { left, top, width, height })
    }

    pub fn left(&self) -> f32 { self.0.left }
    pub fn top(&self) -> f32 { self.0.top }
    pub fn width(&self) -> f32 { self.0.width }
    pub fn height(&self) -> f32 { self.0.height }
}

pub struct Info(nvidia_deepstream_sys::NvDsComp_BboxInfo);

impl Info {
    pub fn org_bbox_coords(&self) -> &Coords {
        unsafe { std::mem::transmute(&self.0.org_bbox_coords) }
    }
}