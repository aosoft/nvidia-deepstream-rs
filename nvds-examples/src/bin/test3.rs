//  port from deepstream-test3

use gstreamer::prelude::*;
use gstreamer::{ChildProxy, Element, PadProbeData, PadProbeReturn, PadProbeType};
use nvidia_deepstream::meta::{BatchMetaExt, BufferExt};
use nvidia_deepstream::yaml;
use nvidia_deepstream::yaml::ElementNvdsYamlExt;

static CONFIG_YML: &str = "dstest3_config.yml";

static PGIE_CLASS_ID_VEHICLE: i32 = 0;
static PGIE_CLASS_ID_PERSON: i32 = 2;

fn main() {
    gstreamer::init().unwrap();

    let pipeline = gstreamer::Pipeline::builder()
        .name("dstest3-pipeline")
        .build();
    let streammux = gstreamer::ElementFactory::make("nvstreammux")
        .name("stream-muxer")
        .build()
        .unwrap();
    pipeline.add(&streammux).unwrap();
    let src_list = yaml::nvds_parse_source_list(CONFIG_YML, "source-list")
        .unwrap();

    let mut index = 0;
    for src in &src_list {
        let source_bin = create_source_bin(index, src.as_str());
        pipeline.add(&source_bin).unwrap();
        let sinkpad = streammux
            .request_pad_simple(format!("sink_{}", index).as_str())
            .unwrap();
        let srcpad = source_bin.static_pad("src").unwrap();
        srcpad.link(&sinkpad).unwrap();

        index += 1;
    }

    let pgie = gstreamer::ElementFactory::make("nvinfer")
        .name("primary-nvinference-engine")
        .build()
        .unwrap();

    let queue1 = gstreamer::ElementFactory::make("queue")
        .name("queue1")
        .build()
        .unwrap();
    let queue2 = gstreamer::ElementFactory::make("queue")
        .name("queue2")
        .build()
        .unwrap();
    let queue3 = gstreamer::ElementFactory::make("queue")
        .name("queue3")
        .build()
        .unwrap();
    let queue4 = gstreamer::ElementFactory::make("queue")
        .name("queue4")
        .build()
        .unwrap();
    let queue5 = gstreamer::ElementFactory::make("queue")
        .name("queue5")
        .build()
        .unwrap();
    let nvdslogger = gstreamer::ElementFactory::make("nvdslogger")
        .name("nvdslogger")
        .build()
        .unwrap();
    let tiler = gstreamer::ElementFactory::make("nvmultistreamtiler")
        .name("nvtiler")
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
    let sink = gstreamer::ElementFactory::make("nveglglessink")
        .name("nvvideo-renderer")
        .build()
        .unwrap();
    streammux.nvds_parse_streammux(CONFIG_YML, "streammux");
    pgie.set_property("config-file-path", "dstest3_pgie_config.yml");

    let pgie_batch_size = pgie.property::<u32>("batch-size");
    if src_list.len() != pgie_batch_size as _ {
        pgie.set_property::<u32>("batch-size", src_list.len() as _);
    }
    nvosd.nvds_parse_osd(CONFIG_YML, "osd");

    let tiler_rows = f64::sqrt(src_list.len() as _) as u32;
    let tiler_columns = f64::ceil(1.0 * src_list.len() as f64 / tiler_rows as f64) as u32;
    tiler.set_property("rows", tiler_rows);
    tiler.set_property("columns", tiler_columns);

    tiler.nvds_parse_tiler(CONFIG_YML, "tiler");
    sink.nvds_parse_egl_sink(CONFIG_YML, "sink");

    pipeline
        .add_many(&[
            &queue1,
            &pgie,
            &queue2,
            &nvdslogger,
            &tiler,
            &queue3,
            &nvvidconv,
            &queue4,
            &nvosd,
            &queue5,
            &sink,
        ])
        .unwrap();
    gstreamer::Element::link_many(&[
        &streammux,
        &queue1,
        &pgie,
        &queue2,
        &nvdslogger,
        &tiler,
        &queue3,
        &nvvidconv,
        &queue4,
        &nvosd,
        &queue5,
        &sink,
    ])
    .unwrap();

    let tiler_src_pad = pgie.static_pad("src").unwrap();
    tiler_src_pad.add_probe(PadProbeType::BUFFER, |_, info| {
        if let PadProbeData::Buffer(buf) = &info.data.as_ref().unwrap() {
            let mut vehicle_count: u32 = 0;
            let mut person_count: u32 = 0;
            let mut num_rects: u32 = 0;
            if let Some(batch_meta) = buf.get_nvds_batch_meta() {
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
                    }
                }
            }
            println!(
                "Number of objects = {} Vehicle Count = {} Person Count = {}",
                num_rects, vehicle_count, person_count
            );
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

fn create_source_bin(index: u32, uri: &str) -> Element {
    let bin = gstreamer::Bin::new(Some(format!("source-bin-{}", index).as_str()));
    let uri_decode_bin = gstreamer::ElementFactory::make("uridecodebin")
        .name("uri-decode-bin")
        .build()
        .unwrap();
    uri_decode_bin.set_property("uri", uri);

    let source_bin = bin.clone();
    uri_decode_bin.connect_pad_added(move |_, pad| {
        let caps = pad.caps().unwrap_or_else(|| pad.query_caps(None));
        let str = caps.structure(0).unwrap();
        let name = str.name();
        let features = caps.features(0).unwrap();
        if name.starts_with("video") && features.contains("memory:NVMM") {
            let bin_ghost_pad = source_bin
                .static_pad("src")
                .unwrap()
                .downcast::<gstreamer::GhostPad>()
                .unwrap();
            bin_ghost_pad.set_target(Some(pad)).unwrap();
        }
    });
    uri_decode_bin
        .clone()
        .dynamic_cast::<gstreamer::ChildProxy>()
        .unwrap()
        .connect_child_added(decodebin_child_added);

    bin.add(&uri_decode_bin).unwrap();
    bin.add_pad(&gstreamer::GhostPad::new(
        Some("src"),
        gstreamer::PadDirection::Src,
    ))
    .unwrap();

    bin.into()
}

fn decodebin_child_added(_: &ChildProxy, object: &gstreamer::glib::Object, name: &str) {
    if name.starts_with("decodebin") {
        object
            .clone()
            .dynamic_cast::<gstreamer::ChildProxy>()
            .unwrap()
            .connect_child_added(decodebin_child_added);
    }
    if name.starts_with("source") && object.has_property("drop-on-latency", None) {
        object.set_property::<bool>("drop-on-latency", true);
    }
}
