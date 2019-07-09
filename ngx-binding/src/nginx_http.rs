use bindings::ngx_command_t;
use bindings::ngx_cycle;
use bindings::ngx_http_headers_in_t;
use bindings::ngx_http_headers_out_t;
use bindings::ngx_http_module_t;
use bindings::ngx_http_request_s;
use bindings::ngx_list_part_t;
use bindings::ngx_list_t;
use bindings::ngx_log_error_core;
use bindings::ngx_log_t;
use bindings::ngx_module_s;
use bindings::ngx_str_t;
use bindings::ngx_table_elt_t;
use bindings::ngx_uint_t;
use bindings::NGX_LOG_ERR;
use std::slice;
use std::str;

impl ngx_str_t {
    pub const fn new() -> Self {
        ngx_str_t {
            len: 0,
            data: std::ptr::null_mut(),
        }
    }

    pub const fn from_static(data: &'static str) -> Self {
        ngx_str_t {
            len: data.len() - 1,
            data: data.as_ptr() as *mut u8,
        }
    }

    // convert nginx string to str slice
    pub fn to_str(&self) -> &str {
        unsafe {
            let slice = slice::from_raw_parts(self.data, self.len);
            str::from_utf8(slice).unwrap()
        }
    }

    // get string
    pub fn to_string(&self) -> String {
        String::from(self.to_str())
    }
}

impl ngx_http_request_s {
    /*
    pub fn scheme(&self) -> char {
        unsafe {  (*self.schema_start)};
    }
    */
}

impl ngx_http_headers_in_t {
    // host
    pub fn host_str(&self) -> &str {
        unsafe { (*self.host).value.to_str() }
    }

    pub fn user_agent_str(&self) -> &str {
        unsafe { (*self.user_agent).value.to_str() }
    }

    // referrer
    pub fn referer_str(&self) -> Option<&str> {
        let referer = self.referer;

        if referer.is_null() {
            return None;
        }

        Some(unsafe { (*referer).value.to_str() })
    }

    pub fn headers_iterator(&self) -> NgxListIterator {
        list_iterator(&self.headers)
    }
}

impl ngx_http_headers_out_t {
    pub fn content_length_str(&self) -> &str {
        unsafe { (*self.content_length).value.to_str() }
    }

    pub fn server_str(&self) -> &str {
        unsafe { (*self.server).value.to_str() }
    }

    pub fn headers_iterator(&self) -> NgxListIterator {
        list_iterator(&self.headers)
    }
}

pub struct NgxListIterator {
    done: bool,
    part: *const ngx_list_part_t,
    h: *const ngx_table_elt_t,
    i: ngx_uint_t,
}

// create new http request iterator
pub fn list_iterator(list: *const ngx_list_t) -> NgxListIterator {
    unsafe {
        let part: *const ngx_list_part_t = &(*list).part;

        NgxListIterator {
            done: false,
            part,
            h: (*part).elts as *const ngx_table_elt_t,
            i: 0,
        }
    }
}

// iterator for ngx_list_t

impl Iterator for NgxListIterator {
    // type Item = (&str,&str);
    // TODO: try to use str instead of string

    type Item = (String, String);

    fn next(&mut self) -> Option<Self::Item> {
        unsafe {
            if self.done {
                None
            } else {
                if self.i >= (*self.part).nelts {
                    if (*self.part).next.is_null() {
                        self.done = true;
                        return None;
                    }

                    // loop back
                    self.part = (*self.part).next;
                    self.h = (*self.part).elts as *mut ngx_table_elt_t;
                    self.i = 0;
                }

                let header: *const ngx_table_elt_t = self.h.add(self.i);

                let header_name: ngx_str_t = (*header).key;

                let header_value: ngx_str_t = (*header).value;

                self.i += 1;

                Some((header_name.to_string(), header_value.to_string()))
            }
        }
    }
}

impl ngx_http_module_t {
    pub const fn new() -> Self {
        ngx_http_module_t {
            preconfiguration: None,
            postconfiguration: None,
            create_main_conf: None,
            init_main_conf: None,
            create_srv_conf: None,
            merge_srv_conf: None,
            create_loc_conf: None,
            merge_loc_conf: None,
        }
    }
}

impl ngx_module_s {
    pub const fn new_v1() -> Self {
        ngx_module_s {
            ctx_index: std::usize::MAX,
            index: std::usize::MAX,
            name: std::ptr::null_mut(),
            spare0: 0,
            spare1: 0,
            version: ::bindings::nginx_version as usize,
            signature: std::ptr::null_mut(), // TODO
            ctx: std::ptr::null_mut(),
            commands: std::ptr::null_mut(),
            type_: ::bindings::NGX_HTTP_MODULE as usize,
            init_master: None,
            init_module: None,
            init_process: None,
            init_thread: None,
            exit_thread: None,
            exit_process: None,
            exit_master: None,
            spare_hook0: 0,
            spare_hook1: 0,
            spare_hook2: 0,
            spare_hook3: 0,
            spare_hook4: 0,
            spare_hook5: 0,
            spare_hook6: 0,
            spare_hook7: 0,
        }
    }
}

unsafe impl Sync for ngx_module_s {}
