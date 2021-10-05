mod imports {
    pub enum Error {
        Failure(String),
        Aborted,
    }
    impl std::fmt::Debug for Error {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            match self {
                Error::Failure(e) => f.debug_tuple("Error::Failure").field(e).finish(),
                Error::Aborted => f.debug_tuple("Error::Aborted").finish(),
            }
        }
    }
    #[derive(Debug)]
    #[repr(transparent)]
    pub struct Response(i32);
    impl Response {
        pub unsafe fn from_raw(raw: i32) -> Self {
            Self(raw)
        }

        pub fn into_raw(self) -> i32 {
            let ret = self.0;
            core::mem::forget(self);
            return ret;
        }

        pub fn as_raw(&self) -> i32 {
            self.0
        }
    }
    impl Drop for Response {
        fn drop(&mut self) {
            #[link(wasm_import_module = "canonical_abi")]
            extern "C" {
                #[link_name = "resource_drop_Response"]
                fn close(fd: i32);
            }
            unsafe {
                close(self.0);
            }
        }
    }
    #[derive(Debug)]
    #[repr(transparent)]
    pub struct Headers(i32);
    impl Headers {
        pub unsafe fn from_raw(raw: i32) -> Self {
            Self(raw)
        }

        pub fn into_raw(self) -> i32 {
            let ret = self.0;
            core::mem::forget(self);
            return ret;
        }

        pub fn as_raw(&self) -> i32 {
            self.0
        }
    }
    impl Drop for Headers {
        fn drop(&mut self) {
            #[link(wasm_import_module = "canonical_abi")]
            extern "C" {
                #[link_name = "resource_drop_Headers"]
                fn close(fd: i32);
            }
            unsafe {
                close(self.0);
            }
        }
    }
    pub async fn fetch(url: &str) -> Result<Response, Error> {
        unsafe {
            let vec0 = url;
            let ptr0 = vec0.as_ptr() as i32;
            let len0 = vec0.len() as i32;
            unsafe extern "C" fn completion_callback(
                sender: usize,
                ret0: i32,
                ret1: i32,
                ret2: i32,
                ret3: i32,
            ) {
                witx_bindgen_rust::rt::Sender::from_usize(sender).send((ret0, ret1, ret2, ret3));
            }
            let (rx, tx) = witx_bindgen_rust::rt::Oneshot::<(i32, i32, i32, i32)>::new();
            #[link(wasm_import_module = "imports")]
            extern "C" {
                #[cfg_attr(target_arch = "wasm32", link_name = "fetch")]
                #[cfg_attr(not(target_arch = "wasm32"), link_name = "imports_fetch")]
                fn witx_import(_: i32, _: i32, _: i32, _: i32);
            }
            witx_import(
                ptr0,
                len0,
                completion_callback as i32,
                tx.into_usize() as i32,
            );
            let (ret1_0, ret1_1, ret1_2, ret1_3) = rx.await;
            match ret1_0 {
                0 => Ok(Response(ret1_1)),
                1 => Err(match ret1_1 {
                    0 => Error::Failure({
                        let len2 = ret1_3 as usize;

                        String::from_utf8(Vec::from_raw_parts(ret1_2 as *mut _, len2, len2))
                            .unwrap()
                    }),
                    1 => Error::Aborted,
                    _ => panic!("invalid enum discriminant"),
                }),
                _ => panic!("invalid enum discriminant"),
            }
        }
    }
    impl Response {
        pub fn headers(&self) -> Headers {
            unsafe {
                #[link(wasm_import_module = "imports")]
                extern "C" {
                    #[cfg_attr(target_arch = "wasm32", link_name = "Response::headers")]
                    #[cfg_attr(
                        not(target_arch = "wasm32"),
                        link_name = "imports_Response::headers"
                    )]
                    fn witx_import(_: i32) -> i32;
                }
                let ret = witx_import(self.0);
                Headers(ret)
            }
        }
    }
    impl Response {
        pub async fn body(&self) -> Vec<u8> {
            unsafe {
                unsafe extern "C" fn completion_callback(sender: usize, ret0: i32, ret1: i32) {
                    witx_bindgen_rust::rt::Sender::from_usize(sender).send((ret0, ret1));
                }
                let (rx, tx) = witx_bindgen_rust::rt::Oneshot::<(i32, i32)>::new();
                #[link(wasm_import_module = "imports")]
                extern "C" {
                    #[cfg_attr(target_arch = "wasm32", link_name = "Response::body")]
                    #[cfg_attr(not(target_arch = "wasm32"), link_name = "imports_Response::body")]
                    fn witx_import(_: i32, _: i32, _: i32);
                }
                witx_import(self.0, completion_callback as i32, tx.into_usize() as i32);
                let (ret0_0, ret0_1) = rx.await;
                let len1 = ret0_1 as usize;
                Vec::from_raw_parts(ret0_0 as *mut _, len1, len1)
            }
        }
    }
    impl Response {
        pub fn status(&self) -> u32 {
            unsafe {
                #[link(wasm_import_module = "imports")]
                extern "C" {
                    #[cfg_attr(target_arch = "wasm32", link_name = "Response::status")]
                    #[cfg_attr(not(target_arch = "wasm32"), link_name = "imports_Response::status")]
                    fn witx_import(_: i32) -> i32;
                }
                let ret = witx_import(self.0);
                ret as u32
            }
        }
    }
    impl Response {
        pub fn status_text(&self) -> String {
            unsafe {
                let ptr0 = RET_AREA.as_mut_ptr() as i32;
                #[link(wasm_import_module = "imports")]
                extern "C" {
                    #[cfg_attr(target_arch = "wasm32", link_name = "Response::status_text")]
                    #[cfg_attr(
                        not(target_arch = "wasm32"),
                        link_name = "imports_Response::status_text"
                    )]
                    fn witx_import(_: i32, _: i32);
                }
                witx_import(self.0, ptr0);
                let len1 = *((ptr0 + 8) as *const i32) as usize;
                String::from_utf8(Vec::from_raw_parts(
                    *((ptr0 + 0) as *const i32) as *mut _,
                    len1,
                    len1,
                ))
                .unwrap()
            }
        }
    }
    impl Headers {
        pub fn get(&self, name: &str) -> Vec<String> {
            unsafe {
                let vec0 = name;
                let ptr0 = vec0.as_ptr() as i32;
                let len0 = vec0.len() as i32;
                let ptr1 = RET_AREA.as_mut_ptr() as i32;
                #[link(wasm_import_module = "imports")]
                extern "C" {
                    #[cfg_attr(target_arch = "wasm32", link_name = "Headers::get")]
                    #[cfg_attr(not(target_arch = "wasm32"), link_name = "imports_Headers::get")]
                    fn witx_import(_: i32, _: i32, _: i32, _: i32);
                }
                witx_import(self.0, ptr0, len0, ptr1);
                let base3 = *((ptr1 + 0) as *const i32);
                let len3 = *((ptr1 + 8) as *const i32);
                let mut result3 = Vec::with_capacity(len3 as usize);
                for i in 0..len3 {
                    let base = base3 + i * 8;
                    result3.push({
                        let len2 = *((base + 4) as *const i32) as usize;

                        String::from_utf8(Vec::from_raw_parts(
                            *((base + 0) as *const i32) as *mut _,
                            len2,
                            len2,
                        ))
                        .unwrap()
                    });
                }
                std::alloc::dealloc(
                    base3 as *mut _,
                    std::alloc::Layout::from_size_align_unchecked((len3 as usize) * 8, 4),
                );
                result3
            }
        }
    }
    impl Headers {
        pub fn entries(&self) -> Vec<(String, Vec<String>)> {
            unsafe {
                let ptr0 = RET_AREA.as_mut_ptr() as i32;
                #[link(wasm_import_module = "imports")]
                extern "C" {
                    #[cfg_attr(target_arch = "wasm32", link_name = "Headers::entries")]
                    #[cfg_attr(not(target_arch = "wasm32"), link_name = "imports_Headers::entries")]
                    fn witx_import(_: i32, _: i32);
                }
                witx_import(self.0, ptr0);
                let base4 = *((ptr0 + 0) as *const i32);
                let len4 = *((ptr0 + 8) as *const i32);
                let mut result4 = Vec::with_capacity(len4 as usize);
                for i in 0..len4 {
                    let base = base4 + i * 16;
                    result4.push({
                        let len1 = *((base + 4) as *const i32) as usize;
                        let base3 = *((base + 8) as *const i32);
                        let len3 = *((base + 12) as *const i32);
                        let mut result3 = Vec::with_capacity(len3 as usize);
                        for i in 0..len3 {
                            let base = base3 + i * 8;
                            result3.push({
                                let len2 = *((base + 4) as *const i32) as usize;

                                String::from_utf8(Vec::from_raw_parts(
                                    *((base + 0) as *const i32) as *mut _,
                                    len2,
                                    len2,
                                ))
                                .unwrap()
                            });
                        }
                        std::alloc::dealloc(
                            base3 as *mut _,
                            std::alloc::Layout::from_size_align_unchecked((len3 as usize) * 8, 4),
                        );

                        (
                            String::from_utf8(Vec::from_raw_parts(
                                *((base + 0) as *const i32) as *mut _,
                                len1,
                                len1,
                            ))
                            .unwrap(),
                            result3,
                        )
                    });
                }
                std::alloc::dealloc(
                    base4 as *mut _,
                    std::alloc::Layout::from_size_align_unchecked((len4 as usize) * 16, 4),
                );
                result4
            }
        }
    }
    /// helper functions to print messages
    pub fn log(msg: &str) {
        unsafe {
            let vec0 = msg;
            let ptr0 = vec0.as_ptr() as i32;
            let len0 = vec0.len() as i32;
            #[link(wasm_import_module = "imports")]
            extern "C" {
                #[cfg_attr(target_arch = "wasm32", link_name = "log")]
                #[cfg_attr(not(target_arch = "wasm32"), link_name = "imports_log")]
                fn witx_import(_: i32, _: i32);
            }
            witx_import(ptr0, len0);
        }
    }
    pub fn log_err(msg: &str) {
        unsafe {
            let vec0 = msg;
            let ptr0 = vec0.as_ptr() as i32;
            let len0 = vec0.len() as i32;
            #[link(wasm_import_module = "imports")]
            extern "C" {
                #[cfg_attr(target_arch = "wasm32", link_name = "log_err")]
                #[cfg_attr(not(target_arch = "wasm32"), link_name = "imports_log_err")]
                fn witx_import(_: i32, _: i32);
            }
            witx_import(ptr0, len0);
        }
    }
    static mut RET_AREA: [i64; 4] = [0; 4];
}
