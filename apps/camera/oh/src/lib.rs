//! The Mate 70 Air camera replica as an OpenHarmony native-widget app:
//! the same `octosense-camera-logic` session and scenes as the Makepad
//! host, mounted as ArkUI nodes through Octoscript-OH's binding, the camera
//! streaming zero-copy into an XComponent surface. ArkTS keeps only the
//! ability shell, the permission prompts and the gallery save.
macro_rules! info { ($($t:tt)*) => { $crate::log::info(&format!($($t)*)) }; }
macro_rules! error { ($($t:tt)*) => { $crate::log::error(&format!($($t)*)) }; }
mod log;
mod camera;
mod host;
mod motion;
mod mount;
mod xcomp;

use napi_derive_ohos::napi;
use napi_ohos::{Env, JsObject, NapiRaw};
use octoscript_oh_arkui::arkui::{self, NodeContentHandle};

extern "C" {
    fn OH_ArkUI_GetNodeContentFromNapiValue(env: napi_ohos::sys::napi_env, value: napi_ohos::sys::napi_value, handle: *mut NodeContentHandle) -> i32;
}

/// Called once from ArkTS with the page's `NodeContent` and the app's files directory.
#[napi(js_name = "mount")]
pub fn mount(env: Env, content: JsObject, files_dir: String) -> napi_ohos::Result<()> {
    if let Err(e) = arkui::init() { log::error(&format!("camera-oh: {e}")); return Ok(()); }
    let mut slot: NodeContentHandle = std::ptr::null_mut();
    let status = unsafe { OH_ArkUI_GetNodeContentFromNapiValue(env.raw(), content.raw(), &mut slot) };
    if status != 0 || slot.is_null() { log::error("camera-oh: no NodeContent"); return Ok(()); }
    if host::Host::create(slot, files_dir).is_none() { log::error("camera-oh: host did not start"); }
    Ok(())
}

/// The frame clock.
#[napi(js_name = "tick")]
pub fn tick() -> napi_ohos::Result<()> {
    host::HOST.with(|h| { if let Some(host) = h.borrow_mut().as_mut() { host.tick(); } });
    Ok(())
}

/// Height of the system status bar (vp): the artboard is laid out below it.
#[napi(js_name = "setTopInset")]
pub fn set_top_inset(top: f64) -> napi_ohos::Result<()> {
    host::TOP_INSET.set(top as f32);
    host::HOST.with(|h| { if let Some(host) = h.borrow_mut().as_mut() { host.set_top_inset(top as f32); } });
    Ok(())
}

/// The next thing ArkTS must do ("permission|camera", "save|<path>"), or "".
#[napi(js_name = "nextRequest")]
pub fn next_request() -> napi_ohos::Result<String> {
    Ok(host::HOST.with(|h| h.borrow_mut().as_mut().map(|host| host.next_request()).unwrap_or_default()))
}

#[napi(js_name = "handlePermissionResult")]
pub fn handle_permission_result(name: String, granted: bool) -> napi_ohos::Result<()> {
    host::HOST.with(|h| { if let Some(host) = h.borrow_mut().as_mut() { host.permission_result(&name, granted); } });
    Ok(())
}

#[napi(js_name = "handleCaptureSaved")]
pub fn handle_capture_saved(path: String, ok: bool) -> napi_ohos::Result<()> {
    host::HOST.with(|h| { if let Some(host) = h.borrow_mut().as_mut() { host.capture_saved(&path, ok); } });
    Ok(())
}

/// The system Back gesture: true when the app consumed it.
#[napi(js_name = "back")]
pub fn back() -> napi_ohos::Result<bool> {
    Ok(host::HOST.with(|h| h.borrow_mut().as_mut().map(|host| host.back()).unwrap_or(false)))
}
