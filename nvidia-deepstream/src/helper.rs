use gstreamer::glib::translate::{FromGlibPtrFull, ToGlibPtr};
use gstreamer::glib::IsA;
use std::ffi::CStr;
use std::ptr::NonNull;

pub trait ElementHelperExt {
    fn configure_source_for_ntp_sync(&self);
    fn gst_nvevent_parse_pad_added(&self) -> u32;
    fn gst_nvevent_parse_pad_deleted(&self) -> u32;
    fn gst_nvevent_parse_stream_eos(&self) -> u32;
    fn gst_nvevent_parse_stream_reset(&self) -> u32;
    fn gst_nvevent_new_stream_start(&self, stream_id: &CStr) -> Option<gstreamer::Event>;
}

impl<O: IsA<gstreamer::Element>> ElementHelperExt for O {
    fn configure_source_for_ntp_sync(&self) {
        unsafe { nvidia_deepstream_sys::configure_source_for_ntp_sync(self.as_ptr() as _) }
    }

    fn gst_nvevent_parse_pad_added(&self) -> u32 {
        unsafe {
            let mut r: u32 = 0;
            nvidia_deepstream_sys::gst_nvevent_parse_pad_added(self.as_ptr() as _, &mut r);
            r
        }
    }

    fn gst_nvevent_parse_pad_deleted(&self) -> u32 {
        unsafe {
            let mut r: u32 = 0;
            nvidia_deepstream_sys::gst_nvevent_parse_pad_deleted(self.as_ptr() as _, &mut r);
            r
        }
    }

    fn gst_nvevent_parse_stream_eos(&self) -> u32 {
        unsafe {
            let mut r: u32 = 0;
            nvidia_deepstream_sys::gst_nvevent_parse_stream_eos(self.as_ptr() as _, &mut r);
            r
        }
    }

    fn gst_nvevent_parse_stream_reset(&self) -> u32 {
        unsafe {
            let mut r: u32 = 0;
            nvidia_deepstream_sys::gst_nvevent_parse_stream_reset(self.as_ptr() as _, &mut r);
            r
        }
    }

    fn gst_nvevent_new_stream_start(&self, stream_id: &CStr) -> Option<gstreamer::Event> {
        unsafe {
            NonNull::new(nvidia_deepstream_sys::gst_nvevent_new_stream_start(
                self.as_ptr() as _,
                stream_id.as_ptr() as _,
            ))
            .map(|p| gstreamer::Event::from_glib_full(p.as_ptr() as _))
        }
    }
}

pub trait EventHelperExt {
    fn gst_nvevent_parse_stream_segment(&self) -> (u32, Option<gstreamer::Segment>);
    fn gst_nvevent_parse_stream_start(&self) -> (u32, Option<&CStr>);
}

impl EventHelperExt for gstreamer::Event {
    fn gst_nvevent_parse_stream_segment(&self) -> (u32, Option<gstreamer::Segment>) {
        unsafe {
            let mut source_id: u32 = 0;
            let mut segment: *mut nvidia_deepstream_sys::GstSegment = std::ptr::null_mut();
            nvidia_deepstream_sys::gst_nvevent_parse_stream_segment(
                self.as_ptr() as _,
                &mut source_id,
                &mut segment,
            );
            (
                source_id,
                NonNull::new(segment).map(|p| gstreamer::Segment::from_glib_full(p.as_ptr() as _)),
            )
        }
    }

    fn gst_nvevent_parse_stream_start(&self) -> (u32, Option<&CStr>) {
        unsafe {
            let mut source_id: u32 = 0;
            let mut stream_id: *mut nvidia_deepstream_sys::gchar = std::ptr::null_mut();
            nvidia_deepstream_sys::gst_nvevent_parse_stream_start(
                self.as_ptr() as _,
                &mut source_id,
                &mut stream_id,
            );
            (
                source_id,
                NonNull::new(stream_id).map(|p| CStr::from_ptr(p.as_ptr() as _)),
            )
        }
    }
}

pub fn gst_nvevent_new_pad_added(source_id: u32) -> Option<gstreamer::Event> {
    unsafe {
        NonNull::new(nvidia_deepstream_sys::gst_nvevent_new_pad_added(source_id))
            .map(|p| gstreamer::Event::from_glib_full(p.as_ptr() as _))
    }
}

pub fn gst_nvevent_new_pad_deleted(source_id: u32) -> Option<gstreamer::Event> {
    unsafe {
        NonNull::new(nvidia_deepstream_sys::gst_nvevent_new_pad_deleted(
            source_id,
        ))
        .map(|p| gstreamer::Event::from_glib_full(p.as_ptr() as _))
    }
}

pub fn gst_nvevent_new_stream_eos(source_id: u32) -> Option<gstreamer::Event> {
    unsafe {
        NonNull::new(nvidia_deepstream_sys::gst_nvevent_new_stream_eos(source_id))
            .map(|p| gstreamer::Event::from_glib_full(p.as_ptr() as _))
    }
}

pub fn gst_nvevent_new_stream_segment(
    source_id: u32,
    segment: &gstreamer::Segment,
) -> Option<gstreamer::Event> {
    unsafe {
        NonNull::new(nvidia_deepstream_sys::gst_nvevent_new_stream_segment(
            source_id,
            segment.to_glib_none().0 as _,
        ))
        .map(|p| gstreamer::Event::from_glib_full(p.as_ptr() as _))
    }
}

pub fn gst_nvevent_new_stream_reset(source_id: u32) -> Option<gstreamer::Event> {
    unsafe {
        NonNull::new(nvidia_deepstream_sys::gst_nvevent_new_stream_reset(
            source_id,
        ))
        .map(|p| gstreamer::Event::from_glib_full(p.as_ptr() as _))
    }
}

pub trait ObjectHelperExt {
    fn gst_nvmessage_new_stream_eos(&self, eos_stream_id: u32) -> Option<gstreamer::Message>;
}

impl<O: IsA<gstreamer::glib::Object>> ObjectHelperExt for O {
    fn gst_nvmessage_new_stream_eos(&self, eos_stream_id: u32) -> Option<gstreamer::Message> {
        unsafe {
            NonNull::new(nvidia_deepstream_sys::gst_nvmessage_new_stream_eos(
                self.to_glib_none().0 as _,
                eos_stream_id,
            ))
            .map(|p| gstreamer::Message::from_glib_full(p.as_ptr() as _))
        }
    }
}

pub trait MessageHelperExt {
    fn gst_nvmessage_is_stream_eos(&self) -> bool;
    fn gst_nvmessage_parse_stream_eos(&self) -> Option<u32>;
}

impl MessageHelperExt for gstreamer::Message {
    fn gst_nvmessage_is_stream_eos(&self) -> bool {
        unsafe {
            nvidia_deepstream_sys::gst_nvmessage_is_stream_eos(self.as_ptr() as _) != 0
        }
    }

    fn gst_nvmessage_parse_stream_eos(&self) -> Option<u32> {
        unsafe {
            let mut eos_stream_id: u32 = 0;
            if nvidia_deepstream_sys::gst_nvmessage_parse_stream_eos(
                self.as_ptr() as _,
                &mut eos_stream_id,
            ) != 0
            {
                Some(eos_stream_id)
            } else {
                None
            }
        }
    }
}
