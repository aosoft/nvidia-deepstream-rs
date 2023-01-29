use crate::{duplicate_glib_string, glib_free, WrapperExt};
use gstreamer::glib::translate::ToGlibPtr;
use std::ffi::{CStr, NulError};
use std::ptr::NonNull;

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

crate::wrapper_impl_value_type!(Rect, nvidia_deepstream_sys::NvDsRect);

impl Rect {
    pub fn new(left: f32, top: f32, width: f32, height: f32) -> Self {
        Rect::from_native_type(nvidia_deepstream_sys::NvDsRect {
            top,
            left,
            width,
            height,
        })
    }

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

crate::wrapper_impl_value_type!(GeoLocation, nvidia_deepstream_sys::NvDsGeoLocation);

impl GeoLocation {
    pub fn new(lat: f64, lon: f64, alt: f64) -> Self {
        GeoLocation::from_native_type(nvidia_deepstream_sys::NvDsGeoLocation { lat, lon, alt })
    }

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

crate::wrapper_impl_value_type!(Coordinate, nvidia_deepstream_sys::NvDsCoordinate);

impl Coordinate {
    pub fn new(x: f64, y: f64, z: f64) -> Self {
        Coordinate::from_native_type(nvidia_deepstream_sys::NvDsCoordinate { x, y, z })
    }

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

crate::wrapper_impl_value_type!(ObjectSignature, nvidia_deepstream_sys::NvDsObjectSignature);

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

crate::wrapper_impl_ref_type!(VehicleObject, nvidia_deepstream_sys::NvDsVehicleObject);

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

impl Clone for VehicleObject {
    fn clone(&self) -> Self {
        unsafe {
            VehicleObject::from_native_type(nvidia_deepstream_sys::NvDsVehicleObject {
                type_: duplicate_glib_string(self.as_native_type_ref().type_),
                make: duplicate_glib_string(self.as_native_type_ref().make),
                model: duplicate_glib_string(self.as_native_type_ref().model),
                color: duplicate_glib_string(self.as_native_type_ref().color),
                region: duplicate_glib_string(self.as_native_type_ref().region),
                license: duplicate_glib_string(self.as_native_type_ref().license),
            })
        }
    }
}

impl Drop for VehicleObject {
    fn drop(&mut self) {
        unsafe {
            glib_free(self.as_native_type_ref().type_);
            glib_free(self.as_native_type_ref().make);
            glib_free(self.as_native_type_ref().model);
            glib_free(self.as_native_type_ref().color);
            glib_free(self.as_native_type_ref().region);
            glib_free(self.as_native_type_ref().license);
        }
    }
}

pub struct VehicleObjectBuilder<'a> {
    type_: Option<&'a str>,
    make: Option<&'a str>,
    model: Option<&'a str>,
    color: Option<&'a str>,
    region: Option<&'a str>,
    license: Option<&'a str>,
}

impl<'a> VehicleObjectBuilder<'a> {
    pub fn type_(mut self, s: &'a str) -> Self {
        self.type_ = Some(s);
        self
    }

    pub fn make(mut self, s: &'a str) -> Self {
        self.make = Some(s);
        self
    }

    pub fn model(mut self, s: &'a str) -> Self {
        self.model = Some(s);
        self
    }

    pub fn color(mut self, s: &'a str) -> Self {
        self.color = Some(s);
        self
    }

    pub fn region(mut self, s: &'a str) -> Self {
        self.region = Some(s);
        self
    }

    pub fn license(mut self, s: &'a str) -> Self {
        self.license = Some(s);
        self
    }

    pub fn build(self) -> Result<VehicleObject, NulError> {
        Ok(VehicleObject::from_native_type(
            nvidia_deepstream_sys::NvDsVehicleObject {
                type_: self.type_.to_glib_full(),
                make: self.make.to_glib_full(),
                model: self.model.to_glib_full(),
                color: self.color.to_glib_full(),
                region: self.region.to_glib_full(),
                license: self.license.to_glib_full(),
            },
        ))
    }
}

crate::wrapper_impl_ref_type!(PersonObject, nvidia_deepstream_sys::NvDsPersonObject);

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

impl Clone for PersonObject {
    fn clone(&self) -> Self {
        unsafe {
            PersonObject::from_native_type(nvidia_deepstream_sys::NvDsPersonObject {
                gender: duplicate_glib_string(self.as_native_type_ref().gender),
                hair: duplicate_glib_string(self.as_native_type_ref().hair),
                cap: duplicate_glib_string(self.as_native_type_ref().cap),
                apparel: duplicate_glib_string(self.as_native_type_ref().apparel),
                age: self.as_native_type_ref().age,
            })
        }
    }
}

impl Drop for PersonObject {
    fn drop(&mut self) {
        unsafe {
            glib_free(self.as_native_type_ref().gender);
            glib_free(self.as_native_type_ref().hair);
            glib_free(self.as_native_type_ref().cap);
            glib_free(self.as_native_type_ref().apparel);
        }
    }
}

pub struct PersonObjectBuilder<'a> {
    gender: Option<&'a str>,
    hair: Option<&'a str>,
    cap: Option<&'a str>,
    apparel: Option<&'a str>,
    age: Option<u32>,
}

impl<'a> PersonObjectBuilder<'a> {
    pub fn gender(mut self, s: &'a str) -> Self {
        self.gender = Some(s);
        self
    }

