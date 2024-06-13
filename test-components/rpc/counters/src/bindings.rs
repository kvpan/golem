// Generated by `wit-bindgen` 0.16.0. DO NOT EDIT!
pub mod exports {
  pub mod rpc {
    pub mod counters {
      
      #[allow(clippy::all)]
      pub mod api {
        #[used]
        #[doc(hidden)]
        #[cfg(target_arch = "wasm32")]
        static __FORCE_SECTION_REF: fn() = super::super::super::super::__link_section;
        
        pub use super::super::super::super::super::Counter as Counter;
        const _: () = {
          #[doc(hidden)]
          #[export_name = "rpc:counters/api#[dtor]counter"]
          #[allow(non_snake_case)]
          unsafe extern "C" fn dtor(rep: usize) {
            wit_bindgen::rt::Resource::<Counter>::dtor(rep)
          }
        };
        unsafe impl wit_bindgen::rt::RustResource for Counter{
          unsafe fn new(_rep: usize) -> u32 {
            #[cfg(not(target_arch = "wasm32"))]
            unreachable!();
            
            #[cfg(target_arch = "wasm32")]
            {
              #[link(wasm_import_module = "[export]rpc:counters/api")]
              extern "C" {
                #[link_name = "[resource-new]counter"]
                fn new(_: usize) -> u32;
              }
              new(_rep)
            }
          }
          
          unsafe fn rep(_handle: u32) -> usize {
            #[cfg(not(target_arch = "wasm32"))]
            unreachable!();
            
            #[cfg(target_arch = "wasm32")]
            {
              #[link(wasm_import_module = "[export]rpc:counters/api")]
              extern "C" {
                #[link_name = "[resource-rep]counter"]
                fn rep(_: u32) -> usize;
              }
              rep(_handle)
            }
          }
        }
        pub type OwnCounter = wit_bindgen::rt::Resource<Counter>;
        
        
        unsafe impl wit_bindgen::rt::WasmResource for Counter{
          #[inline]
          unsafe fn drop(_handle: u32) {
            #[cfg(not(target_arch = "wasm32"))]
            unreachable!();
            
            #[cfg(target_arch = "wasm32")]
            {
              #[link(wasm_import_module = "[export]rpc:counters/api")]
              extern "C" {
                #[link_name = "[resource-drop]counter"]
                fn drop(_: u32);
              }
              
              drop(_handle);
            }
          }
        }
        
