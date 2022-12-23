use std::ptr;
use gstreamer::prelude::*;
use gstreamer::{PadProbeData, PadProbeReturn, PadProbeType};

static CONFIG_YML: &str = "/opt/nvidia/deepstream/deepstream-6.1/sources/apps/sample_apps/deepstream-test1/dstest1_config.yml";

fn main() {
    gstreamer::init().unwrap();

    let pipeline = gstreamer::Pipeline::builder()
        .name("dstest1-pipeline")
        .build();
    let source = gstreamer::ElementFactory::make("filesrc")
        .name("file-source")
        .build()
        .unwrap();
    let h264parser = gstreamer::ElementFactory::make("h264parse")
        .name("h264-parser")
        .build()
        .unwrap();
    let decoder = gstreamer::ElementFactory::make("nvv4l2decoder")
        .name("nvv4l2-decoder")
        .build()
        .unwrap();
    let streammux = gstreamer::ElementFactory::make("nvstreammux")
        .name("stream-muxer")
        .build()
        .unwrap();

    let pgie = gstreamer::ElementFactory::make("nvinfer")
        .name("primary-nvinference-engine")
        .build()
        .unwrap();
    let nvvidconv = gstreamer::ElementFactory::make("nvvideoconvert")
        .name("pnvvideo-converter")
        .build()
        .unwrap();
    let nvosd = gstreamer::ElementFactory::make("nvdsosd")
        .name("nv-onscreendisplay")
        .build()
        .unwrap();
    let transform = gstreamer::ElementFactory::make("nvegltransform")
        .name("nvvideo-transform")
        .build()
        .unwrap();
    let sink = gstreamer::ElementFactory::make("nveglglessink")
        .name("nvvideo-renderer")
        .build()
        .unwrap();

    unsafe {
        nvidia_deepstream_sys::nvds_parse_file_source(
            source.as_ptr() as *mut nvidia_deepstream_sys::GstElement,
            CONFIG_YML.as_ptr() as *mut nvidia_deepstream_sys::gchar,
            "source".as_ptr() as *const ::std::os::raw::c_char,
        );

        nvidia_deepstream_sys::nvds_parse_streammux(
            streammux.as_ptr() as *mut nvidia_deepstream_sys::GstElement,
            CONFIG_YML.as_ptr() as *mut nvidia_deepstream_sys::gchar,
            "streammux".as_ptr() as *const ::std::os::raw::c_char,
        );
    }
    pgie.set_property("config-file-path", "dstest1_pgie_config.yml");

    pipeline
        .add_many(&[
            &source,
            &h264parser,
            &decoder,
            &streammux,
            &pgie,
            &nvvidconv,
            &nvosd,
            &transform,
            &sink,
        ])
        .unwrap();

    gstreamer::Element::link_many(&[&streammux, &pgie, &nvvidconv, &nvosd, &transform, &sink])
        .unwrap();

    let osd_sink_pad = nvosd.static_pad("sink").unwrap();
    osd_sink_pad.add_probe(PadProbeType::BUFFER, |pad, info| {
        if let PadProbeData::Buffer(buf) = &info.data.as_ref().unwrap() {
            unsafe {
                let batch_meta = &*nvidia_deepstream_sys::gst_buffer_get_nvds_batch_meta(
                    buf.as_mut_ptr() as *mut nvidia_deepstream_sys::GstBuffer
                );

                let mut l_frame_p = batch_meta.frame_meta_list;
                while l_frame_p != ptr::null_mut() {
                    let mut l_frame = &*l_frame_p;
                    let frame_meta = &*(l_frame.data as *const nvidia_deepstream_sys::NvDsFrameMeta);
                    l_frame_p = l_frame.next;
                }
            }
        }
        PadProbeReturn::Ok
    });

    pipeline.set_state(gstreamer::State::Playing).unwrap();
    let bus = pipeline.bus().unwrap();
    for msg in bus.iter_timed(gstreamer::ClockTime::NONE) {
        use gstreamer::MessageView;

        match msg.view() {
            MessageView::Error(err) => {
                eprintln!(
                    "Error received from element {:?}: {}",
                    err.src().map(|s| s.path_string()),
                    err.error()
                );
                eprintln!("Debugging information: {:?}", err.debug());
                break;
            }
            MessageView::Eos(..) => break,
            _ => (),
        }
    }

    pipeline.set_state(gstreamer::State::Null).unwrap();

    bus.remove_watch().unwrap();
}
