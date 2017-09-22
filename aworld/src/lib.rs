use std::os::raw::c_int;
extern crate aworld_sys;

fn rc(rc: c_int) -> Result<(), c_int> {
    if rc == 0 {
        Ok(())
    } else {
        Err(rc)
    }
}

pub fn init() -> Result<(), c_int> {
    rc(unsafe {
        aworld_sys::aw_init(101)
    })
}

#[test]
fn testinit() {
    init().expect("Unable to init");
}