        const _: () = {
          
          #[doc(hidden)]
          #[export_name = "rpc:counters/api#[constructor]counter"]
          #[allow(non_snake_case)]
          unsafe extern "C" fn __export_constructor_counter(arg0: i32,arg1: i32,) -> i32 {
            #[allow(unused_imports)]
            use wit_bindgen::rt::{alloc, vec::Vec, string::String};
            
            // Before executing any other code, use this function to run all static
            // constructors, if they have not yet been run. This is a hack required
            // to work around wasi-libc ctors calling import functions to initialize
            // the environment.
            //
            // This functionality will be removed once rust 1.69.0 is stable, at which
            // point wasi-libc will no longer have this behavior.
            //
            // See
            // https://github.com/bytecodealliance/preview2-prototyping/issues/99
            // for more details.
            #[cfg(target_arch="wasm32")]
            wit_bindgen::rt::run_ctors_once();
            
            let len0 = arg1 as usize;
            let bytes0 = Vec::from_raw_parts(arg0 as *mut _, len0, len0);
            let result1 = OwnCounter::new(<_CounterImpl as GuestCounter>::new(wit_bindgen::rt::string_lift(bytes0)));
            wit_bindgen::rt::Resource::into_handle(result1) as i32
          }
        };
        const _: () = {
          
          #[doc(hidden)]
          #[export_name = "rpc:counters/api#[method]counter.inc-by"]
          #[allow(non_snake_case)]
          unsafe extern "C" fn __export_method_counter_inc_by(arg0: i32,arg1: i64,) {
            #[allow(unused_imports)]
            use wit_bindgen::rt::{alloc, vec::Vec, string::String};
            
            // Before executing any other code, use this function to run all static
            // constructors, if they have not yet been run. This is a hack required
            // to work around wasi-libc ctors calling import functions to initialize
            // the environment.
            //
            // This functionality will be removed once rust 1.69.0 is stable, at which
            // point wasi-libc will no longer have this behavior.
            //
            // See
            // https://github.com/bytecodealliance/preview2-prototyping/issues/99
            // for more details.
            #[cfg(target_arch="wasm32")]
            wit_bindgen::rt::run_ctors_once();
            
            <_CounterImpl as GuestCounter>::inc_by(wit_bindgen::rt::Resource::<Counter>::lift_borrow(arg0 as u32 as usize), arg1 as u64);
          }
        };
        const _: () = {
          
          #[doc(hidden)]
          #[export_name = "rpc:counters/api#[method]counter.get-value"]
          #[allow(non_snake_case)]
          unsafe extern "C" fn __export_method_counter_get_value(arg0: i32,) -> i64 {
            #[allow(unused_imports)]
            use wit_bindgen::rt::{alloc, vec::Vec, string::String};
            
            // Before executing any other code, use this function to run all static
            // constructors, if they have not yet been run. This is a hack required
            // to work around wasi-libc ctors calling import functions to initialize
            // the environment.
            //
            // This functionality will be removed once rust 1.69.0 is stable, at which
            // point wasi-libc will no longer have this behavior.
            //
            // See
            // https://github.com/bytecodealliance/preview2-prototyping/issues/99
            // for more details.
            #[cfg(target_arch="wasm32")]
            wit_bindgen::rt::run_ctors_once();
            
            let result0 = <_CounterImpl as GuestCounter>::get_value(wit_bindgen::rt::Resource::<Counter>::lift_borrow(arg0 as u32 as usize));
            wit_bindgen::rt::as_i64(result0)
          }
        };
        const _: () = {
          
          #[doc(hidden)]
          #[export_name = "rpc:counters/api#[method]counter.get-args"]
          #[allow(non_snake_case)]
          unsafe extern "C" fn __export_method_counter_get_args(arg0: i32,) -> i32 {
            #[allow(unused_imports)]
            use wit_bindgen::rt::{alloc, vec::Vec, string::String};
            
            // Before executing any other code, use this function to run all static
            // constructors, if they have not yet been run. This is a hack required
            // to work around wasi-libc ctors calling import functions to initialize
            // the environment.
            //
            // This functionality will be removed once rust 1.69.0 is stable, at which
            // point wasi-libc will no longer have this behavior.
            //
            // See
            // https://github.com/bytecodealliance/preview2-prototyping/issues/99
            // for more details.
            #[cfg(target_arch="wasm32")]
            wit_bindgen::rt::run_ctors_once();
            
            let result0 = <_CounterImpl as GuestCounter>::get_args(wit_bindgen::rt::Resource::<Counter>::lift_borrow(arg0 as u32 as usize));
            let ptr1 = _RET_AREA.0.as_mut_ptr() as i32;
            let vec3 = result0;
            let len3 = vec3.len() as i32;
            let layout3 = alloc::Layout::from_size_align_unchecked(vec3.len() * 8, 4);
            let result3 = if layout3.size() != 0
            {
              let ptr = alloc::alloc(layout3);
              if ptr.is_null()
              {
                alloc::handle_alloc_error(layout3);
              }
              ptr
            }else {{
              ::core::ptr::null_mut()
            }};
            for (i, e) in vec3.into_iter().enumerate() {
              let base = result3 as i32 + (i as i32) * 8;
              {
                let vec2 = (e.into_bytes()).into_boxed_slice();
                let ptr2 = vec2.as_ptr() as i32;
                let len2 = vec2.len() as i32;
                ::core::mem::forget(vec2);
                *((base + 4) as *mut i32) = len2;
                *((base + 0) as *mut i32) = ptr2;
              }
            }
            *((ptr1 + 4) as *mut i32) = len3;
            *((ptr1 + 0) as *mut i32) = result3 as i32;
            ptr1
          }
          
          const _: () = {
            #[doc(hidden)]
            #[export_name = "cabi_post_rpc:counters/api#[method]counter.get-args"]
            #[allow(non_snake_case)]
            unsafe extern "C" fn __post_return_method_counter_get_args(arg0: i32,) {
              let l2 = *((arg0 + 0) as *const i32);
              let l3 = *((arg0 + 4) as *const i32);
              let base4 = l2;
              let len4 = l3;
              for i in 0..len4 {
                let base = base4 + i *8;
                {
                  let l0 = *((base + 0) as *const i32);
                  let l1 = *((base + 4) as *const i32);
                  wit_bindgen::rt::dealloc(l0, (l1) as usize, 1);
                }
              }
              wit_bindgen::rt::dealloc(base4, (len4 as usize) * 8, 4);
            }
          };
        };
        const _: () = {
          
          #[doc(hidden)]
          #[export_name = "rpc:counters/api#[method]counter.get-env"]
          #[allow(non_snake_case)]
          unsafe extern "C" fn __export_method_counter_get_env(arg0: i32,) -> i32 {
            #[allow(unused_imports)]
            use wit_bindgen::rt::{alloc, vec::Vec, string::String};
            
            // Before executing any other code, use this function to run all static
            // constructors, if they have not yet been run. This is a hack required
            // to work around wasi-libc ctors calling import functions to initialize
            // the environment.
            //
            // This functionality will be removed once rust 1.69.0 is stable, at which
            // point wasi-libc will no longer have this behavior.
            //
            // See
            // https://github.com/bytecodealliance/preview2-prototyping/issues/99
            // for more details.
            #[cfg(target_arch="wasm32")]
            wit_bindgen::rt::run_ctors_once();
            
            let result0 = <_CounterImpl as GuestCounter>::get_env(wit_bindgen::rt::Resource::<Counter>::lift_borrow(arg0 as u32 as usize));
            let ptr1 = _RET_AREA.0.as_mut_ptr() as i32;
            let vec5 = result0;
            let len5 = vec5.len() as i32;
            let layout5 = alloc::Layout::from_size_align_unchecked(vec5.len() * 16, 4);
            let result5 = if layout5.size() != 0
            {
              let ptr = alloc::alloc(layout5);
              if ptr.is_null()
              {
                alloc::handle_alloc_error(layout5);
              }
              ptr
            }else {{
              ::core::ptr::null_mut()
            }};
            for (i, e) in vec5.into_iter().enumerate() {
              let base = result5 as i32 + (i as i32) * 16;
              {
                let (t2_0, t2_1, ) = e;
                let vec3 = (t2_0.into_bytes()).into_boxed_slice();
                let ptr3 = vec3.as_ptr() as i32;
                let len3 = vec3.len() as i32;
                ::core::mem::forget(vec3);
                *((base + 4) as *mut i32) = len3;
                *((base + 0) as *mut i32) = ptr3;
                let vec4 = (t2_1.into_bytes()).into_boxed_slice();
                let ptr4 = vec4.as_ptr() as i32;
                let len4 = vec4.len() as i32;
                ::core::mem::forget(vec4);
                *((base + 12) as *mut i32) = len4;
                *((base + 8) as *mut i32) = ptr4;
              }
            }
            *((ptr1 + 4) as *mut i32) = len5;
            *((ptr1 + 0) as *mut i32) = result5 as i32;
            ptr1
          }
          
          const _: () = {
            #[doc(hidden)]
            #[export_name = "cabi_post_rpc:counters/api#[method]counter.get-env"]
            #[allow(non_snake_case)]
            unsafe extern "C" fn __post_return_method_counter_get_env(arg0: i32,) {
              let l4 = *((arg0 + 0) as *const i32);
              let l5 = *((arg0 + 4) as *const i32);
              let base6 = l4;
              let len6 = l5;
              for i in 0..len6 {
                let base = base6 + i *16;
                {
                  let l0 = *((base + 0) as *const i32);
                  let l1 = *((base + 4) as *const i32);
                  wit_bindgen::rt::dealloc(l0, (l1) as usize, 1);
                  let l2 = *((base + 8) as *const i32);
                  let l3 = *((base + 12) as *const i32);
                  wit_bindgen::rt::dealloc(l2, (l3) as usize, 1);
                }
              }
              wit_bindgen::rt::dealloc(base6, (len6 as usize) * 16, 4);
            }
          };
        };
        const _: () = {
          
          #[doc(hidden)]
          #[export_name = "rpc:counters/api#inc-global-by"]
          #[allow(non_snake_case)]
          unsafe extern "C" fn __export_inc_global_by(arg0: i64,) {
            #[allow(unused_imports)]
            use wit_bindgen::rt::{alloc, vec::Vec, string::String};
            
            // Before executing any other code, use this function to run all static
            // constructors, if they have not yet been run. This is a hack required
            // to work around wasi-libc ctors calling import functions to initialize
            // the environment.
            //
            // This functionality will be removed once rust 1.69.0 is stable, at which
            // point wasi-libc will no longer have this behavior.
            //
            // See
            // https://github.com/bytecodealliance/preview2-prototyping/issues/99
            // for more details.
            #[cfg(target_arch="wasm32")]
            wit_bindgen::rt::run_ctors_once();
            
            <_GuestImpl as Guest>::inc_global_by(arg0 as u64);
          }
        };
        const _: () = {
          
          #[doc(hidden)]
          #[export_name = "rpc:counters/api#get-global-value"]
          #[allow(non_snake_case)]
          unsafe extern "C" fn __export_get_global_value() -> i64 {
            #[allow(unused_imports)]
            use wit_bindgen::rt::{alloc, vec::Vec, string::String};
            
            // Before executing any other code, use this function to run all static
            // constructors, if they have not yet been run. This is a hack required
            // to work around wasi-libc ctors calling import functions to initialize
            // the environment.
            //
            // This functionality will be removed once rust 1.69.0 is stable, at which
            // point wasi-libc will no longer have this behavior.
            //
            // See
            // https://github.com/bytecodealliance/preview2-prototyping/issues/99
            // for more details.
            #[cfg(target_arch="wasm32")]
            wit_bindgen::rt::run_ctors_once();
            
            let result0 = <_GuestImpl as Guest>::get_global_value();
            wit_bindgen::rt::as_i64(result0)
          }
        };
        const _: () = {
          
          #[doc(hidden)]
          #[export_name = "rpc:counters/api#get-all-dropped"]
          #[allow(non_snake_case)]
          unsafe extern "C" fn __export_get_all_dropped() -> i32 {
            #[allow(unused_imports)]
            use wit_bindgen::rt::{alloc, vec::Vec, string::String};
            
            // Before executing any other code, use this function to run all static
            // constructors, if they have not yet been run. This is a hack required
            // to work around wasi-libc ctors calling import functions to initialize
            // the environment.
            //
            // This functionality will be removed once rust 1.69.0 is stable, at which
            // point wasi-libc will no longer have this behavior.
            //
            // See
            // https://github.com/bytecodealliance/preview2-prototyping/issues/99
            // for more details.
            #[cfg(target_arch="wasm32")]
            wit_bindgen::rt::run_ctors_once();
            
            let result0 = <_GuestImpl as Guest>::get_all_dropped();
            let ptr1 = _RET_AREA.0.as_mut_ptr() as i32;
            let vec4 = result0;
            let len4 = vec4.len() as i32;
            let layout4 = alloc::Layout::from_size_align_unchecked(vec4.len() * 16, 8);
            let result4 = if layout4.size() != 0
            {
              let ptr = alloc::alloc(layout4);
              if ptr.is_null()
              {
                alloc::handle_alloc_error(layout4);
              }
              ptr
            }else {{
              ::core::ptr::null_mut()
            }};
            for (i, e) in vec4.into_iter().enumerate() {
              let base = result4 as i32 + (i as i32) * 16;
              {
                let (t2_0, t2_1, ) = e;
                let vec3 = (t2_0.into_bytes()).into_boxed_slice();
                let ptr3 = vec3.as_ptr() as i32;
                let len3 = vec3.len() as i32;
                ::core::mem::forget(vec3);
                *((base + 4) as *mut i32) = len3;
                *((base + 0) as *mut i32) = ptr3;
                *((base + 8) as *mut i64) = wit_bindgen::rt::as_i64(t2_1);
              }
            }
            *((ptr1 + 4) as *mut i32) = len4;
            *((ptr1 + 0) as *mut i32) = result4 as i32;
            ptr1
          }
          
          const _: () = {
            #[doc(hidden)]
            #[export_name = "cabi_post_rpc:counters/api#get-all-dropped"]
            #[allow(non_snake_case)]
            unsafe extern "C" fn __post_return_get_all_dropped(arg0: i32,) {
              let l2 = *((arg0 + 0) as *const i32);
              let l3 = *((arg0 + 4) as *const i32);
              let base4 = l2;
              let len4 = l3;
              for i in 0..len4 {
                let base = base4 + i *16;
                {
                  let l0 = *((base + 0) as *const i32);
                  let l1 = *((base + 4) as *const i32);
                  wit_bindgen::rt::dealloc(l0, (l1) as usize, 1);
                }
              }
              wit_bindgen::rt::dealloc(base4, (len4 as usize) * 16, 8);
            }
          };
        };
        use super::super::super::super::super::Component as _GuestImpl;
        pub trait Guest {
          fn inc_global_by(value: u64,);
          fn get_global_value() -> u64;
          fn get_all_dropped() -> wit_bindgen::rt::vec::Vec::<(wit_bindgen::rt::string::String,u64,)>;
        }
        use super::super::super::super::super::Counter as _CounterImpl;
        pub trait GuestCounter {
          fn new(name: wit_bindgen::rt::string::String,) -> Self;
          fn inc_by(&self,value: u64,);
          fn get_value(&self,) -> u64;
          fn get_args(&self,) -> wit_bindgen::rt::vec::Vec::<wit_bindgen::rt::string::String>;
          fn get_env(&self,) -> wit_bindgen::rt::vec::Vec::<(wit_bindgen::rt::string::String,wit_bindgen::rt::string::String,)>;
        }
        
