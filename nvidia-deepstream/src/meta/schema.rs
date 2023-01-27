use crate::WrapperExt;
use std::ffi::CStr;

#[repr(u32)]
#[derive(Default, Clone, Copy, PartialEq, Debug)]
pub enum EventType {
    #[default]
    Entry = nvidia_deepstream_sys::NvDsEventType_NVDS_EVENT_ENTRY as _,
    Exit = nvidia_deepstream_sys::NvDsEventType_NVDS_EVENT_EXIT as _,
    Moving = nvidia_deepstream_sys::NvDsEventType_NVDS_EVENT_MOVING as _,
    Stopped = nvidia_deepstream_sys::NvDsEventType_NVDS_EVENT_STOPPED as _,
    Empty = nvidia_deepstream_sys::NvDsEventType_NVDS_EVENT_EMPTY as _,
    Parked = nvidia_deepstream_sys::NvDsEventType_NVDS_EVENT_PARKED as _,
    Reset = nvidia_deepstream_sys::NvDsEventType_NVDS_EVENT_RESET as _,
    Reserved = nvidia_deepstream_sys::NvDsEventType_NVDS_EVENT_RESERVED as _,
    Custom = nvidia_deepstream_sys::NvDsEventType_NVDS_EVENT_CUSTOM as _,
}

#[repr(u32)]
#[derive(Default, Clone, Copy, PartialEq, Debug)]
pub enum ObjectType {
    #[default]
    Vehicle = nvidia_deepstream_sys::NvDsObjectType_NVDS_OBJECT_TYPE_VEHICLE as _,
    Person = nvidia_deepstream_sys::NvDsObjectType_NVDS_OBJECT_TYPE_PERSON as _,
    Face = nvidia_deepstream_sys::NvDsObjectType_NVDS_OBJECT_TYPE_FACE as _,
    Bag = nvidia_deepstream_sys::NvDsObjectType_NVDS_OBJECT_TYPE_BAG as _,
    Bicycle = nvidia_deepstream_sys::NvDsObjectType_NVDS_OBJECT_TYPE_BICYCLE as _,
    RoadSign = nvidia_deepstream_sys::NvDsObjectType_NVDS_OBJECT_TYPE_ROADSIGN as _,
    VehicleExt = nvidia_deepstream_sys::NvDsObjectType_NVDS_OBJECT_TYPE_VEHICLE_EXT as _,
    PersonExt = nvidia_deepstream_sys::NvDsObjectType_NVDS_OBJECT_TYPE_PERSON_EXT as _,
    FaceExt = nvidia_deepstream_sys::NvDsObjectType_NVDS_OBJECT_TYPE_FACE_EXT as _,
    Reserved = nvidia_deepstream_sys::NvDsObjectType_NVDS_OBJECT_TYPE_RESERVED as _,
    Custom = nvidia_deepstream_sys::NvDsObjectType_NVDS_OBJECT_TYPE_CUSTOM as _,
    Unknown = nvidia_deepstream_sys::NvDsObjectType_NVDS_OBJECT_TYPE_UNKNOWN as _,
}

#[repr(u32)]
#[derive(Default, Clone, Copy, PartialEq, Debug)]
pub enum PayloadType {
    #[default]
    DeepStream = nvidia_deepstream_sys::NvDsPayloadType_NVDS_PAYLOAD_DEEPSTREAM as _,
    DeepstreamMinimal = nvidia_deepstream_sys::NvDsPayloadType_NVDS_PAYLOAD_DEEPSTREAM_MINIMAL as _,
    Reserved = nvidia_deepstream_sys::NvDsPayloadType_NVDS_PAYLOAD_RESERVED as _,
    Custom = nvidia_deepstream_sys::NvDsPayloadType_NVDS_PAYLOAD_CUSTOM as _,
}

crate::wrapper_impl!(Rect, nvidia_deepstream_sys::NvDsRect);

impl Rect {
    pub fn top(&self) -> f32 {
        self.as_native_type_ref().top
    }

    pub fn left(&self) -> f32 {
        self.as_native_type_ref().left
    }

    pub fn width(&self) -> f32 {
        self.as_native_type_ref().width
    }

    pub fn height(&self) -> f32 {
        self.as_native_type_ref().height
    }
}

crate::wrapper_impl!(GeoLocation, nvidia_deepstream_sys::NvDsGeoLocation);

impl GeoLocation {
    pub fn lat(&self) -> f64 {
        self.as_native_type_ref().lat
    }

    pub fn lon(&self) -> f64 {
        self.as_native_type_ref().lon
    }

    pub fn alt(&self) -> f64 {
        self.as_native_type_ref().alt
    }
}

crate::wrapper_impl!(Coordinate, nvidia_deepstream_sys::NvDsCoordinate);

impl Coordinate {
    pub fn x(&self) -> f64 {
        self.as_native_type_ref().x
    }

    pub fn y(&self) -> f64 {
        self.as_native_type_ref().y
    }

    pub fn z(&self) -> f64 {
        self.as_native_type_ref().z
    }
}

crate::wrapper_impl!(ObjectSignature, nvidia_deepstream_sys::NvDsObjectSignature);

impl ObjectSignature {
    pub fn signature(&self) -> &[f64] {
        unsafe {
            std::slice::from_raw_parts(
                self.as_native_type_ref().signature,
                self.as_native_type_ref().size as _,
            )
        }
    }
}

crate::wrapper_impl!(VehicleObject, nvidia_deepstream_sys::NvDsVehicleObject);

impl VehicleObject {
    pub fn type_(&self) -> &CStr {
        unsafe { CStr::from_ptr(self.as_native_type_ref().type_) }
    }

    pub fn make(&self) -> &CStr {
        unsafe { CStr::from_ptr(self.as_native_type_ref().make) }
    }

