use gstreamer::prelude::*;
use gstreamer::{PadProbeData, PadProbeReturn, PadProbeType};
use nvidia_deepstream::meta::osd::{ColorParams, FontParamsBuilder, TextParamsBuilder};
use nvidia_deepstream::meta::schema::EventMsgMetaBuilder;
use nvidia_deepstream::meta::{BatchMetaExt, BufferExt, DisplayMetaBuilder};
use nvidia_deepstream::yaml::ElementNvdsYamlExt;
use std::ffi::{CStr, CString};

static CONFIG_YML: &str = "dstest4_config.yml";
static MSCONV_CONFIG_FILE: &str = "dstest4_msgconv_config.txt";

static PGIE_CLASS_ID_VEHICLE: i32 = 0;
static PGIE_CLASS_ID_PERSON: i32 = 2;

fn main() {
    gstreamer::init().unwrap();

    let pipeline = gstreamer::Pipeline::builder()
        .name("dstest4-pipeline")
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
    let nvstreammux = gstreamer::ElementFactory::make("nvstreammux")
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
    let msgconv = gstreamer::ElementFactory::make("nvmsgconv")
        .name("nvmsg-converter")
        .build()
        .unwrap();
    let msgbroker = gstreamer::ElementFactory::make("nvmsgbroker")
        .name("nvmsg-broker")
        .build()
        .unwrap();
    let tee = gstreamer::ElementFactory::make("tee")
        .name("nvsink-tee")
        .build()
        .unwrap();
    let queue1 = gstreamer::ElementFactory::make("queue")
        .name("nvtee-que1")
        .build()
        .unwrap();
    let queue2 = gstreamer::ElementFactory::make("queue")
        .name("nvtee-que2")
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
    nvstreammux
        .nvds_parse_streammux(CONFIG_YML, "streammux")
        .unwrap();
    pgie.set_property("config-file-path", "dstest4_pgie_config.yml");
    msgconv.set_property("config", "dstest4_msgconv_config.yml");
    msgconv.nvds_parse_msgconv(CONFIG_YML, "msgconv").unwrap();
    msgbroker.nvds_parse_msgbroker(CONFIG_YML, "msgbroker").unwrap();
    sink.nvds_parse_egl_sink(CONFIG_YML, "sink").unwrap();

    pipeline
        .add_many(&[
            &source,
            &h264parser,
            &decoder,
            &nvstreammux,
            &pgie,
            &nvvidconv,
            &nvosd,
            &tee,
            &queue1,
            &queue2,
            &msgconv,
            &msgbroker,
            &sink,
        ])
        .unwrap();

    let sinkpad = nvstreammux.request_pad_simple("sink_0").unwrap();
    let srcpad = decoder.static_pad("src").unwrap();
    srcpad.link(&sinkpad).unwrap();

    gstreamer::Element::link_many(&[&source, &h264parser, &decoder]).unwrap();
    gstreamer::Element::link_many(&[&nvstreammux, &pgie, &nvvidconv, &nvosd, &tee]).unwrap();
    gstreamer::Element::link_many(&[&queue1, &msgconv, &msgbroker]).unwrap();
    gstreamer::Element::link_many(&[&queue2, &sink]).unwrap();

    let sink_pad = queue1.static_pad("sink").unwrap();
    let tee_msg_pad = tee.request_pad_simple("src_%u").unwrap();
    tee_msg_pad.link(&sink_pad).unwrap();

    let sink_pad = queue2.static_pad("sink").unwrap();
    let tee_render_pad = tee.request_pad_simple("src_%u").unwrap();
    tee_render_pad.link(&sink_pad).unwrap();

    let osd_sink_pad = nvosd.static_pad("sink").unwrap();
    osd_sink_pad.add_probe(PadProbeType::BUFFER, |_, info| {
        if let PadProbeData::Buffer(buf) = &info.data.as_ref().unwrap() {
            unsafe {
                let mut vehicle_count: u32 = 0;
                let mut person_count: u32 = 0;
                let mut num_rects: u32 = 0;
                if let Some(batch_meta) = buf.get_nvds_batch_meta() {
                    let mut is_first_object = true;
                    for frame_meta in batch_meta.frame_meta_list().iter() {
                        if let Some(obj_meta_list) = frame_meta.obj_meta_list() {
                            for obj_meta in obj_meta_list.iter() {
                                if obj_meta.class_id() == PGIE_CLASS_ID_VEHICLE {
                                    vehicle_count += 1;
                                    num_rects += 1;
                                }
                                if obj_meta.class_id() == PGIE_CLASS_ID_PERSON {
                                    person_count += 1;
                                    num_rects += 1;
                                }
                            }

                            if let Some(display_meta) = DisplayMetaBuilder::new()
                                .text_params(&[TextParamsBuilder::new()
                                    .display_text(
                                        CString::new(format!(
                                            "Person = {}, Vehicle = {}",
                                            person_count, vehicle_count
                                        ))
                                            .unwrap()
                                            .as_ref(),
                                    )
                                    .x_offset(10)
                                    .y_offset(12)
                                    .font_params(
                                        FontParamsBuilder::new()
                                            .font_name(CStr::from_ptr("Serif\0".as_ptr() as _))
                                            .font_size(10)
                                            .font_color(ColorParams::white())
                                            .build(),
                                    )
                                    .text_bg_clr(ColorParams::black())
                                    .build()])
                                .build(batch_meta)
                            {
                                frame_meta.add_display_meta(display_meta);
                            }
                            if is_first_object {
                                let msg_meta = EventMsgMetaBuilder::new();

                                msg_meta
                                    .build_with_ext_msg(Box::new(nvidia_deepstream::meta::schema::VehicleObjectBuilder::new()
                                        .type_("sedan")
                                        .color("blue")
                                        .make("Bugatti")
                                        .model("M")
                                        .license("XX1234")
                                        .region("CA")
                                        .build()));

                                is_first_object = false;
                            }
                        }
                    }
                }
                println!(
                    "Number of objects = {} Vehicle Count = {} Person Count = {}",
                    num_rects, vehicle_count, person_count
                );
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