    pub fn hair(mut self, s: &'a str) -> Self {
        self.hair = Some(s);
        self
    }

    pub fn cap(mut self, s: &'a str) -> Self {
        self.cap = Some(s);
        self
    }

    pub fn apparel(mut self, s: &'a str) -> Self {
        self.apparel = Some(s);
        self
    }

    pub fn age(mut self, s: u32) -> Self {
        self.age = Some(s);
        self
    }

    pub fn build(self) -> PersonObject {
        PersonObject::from_native_type(nvidia_deepstream_sys::NvDsPersonObject {
            gender: self.gender.to_glib_full(),
            hair: self.hair.to_glib_full(),
            cap: self.cap.to_glib_full(),
            apparel: self.apparel.to_glib_full(),
            age: self.age.unwrap_or_default(),
        })
    }
}

crate::wrapper_impl_ref_type!(FaceObject, nvidia_deepstream_sys::NvDsFaceObject);

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

impl Clone for FaceObject {
    fn clone(&self) -> Self {
        unsafe {
            FaceObject::from_native_type(nvidia_deepstream_sys::NvDsFaceObject {
                gender: duplicate_glib_string(self.as_native_type_ref().gender),
                hair: duplicate_glib_string(self.as_native_type_ref().hair),
                cap: duplicate_glib_string(self.as_native_type_ref().cap),
                glasses: duplicate_glib_string(self.as_native_type_ref().gender),
                facialhair: duplicate_glib_string(self.as_native_type_ref().facialhair),
                name: duplicate_glib_string(self.as_native_type_ref().name),
                eyecolor: duplicate_glib_string(self.as_native_type_ref().eyecolor),
                age: self.as_native_type_ref().age,
            })
        }
    }
}

impl Drop for FaceObject {
    fn drop(&mut self) {
        unsafe {
            glib_free(self.as_native_type_ref().gender);
            glib_free(self.as_native_type_ref().hair);
            glib_free(self.as_native_type_ref().cap);
            glib_free(self.as_native_type_ref().gender);
            glib_free(self.as_native_type_ref().facialhair);
            glib_free(self.as_native_type_ref().name);
            glib_free(self.as_native_type_ref().eyecolor);
        }
    }
}

pub struct FaceObjectBuilder<'a> {
    gender: Option<&'a str>,
    hair: Option<&'a str>,
    cap: Option<&'a str>,
    glasses: Option<&'a str>,
    facialhair: Option<&'a str>,
    name: Option<&'a str>,
    eyecolor: Option<&'a str>,
    age: Option<u32>,
}

impl<'a> FaceObjectBuilder<'a> {
    pub fn gender(mut self, s: &'a str) -> Self {
        self.gender = Some(s);
        self
    }

    pub fn hair(mut self, s: &'a str) -> Self {
        self.hair = Some(s);
        self
    }

    pub fn cap(mut self, s: &'a str) -> Self {
        self.cap = Some(s);
        self
    }

    pub fn glasses(mut self, s: &'a str) -> Self {
        self.glasses = Some(s);
        self
    }

    pub fn facialhair(mut self, s: &'a str) -> Self {
        self.facialhair = Some(s);
        self
    }

    pub fn name(mut self, s: &'a str) -> Self {
        self.name = Some(s);
        self
    }

    pub fn eyecolor(mut self, s: &'a str) -> Self {
        self.eyecolor = Some(s);
        self
    }

    pub fn age(mut self, s: u32) -> Self {
        self.age = Some(s);
        self
    }

    pub fn build(self) -> FaceObject {
        FaceObject::from_native_type(nvidia_deepstream_sys::NvDsFaceObject {
            gender: self.gender.to_glib_full(),
            hair: self.hair.to_glib_full(),
            cap: self.cap.to_glib_full(),
            glasses: self.glasses.to_glib_full(),
            facialhair: self.facialhair.to_glib_full(),
            name: self.name.to_glib_full(),
            eyecolor: self.eyecolor.to_glib_full(),
            age: self.age.unwrap_or_default(),
        })
    }
}

crate::wrapper_impl_ref_type!(
    VehicleObjectExt,
    nvidia_deepstream_sys::NvDsVehicleObjectExt
);

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

    pub fn mask(&self) -> *mut nvidia_deepstream_sys::GList {
        self.as_native_type_ref().mask
    }
}

