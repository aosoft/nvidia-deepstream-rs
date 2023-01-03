use gstreamer::glib::ObjectExt;
use nvidia_deepstream::element::ElementNvdsExt;

static CONFIG_YML: &str = "dstest2_config.yml";
static PGIE_CONFIG_YML: &str = "dstest1_pgie_config.yml";

static PGIE_CLASS_ID_VEHICLE: i32 = 0;
static PGIE_CLASS_ID_PERSON: i32 = 2;

fn main() {
    gstreamer::init().unwrap();

    let pipeline = gstreamer::Pipeline::builder()
        .name("dstest2-pipeline")
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
    let nvtracker = gstreamer::ElementFactory::make("nvtracker")
        .name("tracker")
        .build()
        .unwrap();

    let sgie1 = gstreamer::ElementFactory::make("nvinfer")
        .name("primary-nvinference-engine")
        .build()
        .unwrap();
    let sgie2 = gstreamer::ElementFactory::make("nvinfer")
        .name("primary-nvinference-engine")
        .build()
        .unwrap();
    let sgie3 = gstreamer::ElementFactory::make("nvinfer")
        .name("primary-nvinference-engine")
        .build()
        .unwrap();

    let nvvidconv = gstreamer::ElementFactory::make("nvvideoconvert")
        .name("nvvideo-converter")
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

    source.nvds_parse_file_source(CONFIG_YML, "source").unwrap();
    streammux
        .nvds_parse_streammux(CONFIG_YML, "streammux")
        .unwrap();
    pgie.set_property("config-file-path", "dstest1_pgie_config.yml");
    sgie1.set_property("config-file-path", "dstest2_sgie1_config.yml");
    sgie2.set_property("config-file-path", "dstest2_sgie2_config.yml");
    sgie3.set_property("config-file-path", "dstest2_sgie3_config.yml");

    tracker.nvds_parse_tracker(CONFIG_YML, "tracker").unwrap();
}