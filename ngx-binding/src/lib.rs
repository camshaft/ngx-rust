#![feature(const_fn, const_str_len)]

pub mod bindings;
pub mod log;
pub mod nginx;
pub mod nginx_http;

pub mod conf {
    use crate::bindings::{ngx_conf_t, ngx_pcalloc};

    impl ngx_conf_t {
        pub fn alloc<T: Sized>(&self) -> Option<*mut T> {
            let ptr = unsafe { ngx_pcalloc(self.pool, std::mem::size_of::<T>()) };

            if ptr.is_null() {
                None
            } else {
                Some(ptr as *mut T)
            }
        }
    }
}

pub mod command {
    use crate::bindings::{ngx_command_t, ngx_str_t};

    pub const NULL: ngx_command_t = ngx_command_t::new();

    unsafe impl Sync for ngx_command_t {}

    impl ngx_command_t {
        pub const fn new() -> Self {
            ngx_command_t {
                name: ngx_str_t::new(),
                type_: 0,
                set: None,
                conf: 0,
                offset: 0,
                post: std::ptr::null_mut(),
            }
        }
    }

}
