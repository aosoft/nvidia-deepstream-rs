use gstreamer::prelude::*;
use gstreamer::{PadProbeData, PadProbeReturn, PadProbeType};
use nvidia_deepstream::buffer::BufferNvdsExt;
use nvidia_deepstream::element::ElementNvdsExt;
use nvidia_deepstream::osd::{ColorParams, FontParamsBuilder, TextParamsBuilder};
use std::ffi::CStr;

static CONFIG_YML: &str = "dstest1_config.yml";

static PGIE_CLASS_ID_VEHICLE: i32 = 0;
static PGIE_CLASS_ID_PERSON: i32 = 2;

fn main() {
    let args: Vec<String> = std::env::args().collect();
    if args.len() < 2 {
        println!("Usage:  <H264 filename>");
        return;
    }

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
    let sink = gstreamer::ElementFactory::make("nveglglessink")
        .name("nvvideo-renderer")
        .build()
        .unwrap();

    source.set_property("location", &args[1]);
    streammux.set_property("width", 1920u32);
    streammux.set_property("height", 1080u32);
    streammux.set_property("batch-size", 1u32);
    streammux.set_property("batched-push-timeout", 40000);
    pgie.set_property("config-file-path", "dsmeta_pgie_config.txt");

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

    gstreamer::Element::link_many(&[&source, &h264parser, &decoder]).unwrap();
    gstreamer::Element::link_many(&[
        &streammux, &pgie, &nvvidconv, &nvosd, /*, &transform*/
        &sink,
    ])
        .unwrap();

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
