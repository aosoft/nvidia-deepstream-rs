use crate::WrapperExt;
use std::ffi::CStr;
use std::ptr::NonNull;
crate::wrapper_impl_ref_type!(ObjEncUsrArgs, nvidia_deepstream_sys::NvDsObjEncUsrArgs);

impl ObjEncUsrArgs {
    pub fn save_image(&self) -> bool {
        self.as_native_type_ref().saveImg
    }

    pub fn attach_usr_meta(&self) -> bool {
        self.as_native_type_ref().attachUsrMeta
    }

    pub fn scale_image(&self) -> bool {
        self.as_native_type_ref().scaleImg
    }

    pub fn scaled_width(&self) -> i32 {
        self.as_native_type_ref().scaledWidth
    }

    pub fn scaled_height(&self) -> i32 {
        self.as_native_type_ref().scaledHeight
    }

    pub fn file_name_image(&self) -> &CStr {
        unsafe { CStr::from_ptr(self.as_native_type_ref().fileNameImg.as_ptr()) }
    }

    pub fn obj_num(&self) -> i32 {
        self.as_native_type_ref().objNum
    }

    pub fn quality(&self) -> i32 {
        self.as_native_type_ref().quality
    }
}

pub struct ObjEncUsrArgsBuilder<'a> {
    save_image: Option<bool>,
    attach_usr_meta: Option<bool>,
    scale_image: Option<bool>,
    scaled_width: Option<i32>,
    scaled_height: Option<i32>,
    file_name_image: Option<&'a CStr>,
    obj_num: Option<i32>,
    quality: Option<i32>,
}

impl<'a> ObjEncUsrArgsBuilder<'a> {
    pub fn new() -> Self {
        ObjEncUsrArgsBuilder {
            save_image: None,
            attach_usr_meta: None,
            scale_image: None,
            scaled_width: None,
            scaled_height: None,
            file_name_image: None,
            obj_num: None,
            quality: None,
        }
    }

    pub fn save_image(mut self, flag: bool) -> Self {
        self.save_image = Some(flag);
        self
    }

    pub fn attach_usr_meta(mut self, attach_usr_meta: bool) -> Self {
        self.attach_usr_meta = Some(attach_usr_meta);
        self
    }

    pub fn scale_image(mut self, scale_image: bool) -> Self {
        self.scale_image = Some(scale_image);
        self
    }

    pub fn scaled_width(mut self, scaled_width: i32) -> Self {
        self.scaled_width = Some(scaled_width);
        self
    }

    pub fn scaled_height(mut self, scaled_height: i32) -> Self {
        self.scaled_height = Some(scaled_height);
        self
    }

    pub fn file_name_image(mut self, file_name_image: &'a CStr) -> Self {
        self.file_name_image = Some(file_name_image);
        self
    }

    pub fn obj_num(mut self, obj_num: i32) -> Self {
        self.obj_num = Some(obj_num);
        self
    }

    pub fn quality(mut self, quality: i32) -> Self {
        self.quality = Some(quality);
        self
    }

    pub fn build(self) -> ObjEncUsrArgs {
        let mut r = ObjEncUsrArgs::from_native_type(nvidia_deepstream_sys::NvDsObjEncUsrArgs {
            saveImg: self.save_image.unwrap_or(true),
            attachUsrMeta: self.attach_usr_meta.unwrap_or_default(),
            scaleImg: self.scale_image.unwrap_or_default(),
            scaledWidth: self.scaled_width.unwrap_or(1),
            scaledHeight: self.scaled_height.unwrap_or(1),
            fileNameImg: [0; 1024],
            objNum: self.obj_num.unwrap_or_default(),
            quality: self.quality.unwrap_or_default(),
        });
        if let Some(file_name) = self.file_name_image {
            unsafe {
                std::ptr::copy(
                    file_name.as_ptr(),
                    r.0.fileNameImg.as_mut_ptr(),
                    std::cmp::min(file_name.to_bytes().len(), r.0.fileNameImg.len() - 1),
                );
            }
        }
        r
    }
}

pub struct ObjEnc(NonNull<nvidia_deepstream_sys::_NvDsObjEncCtx>);

impl Drop for ObjEnc {
    fn drop(&mut self) {
        unsafe { nvidia_deepstream_sys::nvds_obj_enc_destroy_context(self.0.as_ptr()) }
    }
}

impl ObjEnc {
    pub fn new() -> Option<ObjEnc> {
        unsafe {
            NonNull::new(nvidia_deepstream_sys::nvds_obj_enc_create_context()).map(|p| ObjEnc(p))
        }
    }

    pub fn process(
        &self,
        user_args: &ObjEncUsrArgs,
        surface: &crate::surface::Surface,
        object_meta: &crate::meta::ObjectMeta,
        frame_meta: &crate::meta::FrameMeta,
    ) -> Result<(), bool> {
        unsafe {
            if nvidia_deepstream_sys::nvds_obj_enc_process(
                self.0.as_ptr(),
                user_args.as_native_type_ptr(),
                surface.as_native_type_ptr(),
                object_meta.as_native_type_ptr(),
                frame_meta.as_native_type_ptr(),
            ) {
                Ok(())
            } else {
                Err(false)
            }
        }
    }

    pub fn finish(&self) {
        unsafe { nvidia_deepstream_sys::nvds_obj_enc_finish(self.0.as_ptr()) }
    }
}
