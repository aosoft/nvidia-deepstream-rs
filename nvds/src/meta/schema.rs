use crate::{duplicate_glib_string, glib_free, WrapperExt};
use gstreamer::glib::translate::ToGlibPtr;
use std::ffi::CStr;
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
    pub fn new() -> VehicleObjectBuilder<'a> {
        VehicleObjectBuilder {
            type_: None,
            make: None,
            model: None,
            color: None,
            region: None,
            license: None,
        }
    }

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

    pub fn build(self) -> VehicleObject {
        VehicleObject::from_native_type(
            nvidia_deepstream_sys::NvDsVehicleObject {
                type_: self.type_.to_glib_full(),
                make: self.make.to_glib_full(),
                model: self.model.to_glib_full(),
                color: self.color.to_glib_full(),
                region: self.region.to_glib_full(),
                license: self.license.to_glib_full(),
            },
        )
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
    pub fn new() -> PersonObjectBuilder<'a> {
        PersonObjectBuilder {
            gender: None,
            hair: None,
            cap: None,
            apparel: None,
            age: None,
        }
    }

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
    facial_hair: Option<&'a str>,
    name: Option<&'a str>,
    eye_color: Option<&'a str>,
    age: Option<u32>,
}

impl<'a> FaceObjectBuilder<'a> {
    pub fn new() -> FaceObjectBuilder<'a> {
        FaceObjectBuilder {
            gender: None,
            hair: None,
            cap: None,
            glasses: None,
            facial_hair: None,
            name: None,
            eye_color: None,
            age: None,
        }
    }

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

    pub fn facial_hair(mut self, s: &'a str) -> Self {
        self.facial_hair = Some(s);
        self
    }

    pub fn name(mut self, s: &'a str) -> Self {
        self.name = Some(s);
        self
    }

    pub fn eye_color(mut self, s: &'a str) -> Self {
        self.eye_color = Some(s);
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
            facialhair: self.facial_hair.to_glib_full(),
            name: self.name.to_glib_full(),
            eyecolor: self.eye_color.to_glib_full(),
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

crate::wrapper_impl_ref_type!(EventMsgMetaBase, nvidia_deepstream_sys::NvDsEventMsgMeta);

pub struct EventMsgMeta<T: Clone>(EventMsgMetaBase, core::marker::PhantomData<T>);

impl<T: Clone> EventMsgMeta<T> {
    pub fn type_(&self) -> EventType {
        unsafe { std::mem::transmute(self.0.as_native_type_ref().type_) }
    }

    pub fn obj_type(&self) -> ObjectType {
        unsafe { std::mem::transmute(self.0.as_native_type_ref().objType) }
    }

    pub fn bbox(&self) -> &Rect {
        Rect::from_native_type_ref(&self.0.as_native_type_ref().bbox)
    }

    pub fn location(&self) -> &GeoLocation {
        GeoLocation::from_native_type_ref(&self.0.as_native_type_ref().location)
    }

    pub fn coordinate(&self) -> &Coordinate {
        Coordinate::from_native_type_ref(&self.0.as_native_type_ref().coordinate)
    }

    pub fn obj_signature(&self) -> &ObjectSignature {
        ObjectSignature::from_native_type_ref(&self.0.as_native_type_ref().objSignature)
    }

    pub fn obj_class_id(&self) -> i32 {
        self.0.as_native_type_ref().objClassId
    }

    pub fn sensor_id(&self) -> i32 {
        self.0.as_native_type_ref().sensorId
    }

    pub fn module_id(&self) -> i32 {
        self.0.as_native_type_ref().moduleId
    }

    pub fn place_id(&self) -> i32 {
        self.0.as_native_type_ref().placeId
    }

    pub fn component_id(&self) -> i32 {
        self.0.as_native_type_ref().componentId
    }

    pub fn frame_id(&self) -> i32 {
        self.0.as_native_type_ref().frameId
    }

    pub fn confidence(&self) -> f64 {
        self.0.as_native_type_ref().confidence
    }

    pub fn tracking_id(&self) -> u64 {
        self.0.as_native_type_ref().trackingId
    }

    pub fn ts(&self) -> &CStr {
        unsafe { CStr::from_ptr(self.0.as_native_type_ref().ts) }
    }

    pub fn object_id(&self) -> &CStr {
        unsafe { CStr::from_ptr(self.0.as_native_type_ref().objectId) }
    }

    pub fn sensor_str(&self) -> &CStr {
        unsafe { CStr::from_ptr(self.0.as_native_type_ref().sensorStr) }
    }

    pub fn other_attrs(&self) -> &CStr {
        unsafe { CStr::from_ptr(self.0.as_native_type_ref().otherAttrs) }
    }

    pub fn video_path(&self) -> &CStr {
        unsafe { CStr::from_ptr(self.0.as_native_type_ref().videoPath) }
    }

    pub unsafe fn ext_msg(&self) -> Option<&T> {
        if self.0.as_native_type_ref().extMsgSize as usize == std::mem::size_of::<T>() {
            NonNull::new(self.0.as_native_type_ref().extMsg as *mut T).map(|p| p.as_ref())
        } else {
            None
        }
    }

    pub extern "C" fn base_meta_copy_func(
        data: nvidia_deepstream_sys::gpointer,
        _: nvidia_deepstream_sys::gpointer,
    ) -> nvidia_deepstream_sys::gpointer {
        unsafe {
            let event_msg_meta = data as *mut EventMsgMeta<T>;
            if event_msg_meta == std::ptr::null_mut() {
                return std::ptr::null_mut();
            }
            let boxed = Box::new((*event_msg_meta).clone());
            Box::into_raw(boxed) as _
        }
    }

    pub extern "C" fn base_meta_release_func(
        data: nvidia_deepstream_sys::gpointer,
        _: nvidia_deepstream_sys::gpointer,
    ) {
        unsafe {
            let event_msg_meta = data as *mut EventMsgMeta<T>;
            if event_msg_meta == std::ptr::null_mut() {
                return;
            }
            let boxed = Box::from_raw(event_msg_meta);
            drop(boxed);
        }
    }
}

impl<T: Clone> Clone for EventMsgMeta<T> {
    fn clone(&self) -> Self {
        unsafe {
            EventMsgMeta::<T>(
                EventMsgMetaBase::from_native_type(nvidia_deepstream_sys::NvDsEventMsgMeta {
                    type_: self.0.as_native_type_ref().type_,
                    objType: self.0.as_native_type_ref().objType,
                    bbox: self.0.as_native_type_ref().bbox,
                    location: self.0.as_native_type_ref().location,
                    coordinate: self.0.as_native_type_ref().coordinate,
                    objSignature: self.0.as_native_type_ref().objSignature,
                    objClassId: self.0.as_native_type_ref().objClassId,
                    sensorId: self.0.as_native_type_ref().sensorId,
                    moduleId: self.0.as_native_type_ref().moduleId,
                    placeId: self.0.as_native_type_ref().placeId,
                    componentId: self.0.as_native_type_ref().componentId,
                    frameId: self.0.as_native_type_ref().frameId,
                    confidence: self.0.as_native_type_ref().confidence,
                    trackingId: self.0.as_native_type_ref().trackingId,
                    ts: nvidia_deepstream_sys::strdup(self.0.as_native_type_ref().ts),
                    objectId: nvidia_deepstream_sys::strdup(self.0.as_native_type_ref().objectId),
                    sensorStr: nvidia_deepstream_sys::strdup(self.0.as_native_type_ref().sensorStr),
                    otherAttrs: nvidia_deepstream_sys::strdup(
                        self.0.as_native_type_ref().otherAttrs,
                    ),
                    videoPath: nvidia_deepstream_sys::strdup(self.0.as_native_type_ref().videoPath),
                    extMsg: std::ptr::null_mut(), // Box::into_raw(x),
                    extMsgSize: self.0.as_native_type_ref().extMsgSize,
                }),
                core::marker::PhantomData,
            )
        }
    }
}

impl<T: Clone> Drop for EventMsgMeta<T> {
    fn drop(&mut self) {
        unsafe {
            let x = Box::from_raw(self.0.as_native_type_ref().extMsg as *mut T);
            nvidia_deepstream_sys::g_free(self.0.as_native_type_ref().ts as _);
            nvidia_deepstream_sys::g_free(self.0.as_native_type_ref().objectId as _);
            nvidia_deepstream_sys::g_free(self.0.as_native_type_ref().sensorStr as _);
            nvidia_deepstream_sys::g_free(self.0.as_native_type_ref().otherAttrs as _);
            nvidia_deepstream_sys::g_free(self.0.as_native_type_ref().videoPath as _);
            drop(x);
        }
    }
}

pub struct EventMsgMetaBuilder<'a> {
    type_: Option<EventType>,
    obj_type: Option<ObjectType>,
    bbox: Option<Rect>,
    location: Option<GeoLocation>,
    coordinate: Option<Coordinate>,
    obj_signature: Option<ObjectSignature>,
    obj_class_id: Option<i32>,
    sensor_id: Option<i32>,
    module_id: Option<i32>,
    place_id: Option<i32>,
    component_id: Option<i32>,
    frame_id: Option<i32>,
    confidence: Option<f64>,
    tracking_id: Option<u64>,
    ts: Option<&'a str>,
    object_id: Option<&'a str>,
    sensor_str: Option<&'a str>,
    other_attrs: Option<&'a str>,
    video_path: Option<&'a str>,
}