    pub fn model(&self) -> &CStr {
        unsafe { CStr::from_ptr(self.as_native_type_ref().model) }
    }

    pub fn color(&self) -> &CStr {
        unsafe { CStr::from_ptr(self.as_native_type_ref().color) }
    }

    pub fn region(&self) -> &CStr {
        unsafe { CStr::from_ptr(self.as_native_type_ref().region) }
    }

    pub fn license(&self) -> &CStr {
        unsafe { CStr::from_ptr(self.as_native_type_ref().license) }
    }
}


crate::wrapper_impl!(PersonObject, nvidia_deepstream_sys::NvDsPersonObject);

impl PersonObject {
    pub fn gender(&self) -> &CStr {
        unsafe { CStr::from_ptr(self.as_native_type_ref().gender) }
    }

    pub fn hair(&self) -> &CStr {
        unsafe { CStr::from_ptr(self.as_native_type_ref().hair) }
    }

    pub fn cap(&self) -> &CStr {
        unsafe { CStr::from_ptr(self.as_native_type_ref().cap) }
    }

    pub fn apparel(&self) -> &CStr {
        unsafe { CStr::from_ptr(self.as_native_type_ref().apparel) }
    }

    pub fn age(&self) -> u32 {
        self.as_native_type_ref().age
    }
}


crate::wrapper_impl!(FaceObject, nvidia_deepstream_sys::NvDsFaceObject);

impl FaceObject {
    pub fn gender(&self) -> &CStr {
        unsafe { CStr::from_ptr(self.as_native_type_ref().gender) }
    }

    pub fn hair(&self) -> &CStr {
        unsafe { CStr::from_ptr(self.as_native_type_ref().hair) }
    }

    pub fn cap(&self) -> &CStr {
        unsafe { CStr::from_ptr(self.as_native_type_ref().cap) }
    }

    pub fn glasses(&self) -> &CStr {
        unsafe { CStr::from_ptr(self.as_native_type_ref().glasses) }
    }

    pub fn facial_hair(&self) -> &CStr {
        unsafe { CStr::from_ptr(self.as_native_type_ref().facialhair) }
    }

    pub fn name(&self) -> &CStr {
        unsafe { CStr::from_ptr(self.as_native_type_ref().name) }
    }

    pub fn eye_color(&self) -> &CStr {
        unsafe { CStr::from_ptr(self.as_native_type_ref().eyecolor) }
    }

    pub fn age(&self) -> u32 {
        self.as_native_type_ref().age
    }
}


crate::wrapper_impl!(VehicleObjectExt, nvidia_deepstream_sys::NvDsVehicleObjectExt);

impl VehicleObjectExt {
    pub fn type_(&self) -> &CStr {
        unsafe { CStr::from_ptr(self.as_native_type_ref().type_) }
    }

    pub fn make(&self) -> &CStr {
        unsafe { CStr::from_ptr(self.as_native_type_ref().make) }
    }

    pub fn model(&self) -> &CStr {
        unsafe { CStr::from_ptr(self.as_native_type_ref().model) }
    }

    pub fn color(&self) -> &CStr {
        unsafe { CStr::from_ptr(self.as_native_type_ref().color) }
    }

    pub fn region(&self) -> &CStr {
        unsafe { CStr::from_ptr(self.as_native_type_ref().region) }
    }

    pub fn license(&self) -> &CStr {
        unsafe { CStr::from_ptr(self.as_native_type_ref().license) }
    }

    //pub fn mask(&self) -> &nvidia_deepstream_sys::GList {
    //}
}


crate::wrapper_impl!(PersonObjectExt, nvidia_deepstream_sys::NvDsPersonObjectExt);

impl PersonObjectExt {
    pub fn gender(&self) -> &CStr {
        unsafe { CStr::from_ptr(self.as_native_type_ref().gender) }
    }

    pub fn hair(&self) -> &CStr {
        unsafe { CStr::from_ptr(self.as_native_type_ref().hair) }
    }

    pub fn cap(&self) -> &CStr {
        unsafe { CStr::from_ptr(self.as_native_type_ref().cap) }
    }

    pub fn apparel(&self) -> &CStr {
        unsafe { CStr::from_ptr(self.as_native_type_ref().apparel) }
    }

    pub fn age(&self) -> u32 {
        self.as_native_type_ref().age
    }

    //pub fn mask(&self) -> &nvidia_deepstream_sys::GList {
    //}
}


crate::wrapper_impl!(FaceObjectExt, nvidia_deepstream_sys::NvDsFaceObjectExt);

impl FaceObjectExt {
    pub fn gender(&self) -> &CStr {
        unsafe { CStr::from_ptr(self.as_native_type_ref().gender) }
    }

    pub fn hair(&self) -> &CStr {
        unsafe { CStr::from_ptr(self.as_native_type_ref().hair) }
    }

    pub fn cap(&self) -> &CStr {
        unsafe { CStr::from_ptr(self.as_native_type_ref().cap) }
    }

    pub fn glasses(&self) -> &CStr {
        unsafe { CStr::from_ptr(self.as_native_type_ref().glasses) }
    }

    pub fn facial_hair(&self) -> &CStr {
        unsafe { CStr::from_ptr(self.as_native_type_ref().facialhair) }
    }

    pub fn name(&self) -> &CStr {
        unsafe { CStr::from_ptr(self.as_native_type_ref().name) }
    }

    pub fn eye_color(&self) -> &CStr {
        unsafe { CStr::from_ptr(self.as_native_type_ref().eyecolor) }
    }

    pub fn age(&self) -> u32 {
        self.as_native_type_ref().age
    }

    //pub fn mask(&self) -> &nvidia_deepstream_sys::GList {
    //}
}