crate::wrapper_impl_ref_type!(PersonObjectExt, nvidia_deepstream_sys::NvDsPersonObjectExt);

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

    pub fn mask(&self) -> *mut nvidia_deepstream_sys::GList {
        self.as_native_type_ref().mask
    }
}

crate::wrapper_impl_ref_type!(FaceObjectExt, nvidia_deepstream_sys::NvDsFaceObjectExt);

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

    pub fn mask(&self) -> *mut nvidia_deepstream_sys::GList {
        self.as_native_type_ref().mask
    }
}

crate::wrapper_impl_ref_type!(EventMsgMeta, nvidia_deepstream_sys::NvDsEventMsgMeta);

impl EventMsgMeta {
    pub fn type_(&self) -> EventType {
        unsafe { std::mem::transmute(self.as_native_type_ref().type_) }
    }

    pub fn obj_type(&self) -> ObjectType {
        unsafe { std::mem::transmute(self.as_native_type_ref().objType) }
    }

    pub fn bbox(&self) -> &Rect {
        Rect::from_native_type_ref(&self.as_native_type_ref().bbox)
    }

    pub fn location(&self) -> &GeoLocation {
        GeoLocation::from_native_type_ref(&self.as_native_type_ref().location)
    }

    pub fn coordinate(&self) -> &Coordinate {
        Coordinate::from_native_type_ref(&self.as_native_type_ref().coordinate)
    }

    pub fn obj_signature(&self) -> &ObjectSignature {
        ObjectSignature::from_native_type_ref(&self.as_native_type_ref().objSignature)
    }

    pub fn obj_class_id(&self) -> i32 {
        self.as_native_type_ref().objClassId
    }

    pub fn sensor_id(&self) -> i32 {
        self.as_native_type_ref().sensorId
    }

    pub fn module_id(&self) -> i32 {
        self.as_native_type_ref().moduleId
    }

    pub fn place_id(&self) -> i32 {
        self.as_native_type_ref().placeId
    }

    pub fn component_id(&self) -> i32 {
        self.as_native_type_ref().componentId
    }

    pub fn frame_id(&self) -> i32 {
        self.as_native_type_ref().frameId
    }

    pub fn confidence(&self) -> f64 {
        self.as_native_type_ref().confidence
    }

    pub fn tracking_id(&self) -> u64 {
        self.as_native_type_ref().trackingId
    }

    pub fn ts(&self) -> &CStr {
        unsafe { CStr::from_ptr(self.as_native_type_ref().ts) }
    }

    pub fn object_id(&self) -> &CStr {
        unsafe { CStr::from_ptr(self.as_native_type_ref().objectId) }
    }

    pub fn sensor_str(&self) -> &CStr {
        unsafe { CStr::from_ptr(self.as_native_type_ref().sensorStr) }
    }

    pub fn other_attrs(&self) -> &CStr {
        unsafe { CStr::from_ptr(self.as_native_type_ref().otherAttrs) }
    }

    pub fn video_path(&self) -> &CStr {
        unsafe { CStr::from_ptr(self.as_native_type_ref().videoPath) }
    }

    pub fn ext_msg(&self) -> &[u8] {
        unsafe {
            std::slice::from_raw_parts(
                self.as_native_type_ref().extMsg as _,
                self.as_native_type_ref().extMsgSize as _,
            )
        }
    }
}

crate::wrapper_impl_ref_type!(Event, nvidia_deepstream_sys::NvDsEvent);

impl Event {
    pub fn event_type(&self) -> EventType {
        unsafe { std::mem::transmute(self.as_native_type_ref().eventType) }
    }

    pub fn metadata(&self) -> Option<&EventMsgMeta> {
        unsafe {
            NonNull::new(self.as_native_type_ref().metadata)
                .map(|p| EventMsgMeta::from_native_type_ref(p.as_ref()))
        }
    }
}

crate::wrapper_impl_ref_type!(CustomMsgInfo, nvidia_deepstream_sys::NvDsCustomMsgInfo);

impl CustomMsgInfo {
    pub fn message(&self) -> &[u8] {
        unsafe {
            std::slice::from_raw_parts(
                self.as_native_type_ref().message as _,
                self.as_native_type_ref().size as _,
            )
        }
    }
}

crate::wrapper_impl_ref_type!(Payload, nvidia_deepstream_sys::NvDsPayload);

impl Payload {
    pub fn payload(&self) -> &[u8] {
        unsafe {
            std::slice::from_raw_parts(
                self.as_native_type_ref().payload as _,
                self.as_native_type_ref().payloadSize as _,
            )
        }
    }

    pub fn component_id(&self) -> u32 {
        self.as_native_type_ref().componentId
    }
}
