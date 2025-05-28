use core::ffi::CStr;

use crate::{LvError, LvResult, NativeObject, Obj, Part, Widget};

pub struct MessageBox<'a> {
    raw: Obj<'a>,
}

impl MessageBox<'_> {
    pub fn create(
        parent: &mut impl crate::NativeObject,
        title: &CStr,
        txt: &CStr,
        btn_txts: *mut *const core::ffi::c_char,
        add_close_btn: bool,
    ) -> LvResult<Self> {
        unsafe {
            let ptr = lvgl_sys::lv_msgbox_create(
                parent.raw().as_mut(),
                title.as_ptr(),
                txt.as_ptr(),
                btn_txts,
                add_close_btn,
            );
            if let Some(raw) = core::ptr::NonNull::new(ptr) {
                Ok(Self {
                    raw: Obj::from_raw(raw).unwrap(),
                })
            } else {
                Err(crate::LvError::InvalidReference)
            }
        }
    }
}

impl NativeObject for MessageBox<'_> {
    fn raw(&self) -> core::ptr::NonNull<lvgl_sys::lv_obj_t> {
        self.raw.raw()
    }
}

impl<'a> Widget<'a> for MessageBox<'a> {
    type SpecialEvent = u32;
    type Part = Part;

    unsafe fn from_raw(raw: core::ptr::NonNull<lvgl_sys::lv_obj_t>) -> Option<Self> {
        Self::try_from(Obj::from_raw(raw)?).ok()
    }
}

impl<'a> TryFrom<Obj<'a>> for MessageBox<'a> {
    type Error = LvError;

    fn try_from(value: Obj<'a>) -> Result<Self, Self::Error> {
        match unsafe { value.raw().as_mut().parent } as usize {
            0 => Ok(Self { raw: value }),
            _ => Err(LvError::InvalidReference),
        }
    }
}
