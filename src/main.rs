use x11::xlib;
use std::ffi::CString;
use std::os::raw::c_uint;
use std::os::raw::c_ulong;
use std::mem;
use libc;

const XKB_ALL_NAMES_MASK: c_uint = 0x3fff;
const XKB_ALL_CTRLS_MASK: c_ulong = 0xF8001FFF;

fn main() {
    unsafe {
        let keyboard = initialize_xkb();

        use_next_layout(keyboard);

        xlib::XCloseDisplay((*keyboard).dpy);
    };
}

unsafe fn initialize_xkb() -> xlib::XkbDescPtr {
    let display_name: CString = CString::new("").expect("Cstring fucked up");
    let display = xlib::XOpenDisplay(display_name.as_ptr());

    let keyboard: xlib::XkbDescPtr = xlib::XkbAllocKeyboard();
    (*keyboard).dpy = display;
    xlib::XkbGetNames(display, XKB_ALL_NAMES_MASK, keyboard);
    xlib::XkbGetControls(display, XKB_ALL_CTRLS_MASK, keyboard);

    return keyboard;
}

unsafe fn get_state(keyboard: xlib::XkbDescPtr) -> xlib::XkbStatePtr
{
    let size = mem::size_of::<xlib::XkbStateRec>();
    let state: xlib::XkbStatePtr = libc::malloc(size) as xlib::XkbStatePtr;

    xlib::XkbGetState((*keyboard).dpy, (*keyboard).device_spec.into(), state);
    return state;
}

unsafe fn use_next_layout(keyboard: xlib::XkbDescPtr) {
    let num_groups = count_groups(keyboard);
    let state = get_state(keyboard);
    let current_group = (*state).group as u32;

    let mut next: u32 = current_group + 1;
    if next >= num_groups as u32 {
        next = 0;
    }
    xlib::XkbLockGroup((*keyboard).dpy, (*keyboard).device_spec.into(), next as u32);
}

unsafe fn count_groups(keyboard: xlib::XkbDescPtr) -> u8 {
    if (*keyboard).ctrls.is_null() {
        let mut num_groups: u8 = 0;
        let group_source = (*(*keyboard).names).groups;

        for group in group_source.iter() {
            if *group == 0 {
                break;
            }
            num_groups += 1;
        }

        return num_groups;
    } else {
        return (*(*keyboard).ctrls).num_groups;
    }
}