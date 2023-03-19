use gstreamer::prelude::*;
use gstreamer::{PadProbeData, PadProbeReturn, PadProbeType};
use nvidia_deepstream::meta::{BatchMetaExt, BufferExt};
use nvidia_deepstream::{meta};
use std::ffi::CStr;

#[derive(Clone)]
struct UserMetaData {
    data: usize,
}

impl Drop for UserMetaData {
    fn drop(&mut self) {}
}

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

    let infer_src_pad = pgie.static_pad("src").unwrap();
    infer_src_pad.add_probe(PadProbeType::BUFFER, |_, info| {
        if let PadProbeData::Buffer(buf) = &info.data.as_ref().unwrap() {
            unsafe {
                if let Some(batch_meta) = buf.get_nvds_batch_meta() {
                    for frame_meta in batch_meta.frame_meta_list().iter() {
                        let user_meta = meta::UserMeta::new(
                            batch_meta,
                            meta::UserMeta::get_user_meta_type(CStr::from_ptr(
                                "NVIDIA.NVINFER.USER_META\0".as_ptr() as _,
                            )),
                            Box::new(UserMetaData {
                                data: 100,
                            }),
                        );
                        if let Some(user_meta) = user_meta {
                            frame_meta.add_user_meta(user_meta);
                        }
                    }
                }
            }
        }
        PadProbeReturn::Ok
    });

    let osd_sink_pad = nvosd.static_pad("sink").unwrap();
    osd_sink_pad.add_probe(PadProbeType::BUFFER, |_, info| {
        if let PadProbeData::Buffer(buf) = &info.data.as_ref().unwrap() {
            unsafe {
                if let Some(batch_meta) = buf.get_nvds_batch_meta() {
                    for frame_meta in batch_meta.frame_meta_list().iter() {
                        if let Some(user_meta_list) = frame_meta.frame_user_meta_list() {
                            for user_meta in user_meta_list.iter() {
                                println!(
                                    "user_meta_data = {}",
                                    user_meta.user_meta_data::<UserMetaData>().unwrap().data
                                )
                            }
                        }
                    }
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
