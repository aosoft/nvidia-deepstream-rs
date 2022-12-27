use gstreamer::glib::IsA;
use gstreamer::Element;
use std::ffi::{CString, NulError};

#[repr(u32)]
pub enum YamlParserStatus {
    Success = nvidia_deepstream_sys::NvDsYamlParserStatus_NVDS_YAML_PARSER_SUCCESS as _,
    Disabled = nvidia_deepstream_sys::NvDsYamlParserStatus_NVDS_YAML_PARSER_DISABLED as _,
    Error = nvidia_deepstream_sys::NvDsYamlParserStatus_NVDS_YAML_PARSER_ERROR as _,
}

pub trait ElementNvdsExt: 'static {
    fn nvds_parse_file_source(
        &self,
        cfg_file_path: &str,
        group: &str,
    ) -> Result<YamlParserStatus, NulError>;

    fn nvds_parse_streammux(
        &self,
        cfg_file_path: &str,
        group: &str,
    ) -> Result<YamlParserStatus, NulError>;
}

impl<O: IsA<Element>> ElementNvdsExt for O {
    fn nvds_parse_file_source(
        &self,
        cfg_file_path: &str,
        group: &str,
    ) -> Result<YamlParserStatus, NulError> {
        unsafe {
            let cfg_file_path = CString::new(cfg_file_path)?;
            let group = CString::new(group)?;
            Ok(std::mem::transmute(
                nvidia_deepstream_sys::nvds_parse_file_source(
                    self.as_ptr() as _,
                    cfg_file_path.as_ptr() as _,
                    group.as_ptr() as _,
                ),
            ))
        }
    }

    fn nvds_parse_streammux(
        &self,
        cfg_file_path: &str,
        group: &str,
    ) -> Result<YamlParserStatus, NulError> {
        unsafe {
            let cfg_file_path = CString::new(cfg_file_path)?;
            let group = CString::new(group)?;
            Ok(std::mem::transmute(
                nvidia_deepstream_sys::nvds_parse_streammux(
                    self.as_ptr() as _,
                    cfg_file_path.as_ptr() as _,
                    group.as_ptr() as _,
                ),
            ))
        }
    }
}
