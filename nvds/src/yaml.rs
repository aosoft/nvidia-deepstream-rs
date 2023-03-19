use gstreamer::glib::{GString, IsA};
use gstreamer::{glib, Element};

#[repr(u32)]
#[derive(Clone, Copy, PartialEq, Debug)]
pub enum YamlParserStatus {
    Success = nvidia_deepstream_sys::NvDsYamlParserStatus_NVDS_YAML_PARSER_SUCCESS as _,
    Disabled = nvidia_deepstream_sys::NvDsYamlParserStatus_NVDS_YAML_PARSER_DISABLED as _,
    Error = nvidia_deepstream_sys::NvDsYamlParserStatus_NVDS_YAML_PARSER_ERROR as _,
}

macro_rules! define_element_nvds_yaml_ext_method {
    ($method:ident) => {
        fn $method(&self, cfg_file_path: &str, group: &str) -> YamlParserStatus;
    };
}

macro_rules! impl_element_nvds_yaml_ext_method {
    ($method:ident) => {
        fn $method(&self, cfg_file_path: &str, group: &str) -> YamlParserStatus {
            unsafe {
                let cfg_file_path = GString::from(cfg_file_path);
                let group = GString::from(group);
                std::mem::transmute(nvidia_deepstream_sys::$method(
                    self.as_ptr() as _,
                    cfg_file_path.as_ptr() as _,
                    group.as_ptr() as _,
                ))
            }
        }
    };
}

pub trait ElementNvdsYamlExt {
    define_element_nvds_yaml_ext_method!(nvds_parse_file_source);
    define_element_nvds_yaml_ext_method!(nvds_parse_uridecodebin);
    define_element_nvds_yaml_ext_method!(nvds_parse_nvarguscamerasrc);
    define_element_nvds_yaml_ext_method!(nvds_parse_v4l2src);
    define_element_nvds_yaml_ext_method!(nvds_parse_alsasrc);
    define_element_nvds_yaml_ext_method!(nvds_parse_streammux);
    define_element_nvds_yaml_ext_method!(nvds_parse_tracker);
    define_element_nvds_yaml_ext_method!(nvds_parse_osd);
    define_element_nvds_yaml_ext_method!(nvds_parse_tiler);
    define_element_nvds_yaml_ext_method!(nvds_parse_msgbroker);
    define_element_nvds_yaml_ext_method!(nvds_parse_msgconv);
    define_element_nvds_yaml_ext_method!(nvds_parse_gie);
    define_element_nvds_yaml_ext_method!(nvds_parse_egl_sink);
    define_element_nvds_yaml_ext_method!(nvds_parse_file_sink);
    define_element_nvds_yaml_ext_method!(nvds_parse_fake_sink);
}

impl<O: IsA<Element>> ElementNvdsYamlExt for O {
    impl_element_nvds_yaml_ext_method!(nvds_parse_file_source);
    impl_element_nvds_yaml_ext_method!(nvds_parse_uridecodebin);
    impl_element_nvds_yaml_ext_method!(nvds_parse_nvarguscamerasrc);
    impl_element_nvds_yaml_ext_method!(nvds_parse_v4l2src);
    impl_element_nvds_yaml_ext_method!(nvds_parse_alsasrc);
    impl_element_nvds_yaml_ext_method!(nvds_parse_streammux);
    impl_element_nvds_yaml_ext_method!(nvds_parse_tracker);
    impl_element_nvds_yaml_ext_method!(nvds_parse_osd);
    impl_element_nvds_yaml_ext_method!(nvds_parse_tiler);
    impl_element_nvds_yaml_ext_method!(nvds_parse_msgbroker);
    impl_element_nvds_yaml_ext_method!(nvds_parse_msgconv);
    impl_element_nvds_yaml_ext_method!(nvds_parse_gie);
    impl_element_nvds_yaml_ext_method!(nvds_parse_egl_sink);
    impl_element_nvds_yaml_ext_method!(nvds_parse_file_sink);
    impl_element_nvds_yaml_ext_method!(nvds_parse_fake_sink);
}

pub fn nvds_parse_source_list(
    cfg_file_path: &str,
    group: &str,
) -> Result<glib::collections::List<GString>, YamlParserStatus> {
    unsafe {
        let mut src_list: *mut nvidia_deepstream_sys::GList = std::ptr::null_mut();
        let cfg_file_path = GString::from(cfg_file_path);
        let group = GString::from(group);
        let r = nvidia_deepstream_sys::nvds_parse_source_list(
            &mut src_list,
            cfg_file_path.as_ptr() as _,
            group.as_ptr() as _,
        );
        match r {
            nvidia_deepstream_sys::NvDsYamlParserStatus_NVDS_YAML_PARSER_SUCCESS => Ok(
                glib::collections::List::<GString>::from_glib_full(src_list as _),
            ),
            _ => Err(std::mem::transmute(r)),
        }
    }
}