impl<'a> EventMsgMetaBuilder<'a> {
    pub fn new() -> EventMsgMetaBuilder<'a> {
        EventMsgMetaBuilder {
            type_: None,
            obj_type: None,
            bbox: None,
            location: None,
            coordinate: None,
            obj_signature: None,
            obj_class_id: None,
            sensor_id: None,
            module_id: None,
            place_id: None,
            component_id: None,
            frame_id: None,
            confidence: None,
            tracking_id: None,
            ts: None,
            object_id: None,
            sensor_str: None,
            other_attrs: None,
            video_path: None,
        }
    }

    pub fn type_(mut self, value: EventType) -> Self {
        self.type_ = Some(value);
        self
    }

    pub fn obj_type(mut self, value: ObjectType) -> Self {
        self.obj_type = Some(value);
        self
    }

    pub fn bbox(mut self, value: Rect) -> Self {
        self.bbox = Some(value);
        self
    }

    pub fn location(mut self, value: GeoLocation) -> Self {
        self.location = Some(value);
        self
    }

    pub fn coordinate(mut self, value: Coordinate) -> Self {
        self.coordinate = Some(value);
        self
    }

    pub fn obj_signature(mut self, value: ObjectSignature) -> Self {
        self.obj_signature = Some(value);
        self
    }

    pub fn obj_class_id(mut self, value: i32) -> Self {
        self.obj_class_id = Some(value);
        self
    }

    pub fn sensor_id(mut self, value: i32) -> Self {
        self.sensor_id = Some(value);
        self
    }

    pub fn module_id(mut self, value: i32) -> Self {
        self.module_id = Some(value);
        self
    }

    pub fn place_id(mut self, value: i32) -> Self {
        self.place_id = Some(value);
        self
    }

    pub fn component_id(mut self, value: i32) -> Self {
        self.component_id = Some(value);
        self
    }

    pub fn frame_id(mut self, value: i32) -> Self {
        self.frame_id = Some(value);
        self
    }

    pub fn confidence(mut self, value: f64) -> Self {
        self.confidence = Some(value);
        self
    }

    pub fn tracking_id(mut self, value: u64) -> Self {
        self.tracking_id = Some(value);
        self
    }

    pub fn ts(mut self, value: &'a str) -> Self {
        self.ts = Some(value);
        self
    }

    pub fn object_id(mut self, value: &'a str) -> Self {
        self.object_id = Some(value);
        self
    }

    pub fn sensor_str(mut self, value: &'a str) -> Self {
        self.sensor_str = Some(value);
        self
    }

    pub fn other_attrs(mut self, value: &'a str) -> Self {
        self.other_attrs = Some(value);
        self
    }

    pub fn video_path(mut self, value: &'a str) -> Self {
        self.video_path = Some(value);
        self
    }

    pub fn build(self) -> Box<EventMsgMeta<()>> {
        self.internal_build(None)
    }

    pub fn build_with_ext_msg<T: Clone>(self, value: Box<T>) -> Box<EventMsgMeta<T>> {
        self.internal_build(Some(value))
    }

    fn internal_build<T: Clone>(self, value: Option<Box<T>>) -> Box<EventMsgMeta<T>> {
        let (ext_msg, ext_msg_size) = value.map_or_else(
            || (std::ptr::null_mut(), 0),
            |x| (Box::into_raw(x), std::mem::size_of::<T>()),
        );
        Box::new(EventMsgMeta::<T>(
            EventMsgMetaBase::from_native_type(nvidia_deepstream_sys::NvDsEventMsgMeta {
                type_: self.type_.unwrap_or_default() as _,
                objType: self.obj_type.unwrap_or_default() as _,
                bbox: self.bbox.unwrap_or_default().as_native_type(),
                location: self.location.unwrap_or_default().as_native_type(),
                coordinate: self.coordinate.unwrap_or_default().as_native_type(),
                objSignature: self.obj_signature.unwrap_or_default().as_native_type(),
                objClassId: self.obj_class_id.unwrap_or_default() as _,
                sensorId: self.sensor_id.unwrap_or_default() as _,
                moduleId: self.module_id.unwrap_or_default() as _,
                placeId: self.place_id.unwrap_or_default() as _,
                componentId: self.component_id.unwrap_or_default() as _,
                frameId: self.frame_id.unwrap_or_default() as _,
                confidence: self.confidence.unwrap_or_default() as _,
                trackingId: self.tracking_id.unwrap_or_default() as _,
                ts: self.ts.to_glib_full(),
                objectId: self.object_id.to_glib_full(),
                sensorStr: self.sensor_str.to_glib_full(),
                otherAttrs: self.other_attrs.to_glib_full(),
                videoPath: self.video_path.to_glib_full(),
                extMsg: ext_msg as _,
                extMsgSize: ext_msg_size as _,
            }),
            core::marker::PhantomData,
        ))
    }
}

crate::wrapper_impl_ref_type!(Event, nvidia_deepstream_sys::NvDsEvent);

impl Event {
    pub fn event_type(&self) -> EventType {
        unsafe { std::mem::transmute(self.as_native_type_ref().eventType) }
    }

    pub fn metadata<T: Clone>(&self) -> Option<&EventMsgMeta<T>> {
        unsafe {
            NonNull::new(self.as_native_type_ref().metadata)
                .map(|p| std::mem::transmute(p.as_ref()))
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
