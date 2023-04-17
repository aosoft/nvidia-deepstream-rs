//  port from deepstream-test1

use gstreamer::prelude::*;
use gstreamer::{PadProbeData, PadProbeReturn, PadProbeType};
use nvidia_deepstream::meta::osd::{ColorParams, FontParamsBuilder, TextParamsBuilder};
use nvidia_deepstream::meta::{BatchMetaExt, BufferExt, DisplayMetaBuilder};
use nvidia_deepstream::yaml::ElementNvdsYamlExt;
use gstreamer::glib::{GStr, GString};

static CONFIG_YML: &str = "dstest1_config.yml";

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

    source.nvds_parse_file_source(CONFIG_YML, "source");
    streammux.nvds_parse_streammux(CONFIG_YML, "streammux");
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

    let osd_sink_pad = nvosd.static_pad("sink").unwrap();
    osd_sink_pad.add_probe(PadProbeType::BUFFER, |_, info| {
        if let PadProbeData::Buffer(buf) = &info.data.as_ref().unwrap() {
            unsafe {
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

                            if let Some(display_meta) = DisplayMetaBuilder::new()
                                .text_params(&mut [TextParamsBuilder::new()
                                    .display_text(
                                        GString::from(format!(
                                            "Person = {}, Vehicle = {}",
                                            person_count, vehicle_count
                                        )),
                                    )
                                    .x_offset(10)
                                    .y_offset(12)
                                    .font_params(
                                        FontParamsBuilder::new()
                                            .font_name(GStr::from_ptr("Serif\0".as_ptr() as _))
                                            .font_size(10)
                                            .font_color(ColorParams::white())
                                            .build(),
                                    )
                                    .text_bg_clr(ColorParams::black())])
                                .build(batch_meta)
                            {
                                frame_meta.add_display_meta(display_meta);
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
