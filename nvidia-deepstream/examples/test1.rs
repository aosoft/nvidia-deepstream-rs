use std::ffi::CString;
use gstreamer::prelude::*;
use gstreamer::{PadProbeData, PadProbeReturn, PadProbeType};
use std::ptr;
use nvidia_deepstream::element::ElementNvdsExt;

static CONFIG_YML: &str = "dstest1_config.yml";
static PGIE_CONFIG_YML: &str = "dstest1_pgie_config.yml";

static MAX_DISPLAY_LEN: i32 = 64;
static PGIE_CLASS_ID_VEHICLE: i32 = 0;
static PGIE_CLASS_ID_PERSON: i32 = 2;

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
    /*let transform = gstreamer::ElementFactory::make("nvegltransform")
        .name("nvvideo-transform")
        .build()
        .unwrap();*/
    let sink = gstreamer::ElementFactory::make("nveglglessink")
        .name("nvvideo-renderer")
        .build()
        .unwrap();

    source.nvds_parse_file_source(CONFIG_YML, "source").unwrap();
    streammux.nvds_parse_streammux(CONFIG_YML, "streammux").unwrap();
    pgie.set_property("config-file-path", PGIE_CONFIG_YML);

    pipeline
        .add_many(&[
            &source,
            &h264parser,
            &decoder,
            &streammux,
            &pgie,
            &nvvidconv,
            &nvosd,
            //&transform,
            &sink,
        ])
        .unwrap();

    let sinkpad = streammux.request_pad_simple("sink_0").unwrap();
    let srcpad = decoder.static_pad("src").unwrap();
    srcpad.link(&sinkpad).unwrap();

    gstreamer::Element::link_many(&[&source, &h264parser, &decoder])
        .unwrap();
    gstreamer::Element::link_many(&[&streammux, &pgie, &nvvidconv, &nvosd/*, &transform*/, &sink])
        .unwrap();

    let osd_sink_pad = nvosd.static_pad("sink").unwrap();
    osd_sink_pad.add_probe(PadProbeType::BUFFER, |pad, info| {
        if let PadProbeData::Buffer(buf) = &info.data.as_ref().unwrap() {
            unsafe {
                let mut vehicle_count: u32 = 0;
                let mut person_count: u32 = 0;
                let mut num_rects: u32 = 0;
                let batch_meta = &mut *nvidia_deepstream_sys::gst_buffer_get_nvds_batch_meta(
                    buf.as_mut_ptr() as *mut nvidia_deepstream_sys::GstBuffer,
                );

                let mut l_frame_p = batch_meta.frame_meta_list;
                while l_frame_p != ptr::null_mut() {
                    let mut l_frame = &*l_frame_p;
                    let frame_meta =
                        &mut *(l_frame.data as *mut nvidia_deepstream_sys::NvDsFrameMeta);
                    let mut l_obj_p = frame_meta.obj_meta_list;
                    while l_obj_p != ptr::null_mut() {
                        let mut l_obj = &*l_obj_p;
                        let obj_meta =
                            &mut *(l_obj.data as *mut nvidia_deepstream_sys::NvDsObjectMeta);
                        if obj_meta.class_id == PGIE_CLASS_ID_VEHICLE {
                            vehicle_count += 1;
                            num_rects += 1;
                        }
                        if obj_meta.class_id == PGIE_CLASS_ID_PERSON {
                            person_count += 1;
                            num_rects += 1;
                        }

                        let display_meta = &mut *(nvidia_deepstream_sys::nvds_acquire_display_meta_from_pool(batch_meta as _));
                        display_meta.num_labels = 1;

                        /* Now set the offsets where the string should appear */
                        display_meta.text_params[0].x_offset = 10;
                        display_meta.text_params[0].y_offset = 12;

                        /* Font , font-color and font-size */
                        display_meta.text_params[0].font_params.font_name = CString::new("Serif").unwrap().as_ptr() as _;
                        display_meta.text_params[0].font_params.font_size = 10;
                        display_meta.text_params[0].font_params.font_color.red = 1.0;
                        display_meta.text_params[0].font_params.font_color.green = 1.0;
                        display_meta.text_params[0].font_params.font_color.blue = 1.0;
                        display_meta.text_params[0].font_params.font_color.alpha = 1.0;

                        /* Text background color */
                        display_meta.text_params[0].set_bg_clr = 1;
                        display_meta.text_params[0].text_bg_clr.red = 0.0;
                        display_meta.text_params[0].text_bg_clr.green = 0.0;
                        display_meta.text_params[0].text_bg_clr.blue = 0.0;
                        display_meta.text_params[0].text_bg_clr.alpha = 1.0;

                        nvidia_deepstream_sys::nvds_add_display_meta_to_frame(frame_meta as _, display_meta as _);

                        l_obj_p = l_obj.next;
                    }
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
}
