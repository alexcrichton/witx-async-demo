mod exports {
    pub enum Error {
        BadStatus(String),
        NotUtf8(String),
        Io(String),
    }
    impl std::fmt::Debug for Error {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            match self {
                Error::BadStatus(e) => f.debug_tuple("Error::BadStatus").field(e).finish(),
                Error::NotUtf8(e) => f.debug_tuple("Error::NotUtf8").field(e).finish(),
                Error::Io(e) => f.debug_tuple("Error::Io").field(e).finish(),
            }
        }
    }

    unsafe impl witx_bindgen_rust::HandleType for super::Tarball {
        #[inline]
        fn clone(_val: i32) -> i32 {
            #[cfg(not(target_arch = "wasm32"))]
            {
                panic!("handles can only be used on wasm32");
            }
            #[cfg(target_arch = "wasm32")]
            {
                #[link(wasm_import_module = "canonical_abi")]
                extern "C" {
                    #[link_name = "resource_clone_Tarball"]
                    fn clone(val: i32) -> i32;
                }
                unsafe { clone(_val) }
            }
        }

        #[inline]
        fn drop(_val: i32) {
            #[cfg(not(target_arch = "wasm32"))]
            {
                panic!("handles can only be used on wasm32");
            }
            #[cfg(target_arch = "wasm32")]
            {
                #[link(wasm_import_module = "canonical_abi")]
                extern "C" {
                    #[link_name = "resource_drop_Tarball"]
                    fn drop(val: i32);
                }
                unsafe { drop(_val) }
            }
        }
    }

    unsafe impl witx_bindgen_rust::LocalHandle for super::Tarball {
        #[inline]
        fn new(_val: i32) -> i32 {
            #[cfg(not(target_arch = "wasm32"))]
            {
                panic!("handles can only be used on wasm32");
            }
            #[cfg(target_arch = "wasm32")]
            {
                #[link(wasm_import_module = "canonical_abi")]
                extern "C" {
                    #[link_name = "resource_new_Tarball"]
                    fn new(val: i32) -> i32;
                }
                unsafe { new(_val) }
            }
        }

        #[inline]
        fn get(_val: i32) -> i32 {
            #[cfg(not(target_arch = "wasm32"))]
            {
                panic!("handles can only be used on wasm32");
            }
            #[cfg(target_arch = "wasm32")]
            {
                #[link(wasm_import_module = "canonical_abi")]
                extern "C" {
                    #[link_name = "resource_get_Tarball"]
                    fn get(val: i32) -> i32;
                }
                unsafe { get(_val) }
            }
        }
    }

    const _: () = {
        #[export_name = "canonical_abi_drop_Tarball"]
        extern "C" fn drop(ty: Box<super::Tarball>) {
            <super::Exports as Exports>::drop_tarball(*ty)
        }
    };
    #[export_name = "Tarball::fetch"]
    unsafe extern "C" fn __witx_bindgen_tarball_fetch(arg0: i32, arg1: i32, arg2: i32) {
        let future = async move {
            let len0 = arg1 as usize;
            let result1 = <super::Tarball as Tarball>::fetch(
                String::from_utf8(Vec::from_raw_parts(arg0 as *mut _, len0, len0)).unwrap(),
            )
            .await;
            let (result6_0, result6_1, result6_2, result6_3) = match result1 {
                Ok(e) => (0i32, witx_bindgen_rust::Handle::into_raw(e), 0i32, 0i32),
                Err(e) => {
                    let (result5_0, result5_1, result5_2) = match e {
                        Error::BadStatus(e) => {
                            let vec2 = (e.into_bytes()).into_boxed_slice();
                            let ptr2 = vec2.as_ptr() as i32;
                            let len2 = vec2.len() as i32;
                            core::mem::forget(vec2);

                            (0i32, ptr2, len2)
                        }
                        Error::NotUtf8(e) => {
                            let vec3 = (e.into_bytes()).into_boxed_slice();
                            let ptr3 = vec3.as_ptr() as i32;
                            let len3 = vec3.len() as i32;
                            core::mem::forget(vec3);

                            (1i32, ptr3, len3)
                        }
                        Error::Io(e) => {
                            let vec4 = (e.into_bytes()).into_boxed_slice();
                            let ptr4 = vec4.as_ptr() as i32;
                            let len4 = vec4.len() as i32;
                            core::mem::forget(vec4);

                            (2i32, ptr4, len4)
                        }
                    };

                    (1i32, result5_0, result5_1, result5_2)
                }
            };
            let ptr7 = RET_AREA.as_mut_ptr() as i32;
            *((ptr7 + 24) as *mut i32) = result6_3;
            *((ptr7 + 16) as *mut i32) = result6_2;
            *((ptr7 + 8) as *mut i32) = result6_1;
            *((ptr7 + 0) as *mut i32) = result6_0;
            unsafe {
                witx_bindgen_rust::rt::async_export_done(arg2, ptr7);
            }
        };
        witx_bindgen_rust::rt::execute(Box::pin(future));
    }
    #[export_name = "Tarball::files"]
    unsafe extern "C" fn __witx_bindgen_tarball_files(arg0: i32) -> i32 {
        let result0 =
            <super::Tarball as Tarball>::files(&witx_bindgen_rust::Handle::from_raw(arg0));
        let vec2 = result0;
        let len2 = vec2.len() as i32;
        let layout2 = core::alloc::Layout::from_size_align_unchecked(vec2.len() * 8, 4);
        let result2 = std::alloc::alloc(layout2);
        if result2.is_null() {
            std::alloc::handle_alloc_error(layout2);
        }
        for (i, e) in vec2.into_iter().enumerate() {
            let base = result2 as i32 + (i as i32) * 8;
            {
                let vec1 = (e.into_bytes()).into_boxed_slice();
                let ptr1 = vec1.as_ptr() as i32;
                let len1 = vec1.len() as i32;
                core::mem::forget(vec1);
                *((base + 4) as *mut i32) = len1;
                *((base + 0) as *mut i32) = ptr1;
            }
        }
        let ptr3 = RET_AREA.as_mut_ptr() as i32;
        *((ptr3 + 8) as *mut i32) = len2;
        *((ptr3 + 0) as *mut i32) = result2 as i32;
        ptr3
    }
    #[export_name = "Tarball::contents"]
    unsafe extern "C" fn __witx_bindgen_tarball_contents(arg0: i32, arg1: i32) -> i32 {
        let result0 = <super::Tarball as Tarball>::contents(
            &witx_bindgen_rust::Handle::from_raw(arg0),
            arg1 as u32,
        );
        let vec1 = (result0.into_bytes()).into_boxed_slice();
        let ptr1 = vec1.as_ptr() as i32;
        let len1 = vec1.len() as i32;
        core::mem::forget(vec1);
        let ptr2 = RET_AREA.as_mut_ptr() as i32;
        *((ptr2 + 8) as *mut i32) = len1;
        *((ptr2 + 0) as *mut i32) = ptr1;
        ptr2
    }
    #[witx_bindgen_rust::async_trait(?Send)]
    pub trait Exports {
        /// An optional callback invoked when a handle is finalized
        /// and destroyed.
        fn drop_tarball(val: super::Tarball) {
            drop(val);
        }
    }
    #[witx_bindgen_rust::async_trait(?Send)]
    pub trait Tarball {
        async fn fetch(url: String) -> Result<witx_bindgen_rust::Handle<super::Tarball>, Error>;
        fn files(&self) -> Vec<String>;
        fn contents(&self, idx: u32) -> String;
    }
    static mut RET_AREA: [i64; 4] = [0; 4];
}
