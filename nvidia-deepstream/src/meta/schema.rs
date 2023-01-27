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
