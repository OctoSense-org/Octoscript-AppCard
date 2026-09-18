//! An `ARKUI_NODE_XCOMPONENT` surface the camera streams into (zero copy),
//! after Octoscript-OH's `xcomp.rs`.
use octoscript_oh_arkui::arkui::Node;
use std::ffi::c_void;
use std::os::raw::c_char;
use std::sync::atomic::{AtomicU64, Ordering};

pub const ARKUI_NODE_XCOMPONENT: i32 = 12;
const NODE_XCOMPONENT_TYPE: i32 = 1000 * ARKUI_NODE_XCOMPONENT + 1;
const XCOMPONENT_TYPE_SURFACE: i32 = 0;

#[repr(C)]
struct XComponentCallback {
    on_surface_created: extern "C" fn(*mut c_void, *mut c_void),
    on_surface_changed: extern "C" fn(*mut c_void, *mut c_void),
    on_surface_destroyed: extern "C" fn(*mut c_void, *mut c_void),
    dispatch_touch_event: extern "C" fn(*mut c_void, *mut c_void),
}
extern "C" {
    fn OH_NativeXComponent_GetNativeXComponent(node: *mut c_void) -> *mut c_void;
    fn OH_NativeXComponent_RegisterCallback(component: *mut c_void, callback: *mut XComponentCallback) -> i32;
    fn dlopen(file: *const c_char, mode: i32) -> *mut c_void;
    fn dlsym(handle: *mut c_void, name: *const c_char) -> *mut c_void;
}

/// The live surface id (0 until the surface exists).
pub static SURFACE_ID: AtomicU64 = AtomicU64::new(0);

extern "C" fn on_created(_component: *mut c_void, window: *mut c_void) {
    let mut id = 0u64;
    unsafe {
        let lib = dlopen(c"libnative_window.so".as_ptr(), 2);
        if !lib.is_null() {
            let p = dlsym(lib, c"OH_NativeWindow_GetSurfaceId".as_ptr());
            if !p.is_null() {
                let get: unsafe extern "C" fn(*mut c_void, *mut u64) -> i32 = std::mem::transmute(p);
                get(window, &mut id);
            }
        }
    }
    SURFACE_ID.store(id, Ordering::SeqCst);
    info!("xcomp: surface created, id {id}");
}
extern "C" fn on_changed(_c: *mut c_void, _w: *mut c_void) {}
extern "C" fn on_destroyed(_c: *mut c_void, _w: *mut c_void) {
    SURFACE_ID.store(0, Ordering::SeqCst);
    info!("xcomp: surface destroyed");
}
extern "C" fn on_touch(_c: *mut c_void, _w: *mut c_void) {}
static mut CALLBACKS: XComponentCallback = XComponentCallback {
    on_surface_created: on_created,
    on_surface_changed: on_changed,
    on_surface_destroyed: on_destroyed,
    dispatch_touch_event: on_touch,
};

pub fn surface(w: f32, h: f32) -> Option<Node> {
    let node = Node::new(ARKUI_NODE_XCOMPONENT)?.i32_attr(NODE_XCOMPONENT_TYPE, XCOMPONENT_TYPE_SURFACE).width(w).height(h);
    let component = unsafe { OH_NativeXComponent_GetNativeXComponent(node.raw()) };
    if component.is_null() {
        error!("xcomp: no OH_NativeXComponent behind the node");
        return Some(node);
    }
    let rc = unsafe { OH_NativeXComponent_RegisterCallback(component, &raw mut CALLBACKS) };
    info!("xcomp: surface callbacks registered ({rc})");
    Some(node)
}
