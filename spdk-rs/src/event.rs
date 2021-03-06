/*************************************************************************
 > File Name:       event.rs
 > Author:          Zeyuan Hu
 > Mail:            iamzeyuanhu@utexas.edu
 > Created Time:    10/2/18
 > Description:

   FFI for "event.h"
************************************************************************/

use crate::raw;

use failure::Error;
use std::ffi::CString;
use std::os::raw::c_void;
use std::ptr;

#[derive(Debug, Fail)]
enum AppError {
    #[fail(display = "Spdk failed to start: {}", _0)]
    StartupError(i32),
}

#[derive(Default)]
pub struct SpdkAppOpts(raw::spdk_app_opts);

impl SpdkAppOpts {
    pub fn new() -> Self {
        let mut opts: raw::spdk_app_opts = Default::default();
        unsafe {
            raw::spdk_app_opts_init(&mut opts as *mut raw::spdk_app_opts);
        }
        SpdkAppOpts(opts)
    }

    pub fn name(&mut self, name: &str) {
        self.0.name = CString::new(name)
            .expect("Couldn't create a string")
            .into_raw()
    }

    pub fn config_file(&mut self, config_file: &str) {
        self.0.config_file = CString::new(config_file)
            .expect("Couldn't create a string")
            .into_raw()
    }

    pub fn start<F>(mut self, f: F) -> Result<(), Error>
    where
        F: FnMut() -> (),
    {
        let user_data = &f as *const _ as *mut c_void;

        extern "C" fn start_wrapper<F>(closure: *mut c_void, _: *mut c_void)
        where
            F: FnMut() -> (),
        {
            let opt_closure = closure as *mut F;
            unsafe { (*opt_closure)() }
        }

        let ret = unsafe {
            let self_ref = &mut self;
            let opts_ref = &mut self_ref.0;
            raw::spdk_app_start(
                opts_ref as *mut raw::spdk_app_opts,
                Some(start_wrapper::<F>),
                user_data,
                ptr::null_mut(),
            )
        };

        unsafe {
            //            if (context.buff != ptr::null_mut()) {
            //                raw::spdk_dma_free(context.buff as *mut c_void);
            //            }
            raw::spdk_app_fini();
        }

        if ret == 0 {
            Ok(())
        } else {
            Err(AppError::StartupError(ret))?
        }
    }
}

pub fn app_stop(success: bool) {
    unsafe {
        raw::spdk_app_stop(if success { 0 } else { -1 });
    };
}

//impl Drop for SpdAppOpts {
//    fn drop(&mut self) {
//        drop_if_not_null(self.0.name as *mut c_char);
//        drop_if_not_null(self.0.config_file as *mut c_char);
//    }
//}
//
//fn drop_if_not_null(string: *mut c_char) {
//    if !string.is_null() {
//        unsafe { CString::from_raw(string as *mut c_char) };
//    }
//}