        #[allow(unused_imports)]
        use wit_bindgen::rt::{alloc, vec::Vec, string::String};
        
        #[repr(align(4))]
        struct _RetArea([u8; 8]);
        static mut _RET_AREA: _RetArea = _RetArea([0; 8]);
        
      }
      
    }
  }
}

#[cfg(target_arch = "wasm32")]
#[link_section = "component-type:counters"]
#[doc(hidden)]
pub static __WIT_BINDGEN_COMPONENT_TYPE: [u8; 863] = [3, 0, 8, 99, 111, 117, 110, 116, 101, 114, 115, 0, 97, 115, 109, 13, 0, 1, 0, 7, 218, 2, 1, 65, 2, 1, 66, 24, 4, 0, 7, 99, 111, 117, 110, 116, 101, 114, 3, 1, 1, 105, 0, 1, 64, 1, 4, 110, 97, 109, 101, 115, 0, 1, 4, 0, 20, 91, 99, 111, 110, 115, 116, 114, 117, 99, 116, 111, 114, 93, 99, 111, 117, 110, 116, 101, 114, 1, 2, 1, 104, 0, 1, 64, 2, 4, 115, 101, 108, 102, 3, 5, 118, 97, 108, 117, 101, 119, 1, 0, 4, 0, 22, 91, 109, 101, 116, 104, 111, 100, 93, 99, 111, 117, 110, 116, 101, 114, 46, 105, 110, 99, 45, 98, 121, 1, 4, 1, 64, 1, 4, 115, 101, 108, 102, 3, 0, 119, 4, 0, 25, 91, 109, 101, 116, 104, 111, 100, 93, 99, 111, 117, 110, 116, 101, 114, 46, 103, 101, 116, 45, 118, 97, 108, 117, 101, 1, 5, 1, 112, 115, 1, 64, 1, 4, 115, 101, 108, 102, 3, 0, 6, 4, 0, 24, 91, 109, 101, 116, 104, 111, 100, 93, 99, 111, 117, 110, 116, 101, 114, 46, 103, 101, 116, 45, 97, 114, 103, 115, 1, 7, 1, 111, 2, 115, 115, 1, 112, 8, 1, 64, 1, 4, 115, 101, 108, 102, 3, 0, 9, 4, 0, 23, 91, 109, 101, 116, 104, 111, 100, 93, 99, 111, 117, 110, 116, 101, 114, 46, 103, 101, 116, 45, 101, 110, 118, 1, 10, 1, 64, 1, 5, 118, 97, 108, 117, 101, 119, 1, 0, 4, 0, 13, 105, 110, 99, 45, 103, 108, 111, 98, 97, 108, 45, 98, 121, 1, 11, 1, 64, 0, 0, 119, 4, 0, 16, 103, 101, 116, 45, 103, 108, 111, 98, 97, 108, 45, 118, 97, 108, 117, 101, 1, 12, 1, 111, 2, 115, 119, 1, 112, 13, 1, 64, 0, 0, 14, 4, 0, 15, 103, 101, 116, 45, 97, 108, 108, 45, 100, 114, 111, 112, 112, 101, 100, 1, 15, 4, 1, 16, 114, 112, 99, 58, 99, 111, 117, 110, 116, 101, 114, 115, 47, 97, 112, 105, 5, 0, 11, 9, 1, 0, 3, 97, 112, 105, 3, 0, 0, 7, 247, 2, 1, 65, 2, 1, 65, 2, 1, 66, 24, 4, 0, 7, 99, 111, 117, 110, 116, 101, 114, 3, 1, 1, 105, 0, 1, 64, 1, 4, 110, 97, 109, 101, 115, 0, 1, 4, 0, 20, 91, 99, 111, 110, 115, 116, 114, 117, 99, 116, 111, 114, 93, 99, 111, 117, 110, 116, 101, 114, 1, 2, 1, 104, 0, 1, 64, 2, 4, 115, 101, 108, 102, 3, 5, 118, 97, 108, 117, 101, 119, 1, 0, 4, 0, 22, 91, 109, 101, 116, 104, 111, 100, 93, 99, 111, 117, 110, 116, 101, 114, 46, 105, 110, 99, 45, 98, 121, 1, 4, 1, 64, 1, 4, 115, 101, 108, 102, 3, 0, 119, 4, 0, 25, 91, 109, 101, 116, 104, 111, 100, 93, 99, 111, 117, 110, 116, 101, 114, 46, 103, 101, 116, 45, 118, 97, 108, 117, 101, 1, 5, 1, 112, 115, 1, 64, 1, 4, 115, 101, 108, 102, 3, 0, 6, 4, 0, 24, 91, 109, 101, 116, 104, 111, 100, 93, 99, 111, 117, 110, 116, 101, 114, 46, 103, 101, 116, 45, 97, 114, 103, 115, 1, 7, 1, 111, 2, 115, 115, 1, 112, 8, 1, 64, 1, 4, 115, 101, 108, 102, 3, 0, 9, 4, 0, 23, 91, 109, 101, 116, 104, 111, 100, 93, 99, 111, 117, 110, 116, 101, 114, 46, 103, 101, 116, 45, 101, 110, 118, 1, 10, 1, 64, 1, 5, 118, 97, 108, 117, 101, 119, 1, 0, 4, 0, 13, 105, 110, 99, 45, 103, 108, 111, 98, 97, 108, 45, 98, 121, 1, 11, 1, 64, 0, 0, 119, 4, 0, 16, 103, 101, 116, 45, 103, 108, 111, 98, 97, 108, 45, 118, 97, 108, 117, 101, 1, 12, 1, 111, 2, 115, 119, 1, 112, 13, 1, 64, 0, 0, 14, 4, 0, 15, 103, 101, 116, 45, 97, 108, 108, 45, 100, 114, 111, 112, 112, 101, 100, 1, 15, 4, 1, 16, 114, 112, 99, 58, 99, 111, 117, 110, 116, 101, 114, 115, 47, 97, 112, 105, 5, 0, 4, 1, 21, 114, 112, 99, 58, 99, 111, 117, 110, 116, 101, 114, 115, 47, 99, 111, 117, 110, 116, 101, 114, 115, 4, 0, 11, 14, 1, 0, 8, 99, 111, 117, 110, 116, 101, 114, 115, 3, 2, 0, 0, 16, 12, 112, 97, 99, 107, 97, 103, 101, 45, 100, 111, 99, 115, 0, 123, 125, 0, 70, 9, 112, 114, 111, 100, 117, 99, 101, 114, 115, 1, 12, 112, 114, 111, 99, 101, 115, 115, 101, 100, 45, 98, 121, 2, 13, 119, 105, 116, 45, 99, 111, 109, 112, 111, 110, 101, 110, 116, 6, 48, 46, 49, 56, 46, 50, 16, 119, 105, 116, 45, 98, 105, 110, 100, 103, 101, 110, 45, 114, 117, 115, 116, 6, 48, 46, 49, 54, 46, 48];

#[inline(never)]
#[doc(hidden)]
#[cfg(target_arch = "wasm32")]
pub fn __link_section() {}
