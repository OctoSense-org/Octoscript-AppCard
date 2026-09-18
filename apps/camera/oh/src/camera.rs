//! The OpenHarmony camera NDK for the native-widget host: preview into an
//! XComponent surface (zero copy), stills through a JPEG image receiver,
//! recordings through the AV recorder, and the session controls. A port of
//! the Makepad backend's `oh_camera.rs` without its CPU frame path; every
//! symbol is dlopen'ed.
use std::collections::VecDeque;
use std::ffi::{c_char, c_void, CStr, CString};
use std::os::fd::AsRawFd;
use std::sync::atomic::{AtomicU64, Ordering};
use std::sync::{Arc, Mutex};

// ---------------------------------------------------------------------------
// ABI
// ---------------------------------------------------------------------------
#[repr(C)] #[derive(Clone, Copy)] struct CameraSize { width: u32, height: u32 }
#[repr(C)] #[derive(Clone, Copy)] struct CameraProfile { format: i32, size: CameraSize }
#[repr(C)] #[derive(Clone, Copy)] struct CameraFrameRateRange { min: u32, max: u32 }
#[repr(C)] #[derive(Clone, Copy)] struct CameraVideoProfile { format: i32, size: CameraSize, range: CameraFrameRateRange }
#[repr(C)] struct CameraDevice { camera_id: *mut c_char, position: i32, camera_type: i32, connection: i32 }
#[repr(C)] struct OutputCapability {
    preview_profiles: *mut *mut CameraProfile, preview_profiles_size: u32,
    photo_profiles: *mut *mut CameraProfile, photo_profiles_size: u32,
    video_profiles: *mut *mut CameraVideoProfile, video_profiles_size: u32,
    supported_metadata_object_types: *mut *mut c_void, metadata_types_size: u32,
}
#[repr(C)] #[derive(Clone, Copy)] struct ImageSize { width: u32, height: u32 }
#[repr(C)] #[derive(Clone, Copy)] struct CameraPoint { x: f64, y: f64 }
#[repr(C)] #[derive(Clone, Copy)] struct CameraPhotoCaptureSetting { quality: i32, rotation: i32, location: *mut c_void, mirror: bool }
#[repr(C)] #[derive(Clone, Copy)] struct AvProfile { audio_bitrate: i32, audio_channels: i32, audio_codec: i32, audio_sample_rate: i32, file_format: i32, video_bitrate: i32, video_codec: i32, video_frame_width: i32, video_frame_height: i32, video_frame_rate: i32, is_hdr: bool, enable_temporal_scale: bool }
#[repr(C)] #[derive(Clone, Copy)] struct AvLocation { latitude: f32, longitude: f32 }
#[repr(C)] #[derive(Clone, Copy)] struct AvMetadataTemplate { key: *mut c_char, value: *mut c_char }
#[repr(C)] #[derive(Clone, Copy)] struct AvMetadata { genre: *mut c_char, video_orientation: *mut c_char, location: AvLocation, custom_info: AvMetadataTemplate }
#[repr(C)] #[derive(Clone, Copy)] struct AvConfig { audio_source_type: i32, video_source_type: i32, profile: AvProfile, url: *mut c_char, file_generation_mode: i32, metadata: AvMetadata, max_duration: i32 }

const CAMERA_FORMAT_YUV_420_SP: i32 = 1003;
const CAMERA_FORMAT_JPEG: i32 = 2000;
const CAMERA_POSITION_FRONT: i32 = 2;
type Rc32 = i32;
type ReceiverCb = unsafe extern "C" fn(*mut c_void);
type RecorderStateCb = unsafe extern "C" fn(*mut c_void, i32, i32, *mut c_void);
type RecorderErrorCb = unsafe extern "C" fn(*mut c_void, i32, *const c_char, *mut c_void);

extern "C" {
    fn dlopen(file: *const c_char, mode: i32) -> *mut c_void;
    fn dlsym(handle: *mut c_void, name: *const c_char) -> *mut c_void;
}
struct Lib(*mut c_void);
impl Lib {
    fn open(name: &str) -> Option<Lib> {
        let c = CString::new(name).ok()?;
        let h = unsafe { dlopen(c.as_ptr(), 2) };
        if h.is_null() { None } else { Some(Lib(h)) }
    }
    fn sym<T: Copy>(&self, name: &str) -> Option<T> {
        let c = CString::new(name).ok()?;
        let p = unsafe { dlsym(self.0, c.as_ptr()) };
        if p.is_null() { None } else { Some(unsafe { std::mem::transmute_copy::<*mut c_void, T>(&p) }) }
    }
}
unsafe impl Send for Lib {}
unsafe impl Sync for Lib {}

macro_rules! api_struct {
    ($($name:ident : $ty:ty = $lib:literal . $sym:literal),* $(,)?) => {
        struct Api { _libs: Vec<Lib>, $($name: $ty,)* }
        fn load() -> Option<Api> {
            let mut libs: std::collections::HashMap<&'static str, Lib> = std::collections::HashMap::new();
            for name in ["libohcamera.so", "libimage_receiver.so", "libohimage.so", "libnative_buffer.so", "libavrecorder.so", "libnative_window.so"] {
                match Lib::open(name) { Some(l) => { libs.insert(name, l); } None => crate::log::info(&format!("camera-oh: {name} did not load")) }
            }
            if !libs.contains_key("libohcamera.so") { return None; }
            let api = Api {
                $($name: libs.get($lib).and_then(|l| l.sym($sym)).or_else(|| { crate::log::info(&format!("camera-oh: NDK symbol {} missing", $sym)); None }),)*
                _libs: libs.into_values().collect(),
            };
            Some(api)
        }
    };
}
api_struct! {
    get_manager: Option<unsafe extern "C" fn(*mut *mut c_void) -> Rc32> = "libohcamera.so"."OH_Camera_GetCameraManager",
    get_supported: Option<unsafe extern "C" fn(*mut c_void, *mut *mut CameraDevice, *mut u32) -> Rc32> = "libohcamera.so"."OH_CameraManager_GetSupportedCameras",
    get_capability: Option<unsafe extern "C" fn(*mut c_void, *const CameraDevice, *mut *mut OutputCapability) -> Rc32> = "libohcamera.so"."OH_CameraManager_GetSupportedCameraOutputCapability",
    delete_capability: Option<unsafe extern "C" fn(*mut c_void, *mut OutputCapability) -> Rc32> = "libohcamera.so"."OH_CameraManager_DeleteSupportedCameraOutputCapability",
    create_input: Option<unsafe extern "C" fn(*mut c_void, *const CameraDevice, *mut *mut c_void) -> Rc32> = "libohcamera.so"."OH_CameraManager_CreateCameraInput",
    input_open: Option<unsafe extern "C" fn(*mut c_void) -> Rc32> = "libohcamera.so"."OH_CameraInput_Open",
    input_close: Option<unsafe extern "C" fn(*mut c_void) -> Rc32> = "libohcamera.so"."OH_CameraInput_Close",
    input_release: Option<unsafe extern "C" fn(*mut c_void) -> Rc32> = "libohcamera.so"."OH_CameraInput_Release",
    create_session: Option<unsafe extern "C" fn(*mut c_void, *mut *mut c_void) -> Rc32> = "libohcamera.so"."OH_CameraManager_CreateCaptureSession",
    begin_config: Option<unsafe extern "C" fn(*mut c_void) -> Rc32> = "libohcamera.so"."OH_CaptureSession_BeginConfig",
    add_input: Option<unsafe extern "C" fn(*mut c_void, *mut c_void) -> Rc32> = "libohcamera.so"."OH_CaptureSession_AddInput",
    create_preview: Option<unsafe extern "C" fn(*mut c_void, *const CameraProfile, *const c_char, *mut *mut c_void) -> Rc32> = "libohcamera.so"."OH_CameraManager_CreatePreviewOutput",
    add_preview: Option<unsafe extern "C" fn(*mut c_void, *mut c_void) -> Rc32> = "libohcamera.so"."OH_CaptureSession_AddPreviewOutput",
    preview_release: Option<unsafe extern "C" fn(*mut c_void) -> Rc32> = "libohcamera.so"."OH_PreviewOutput_Release",
    commit_config: Option<unsafe extern "C" fn(*mut c_void) -> Rc32> = "libohcamera.so"."OH_CaptureSession_CommitConfig",
    session_start: Option<unsafe extern "C" fn(*mut c_void) -> Rc32> = "libohcamera.so"."OH_CaptureSession_Start",
    session_stop: Option<unsafe extern "C" fn(*mut c_void) -> Rc32> = "libohcamera.so"."OH_CaptureSession_Stop",
    session_release: Option<unsafe extern "C" fn(*mut c_void) -> Rc32> = "libohcamera.so"."OH_CaptureSession_Release",
    set_focus_mode: Option<unsafe extern "C" fn(*mut c_void, i32) -> Rc32> = "libohcamera.so"."OH_CaptureSession_SetFocusMode",
    set_focus_point: Option<unsafe extern "C" fn(*mut c_void, CameraPoint) -> Rc32> = "libohcamera.so"."OH_CaptureSession_SetFocusPoint",
    set_metering_point: Option<unsafe extern "C" fn(*mut c_void, CameraPoint) -> Rc32> = "libohcamera.so"."OH_CaptureSession_SetMeteringPoint",
    set_exposure_mode: Option<unsafe extern "C" fn(*mut c_void, i32) -> Rc32> = "libohcamera.so"."OH_CaptureSession_SetExposureMode",
    get_zoom_range: Option<unsafe extern "C" fn(*mut c_void, *mut f32, *mut f32) -> Rc32> = "libohcamera.so"."OH_CaptureSession_GetZoomRatioRange",
    set_zoom: Option<unsafe extern "C" fn(*mut c_void, f32) -> Rc32> = "libohcamera.so"."OH_CaptureSession_SetZoomRatio",
    get_ev_range: Option<unsafe extern "C" fn(*mut c_void, *mut f32, *mut f32, *mut f32) -> Rc32> = "libohcamera.so"."OH_CaptureSession_GetExposureBiasRange",
    set_ev: Option<unsafe extern "C" fn(*mut c_void, f32) -> Rc32> = "libohcamera.so"."OH_CaptureSession_SetExposureBias",
    set_flash_mode: Option<unsafe extern "C" fn(*mut c_void, i32) -> Rc32> = "libohcamera.so"."OH_CaptureSession_SetFlashMode",
    create_photo_output: Option<unsafe extern "C" fn(*mut c_void, *const CameraProfile, *const c_char, *mut *mut c_void) -> Rc32> = "libohcamera.so"."OH_CameraManager_CreatePhotoOutput",
    add_photo_output: Option<unsafe extern "C" fn(*mut c_void, *mut c_void) -> Rc32> = "libohcamera.so"."OH_CaptureSession_AddPhotoOutput",
    photo_capture: Option<unsafe extern "C" fn(*mut c_void, CameraPhotoCaptureSetting) -> Rc32> = "libohcamera.so"."OH_PhotoOutput_Capture_WithCaptureSetting",
    photo_rotation: Option<unsafe extern "C" fn(*mut c_void, i32, *mut i32) -> Rc32> = "libohcamera.so"."OH_PhotoOutput_GetPhotoRotation",
    photo_release: Option<unsafe extern "C" fn(*mut c_void) -> Rc32> = "libohcamera.so"."OH_PhotoOutput_Release",
    create_video_output: Option<unsafe extern "C" fn(*mut c_void, *const CameraVideoProfile, *const c_char, *mut *mut c_void) -> Rc32> = "libohcamera.so"."OH_CameraManager_CreateVideoOutput",
    add_video_output: Option<unsafe extern "C" fn(*mut c_void, *mut c_void) -> Rc32> = "libohcamera.so"."OH_CaptureSession_AddVideoOutput",
    remove_video_output: Option<unsafe extern "C" fn(*mut c_void, *mut c_void) -> Rc32> = "libohcamera.so"."OH_CaptureSession_RemoveVideoOutput",
    video_output_start: Option<unsafe extern "C" fn(*mut c_void) -> Rc32> = "libohcamera.so"."OH_VideoOutput_Start",
    video_output_stop: Option<unsafe extern "C" fn(*mut c_void) -> Rc32> = "libohcamera.so"."OH_VideoOutput_Stop",
    video_output_release: Option<unsafe extern "C" fn(*mut c_void) -> Rc32> = "libohcamera.so"."OH_VideoOutput_Release",
    opts_create: Option<unsafe extern "C" fn(*mut *mut c_void) -> Rc32> = "libimage_receiver.so"."OH_ImageReceiverOptions_Create",
    opts_set_size: Option<unsafe extern "C" fn(*mut c_void, ImageSize) -> Rc32> = "libimage_receiver.so"."OH_ImageReceiverOptions_SetSize",
    opts_set_capacity: Option<unsafe extern "C" fn(*mut c_void, i32) -> Rc32> = "libimage_receiver.so"."OH_ImageReceiverOptions_SetCapacity",
    opts_release: Option<unsafe extern "C" fn(*mut c_void) -> Rc32> = "libimage_receiver.so"."OH_ImageReceiverOptions_Release",
    receiver_create: Option<unsafe extern "C" fn(*mut c_void, *mut *mut c_void) -> Rc32> = "libimage_receiver.so"."OH_ImageReceiverNative_Create",
    receiver_surface_id: Option<unsafe extern "C" fn(*mut c_void, *mut u64) -> Rc32> = "libimage_receiver.so"."OH_ImageReceiverNative_GetReceivingSurfaceId",
    receiver_read_latest: Option<unsafe extern "C" fn(*mut c_void, *mut *mut c_void) -> Rc32> = "libimage_receiver.so"."OH_ImageReceiverNative_ReadLatestImage",
    receiver_on: Option<unsafe extern "C" fn(*mut c_void, ReceiverCb) -> Rc32> = "libimage_receiver.so"."OH_ImageReceiverNative_On",
    receiver_off: Option<unsafe extern "C" fn(*mut c_void) -> Rc32> = "libimage_receiver.so"."OH_ImageReceiverNative_Off",
    receiver_release: Option<unsafe extern "C" fn(*mut c_void) -> Rc32> = "libimage_receiver.so"."OH_ImageReceiverNative_Release",
    image_size: Option<unsafe extern "C" fn(*mut c_void, *mut ImageSize) -> Rc32> = "libohimage.so"."OH_ImageNative_GetImageSize",
    image_component_types: Option<unsafe extern "C" fn(*mut c_void, *mut *mut u32, *mut usize) -> Rc32> = "libohimage.so"."OH_ImageNative_GetComponentTypes",
    image_byte_buffer: Option<unsafe extern "C" fn(*mut c_void, u32, *mut *mut c_void) -> Rc32> = "libohimage.so"."OH_ImageNative_GetByteBuffer",
    image_buffer_size: Option<unsafe extern "C" fn(*mut c_void, u32, *mut usize) -> Rc32> = "libohimage.so"."OH_ImageNative_GetBufferSize",
    image_release: Option<unsafe extern "C" fn(*mut c_void) -> Rc32> = "libohimage.so"."OH_ImageNative_Release",
    buffer_map: Option<unsafe extern "C" fn(*mut c_void, *mut *mut c_void) -> Rc32> = "libnative_buffer.so"."OH_NativeBuffer_Map",
    buffer_unmap: Option<unsafe extern "C" fn(*mut c_void) -> Rc32> = "libnative_buffer.so"."OH_NativeBuffer_Unmap",
    recorder_create: Option<unsafe extern "C" fn() -> *mut c_void> = "libavrecorder.so"."OH_AVRecorder_Create",
    recorder_prepare: Option<unsafe extern "C" fn(*mut c_void, *mut AvConfig) -> Rc32> = "libavrecorder.so"."OH_AVRecorder_Prepare",
    recorder_surface: Option<unsafe extern "C" fn(*mut c_void, *mut *mut c_void) -> Rc32> = "libavrecorder.so"."OH_AVRecorder_GetInputSurface",
    recorder_rotation: Option<unsafe extern "C" fn(*mut c_void, i32) -> Rc32> = "libavrecorder.so"."OH_AVRecorder_UpdateRotation",
    recorder_start: Option<unsafe extern "C" fn(*mut c_void) -> Rc32> = "libavrecorder.so"."OH_AVRecorder_Start",
    recorder_pause: Option<unsafe extern "C" fn(*mut c_void) -> Rc32> = "libavrecorder.so"."OH_AVRecorder_Pause",
    recorder_resume: Option<unsafe extern "C" fn(*mut c_void) -> Rc32> = "libavrecorder.so"."OH_AVRecorder_Resume",
    recorder_stop: Option<unsafe extern "C" fn(*mut c_void) -> Rc32> = "libavrecorder.so"."OH_AVRecorder_Stop",
    recorder_release: Option<unsafe extern "C" fn(*mut c_void) -> Rc32> = "libavrecorder.so"."OH_AVRecorder_Release",
    recorder_set_state_cb: Option<unsafe extern "C" fn(*mut c_void, RecorderStateCb, *mut c_void) -> Rc32> = "libavrecorder.so"."OH_AVRecorder_SetStateCallback",
    recorder_set_error_cb: Option<unsafe extern "C" fn(*mut c_void, RecorderErrorCb, *mut c_void) -> Rc32> = "libavrecorder.so"."OH_AVRecorder_SetErrorCallback",
    window_surface_id: Option<unsafe extern "C" fn(*mut c_void, *mut u64) -> Rc32> = "libnative_window.so"."OH_NativeWindow_GetSurfaceId",
}
static API: std::sync::OnceLock<Option<Api>> = std::sync::OnceLock::new();
fn api() -> Option<&'static Api> { API.get_or_init(load).as_ref() }

fn cam_error(rc: i32) -> String {
    match rc {
        7_400_101 => "invalid argument".into(), 7_400_102 => "operation not allowed in this session state".into(), 7_400_103 => "session not configured".into(),
        7_400_201 => "camera unavailable (in use by another app, or ohos.permission.CAMERA not granted)".into(), 7_400_202 => "camera disabled by device policy".into(),
        7_400_203 => "camera already in use here".into(), 201 => "permission denied (ohos.permission.CAMERA)".into(), _ => format!("camera error {rc}"),
    }
}
unsafe extern "C" fn noop_release(_: *mut c_void) -> Rc32 { 0 }

// ---------------------------------------------------------------------------
// Public types
// ---------------------------------------------------------------------------
#[derive(Clone, Copy, Debug)]
pub enum Control { FocusPoint { x: f64, y: f64 }, ContinuousFocus, Zoom(f32), ExposureBias(f32), Flash(i32) }
#[derive(Clone, Debug)]
pub enum Capture { Photo { path: String }, StartVideo { path: String, audio: bool }, PauseVideo, ResumeVideo, StopVideo }
#[derive(Clone, Debug)]
pub enum CaptureResult { Photo { path: String, width: u32, height: u32 }, VideoStarted { path: String }, VideoPaused, VideoResumed, VideoStopped { path: String }, Failed { what: String, error: String } }

static RESULTS: Mutex<Vec<CaptureResult>> = Mutex::new(Vec::new());
pub fn take_results() -> Vec<CaptureResult> { RESULTS.lock().map(|mut r| std::mem::take(&mut *r)).unwrap_or_default() }
fn push_result(r: CaptureResult) { if let Ok(mut q) = RESULTS.lock() { q.push(r); } }

struct Device { index: isize, front: bool, name: String, previews: Vec<CameraProfile>, photos: Vec<CameraProfile>, videos: Vec<CameraVideoProfile> }

/// State shared with the photo receiver's thread.
struct PhotoShared { receiver: AtomicU64, pending: Mutex<VecDeque<String>> }
static ACTIVE_PHOTO: Mutex<Option<Arc<PhotoShared>>> = Mutex::new(None);

struct VideoRec { recorder: *mut c_void, output: *mut c_void, path: String, _file: std::fs::File, _strings: Vec<CString> }
struct Session { input: *mut c_void, session: *mut c_void, preview: *mut c_void, photo_output: *mut c_void, photo_receiver: *mut c_void, photo: Option<Arc<PhotoShared>>, front: bool, size: CameraSize, video: Option<VideoRec> }

pub struct Camera { manager: *mut c_void, list: *mut CameraDevice, devices: Vec<Device>, session: Option<Session> }

impl Camera {
    pub fn new() -> Camera {
        let mut cam = Camera { manager: std::ptr::null_mut(), list: std::ptr::null_mut(), devices: Vec::new(), session: None };
        let Some(api) = api() else { error!("camera-oh: camera NDK unavailable"); return cam };
        let (Some(get_manager), Some(get_supported), Some(get_capability), Some(delete_capability)) = (api.get_manager, api.get_supported, api.get_capability, api.delete_capability) else { return cam };
        unsafe {
            if get_manager(&mut cam.manager) != 0 || cam.manager.is_null() { error!("camera-oh: no camera manager"); return cam; }
            let mut count = 0u32;
            if get_supported(cam.manager, &mut cam.list, &mut count) != 0 || cam.list.is_null() { error!("camera-oh: enumerate failed"); return cam; }
            for i in 0..count as isize {
                let d = &*cam.list.offset(i);
                let id = if d.camera_id.is_null() { format!("camera{i}") } else { CStr::from_ptr(d.camera_id).to_string_lossy().into_owned() };
                let front = d.position == CAMERA_POSITION_FRONT;
                let mut cap: *mut OutputCapability = std::ptr::null_mut();
                let (mut previews, mut photos, mut videos) = (Vec::new(), Vec::new(), Vec::new());
                if get_capability(cam.manager, d, &mut cap) == 0 && !cap.is_null() {
                    let c = &*cap;
                    for p in 0..c.preview_profiles_size as isize { let pr = *c.preview_profiles.offset(p); if !pr.is_null() && (*pr).format == CAMERA_FORMAT_YUV_420_SP { previews.push(*pr); } }
                    for p in 0..c.photo_profiles_size as isize { let pr = *c.photo_profiles.offset(p); if !pr.is_null() && (*pr).format == CAMERA_FORMAT_JPEG { photos.push(*pr); } }
                    for p in 0..c.video_profiles_size as isize { let pr = *c.video_profiles.offset(p); if !pr.is_null() && (*pr).format == CAMERA_FORMAT_YUV_420_SP { videos.push(*pr); } }
                    delete_capability(cam.manager, cap);
                }
                info!("camera-oh: {} camera ({id}): {} preview, {} photo, {} video profiles", if front { "front" } else { "back" }, previews.len(), photos.len(), videos.len());
                cam.devices.push(Device { index: i, front, name: id, previews, photos, videos });
            }
        }
        cam
    }

    pub fn has_camera(&self, front: bool) -> bool { self.devices.iter().any(|d| d.front == front) }

    /// Open the camera into `surface_id` with the preview profile nearest `aspect` (≤1080p). Returns the preview size.
    pub fn start_preview(&mut self, front: bool, aspect: f64, surface_id: u64) -> Result<(u32, u32), String> {
        let api = api().ok_or("camera NDK unavailable")?;
        self.stop_preview();
        let device = self.devices.iter().find(|d| d.front == front).or(self.devices.first()).ok_or("no camera")?;
        let usable = |p: &&CameraProfile| p.size.width <= 1920 && p.size.height <= 1080 && p.size.height > 0;
        let profile = *device.previews.iter().filter(usable).min_by(|a, b| {
            let key = |p: &CameraProfile| (((p.size.width as f64 / p.size.height as f64 - aspect).abs() * 50.0).round() as i64, -((p.size.width * p.size.height) as i64));
            key(a).cmp(&key(b))
        }).ok_or("no preview profile")?;
        let photo_profile = device.photos.iter().filter(|p| p.size.height > 0 && ((p.size.width as f64 / p.size.height as f64) - aspect).abs() < 0.03).max_by_key(|p| p.size.width as u64 * p.size.height as u64).copied();
        let device_ptr = unsafe { self.list.offset(device.index) };
        let (front, name) = (device.front, device.name.clone());
        let mgr = self.manager;
        let (Some(create_input), Some(input_open), Some(input_close), Some(input_release), Some(create_preview), Some(create_session), Some(begin), Some(add_input), Some(add_preview), Some(commit), Some(start), Some(session_release), Some(preview_release)) =
            (api.create_input, api.input_open, api.input_close, api.input_release, api.create_preview, api.create_session, api.begin_config, api.add_input, api.add_preview, api.commit_config, api.session_start, api.session_release, api.preview_release) else { return Err("camera NDK incomplete".into()) };
        unsafe {
            // still output: a JPEG receiver of the photo size
            let (mut photo_receiver, mut photo_surface) = (std::ptr::null_mut(), 0u64);
            if let (Some(pp), Some(oc), Some(ss), Some(sc), Some(orl), Some(rc_create), Some(rsid)) = (photo_profile, api.opts_create, api.opts_set_size, api.opts_set_capacity, api.opts_release, api.receiver_create, api.receiver_surface_id) {
                let mut opts = std::ptr::null_mut();
                if oc(&mut opts) == 0 && !opts.is_null() {
                    ss(opts, ImageSize { width: pp.size.width, height: pp.size.height }); sc(opts, 2);
                    let rc = rc_create(opts, &mut photo_receiver); orl(opts);
                    if rc != 0 || photo_receiver.is_null() || rsid(photo_receiver, &mut photo_surface) != 0 { photo_receiver = std::ptr::null_mut(); }
                }
            }
            let mut input = std::ptr::null_mut();
            let rc = create_input(mgr, device_ptr, &mut input);
            if rc != 0 || input.is_null() { return Err(format!("camera input: {}", cam_error(rc))); }
            let rc = input_open(input);
            if rc != 0 { input_release(input); return Err(format!("open camera: {}", cam_error(rc))); }
            let sid = CString::new(surface_id.to_string()).unwrap();
            let mut preview = std::ptr::null_mut();
            let rc = create_preview(mgr, &profile, sid.as_ptr(), &mut preview);
            if rc != 0 || preview.is_null() { input_close(input); input_release(input); return Err(format!("preview output: {}", cam_error(rc))); }
            let mut photo_output = std::ptr::null_mut();
            if !photo_receiver.is_null() {
                if let (Some(pp), Some(cpo)) = (photo_profile, api.create_photo_output) {
                    let psid = CString::new(photo_surface.to_string()).unwrap();
                    if cpo(mgr, &pp, psid.as_ptr(), &mut photo_output) != 0 { photo_output = std::ptr::null_mut(); }
                }
            }
            let mut session = std::ptr::null_mut();
            let rc = create_session(mgr, &mut session);
            if rc != 0 || session.is_null() { preview_release(preview); input_close(input); input_release(input); return Err(format!("session: {}", cam_error(rc))); }
            let mut steps = vec![("beginConfig", begin(session)), ("addInput", add_input(session, input)), ("addPreviewOutput", add_preview(session, preview))];
            if !photo_output.is_null() {
                if let Some(apo) = api.add_photo_output { if apo(session, photo_output) != 0 { api.photo_release.unwrap_or(noop_release)(photo_output); photo_output = std::ptr::null_mut(); } }
            }
            steps.push(("commitConfig", commit(session))); steps.push(("start", start(session)));
            for (what, rc) in steps {
                if rc != 0 {
                    session_release(session); preview_release(preview); if !photo_output.is_null() { api.photo_release.unwrap_or(noop_release)(photo_output); } input_close(input); input_release(input);
                    if !photo_receiver.is_null() { api.receiver_release.map(|f| f(photo_receiver)); }
                    return Err(format!("{what}: {}", cam_error(rc)));
                }
            }
            let photo = if photo_output.is_null() { if !photo_receiver.is_null() { api.receiver_release.map(|f| f(photo_receiver)); photo_receiver = std::ptr::null_mut(); } None } else {
                let shared = Arc::new(PhotoShared { receiver: AtomicU64::new(photo_receiver as usize as u64), pending: Mutex::new(VecDeque::new()) });
                if let Ok(mut a) = ACTIVE_PHOTO.lock() { *a = Some(shared.clone()); }
                if let Some(on) = api.receiver_on { on(photo_receiver, on_photo_arrived); }
                if let Some(pp) = photo_profile { info!("camera-oh: stills at {}x{}", pp.size.width, pp.size.height); }
                Some(shared)
            };
            info!("camera-oh: streaming {name} at {}x{}", profile.size.width, profile.size.height);
            self.session = Some(Session { input, session, preview, photo_output, photo_receiver, photo, front, size: profile.size, video: None });
        }
        Ok((profile.size.width, profile.size.height))
    }

    pub fn stop_preview(&mut self) {
        if self.session.as_ref().map_or(false, |s| s.video.is_some()) { match self.stop_video() { Ok(path) => push_result(CaptureResult::VideoStopped { path }), Err(e) => error!("camera-oh: {e}") } }
        let Some(s) = self.session.take() else { return };
        if let Some(p) = &s.photo { p.receiver.store(0, Ordering::Release); }
        if let Ok(mut a) = ACTIVE_PHOTO.lock() { *a = None; }
        let Some(api) = api() else { return };
        unsafe {
            if !s.photo_receiver.is_null() { api.receiver_off.map(|f| f(s.photo_receiver)); }
            api.session_stop.map(|f| f(s.session)); api.session_release.map(|f| f(s.session));
            api.preview_release.map(|f| f(s.preview));
            if !s.photo_output.is_null() { api.photo_release.map(|f| f(s.photo_output)); }
            api.input_close.map(|f| f(s.input)); api.input_release.map(|f| f(s.input));
            if !s.photo_receiver.is_null() { api.receiver_release.map(|f| f(s.photo_receiver)); }
        }
        info!("camera-oh: stopped");
    }

    pub fn control(&mut self, control: Control) {
        let (Some(api), Some(s)) = (api(), self.session.as_ref()) else { return };
        let h = s.session;
        let check = |what: &str, rc: Rc32| if rc != 0 { error!("camera-oh: {what}: {}", cam_error(rc)) };
        unsafe {
            match control {
                Control::FocusPoint { x, y } => {
                    let p = CameraPoint { x: x.clamp(0.0, 1.0), y: y.clamp(0.0, 1.0) };
                    if let Some(f) = api.set_focus_mode { check("focus mode", f(h, 2)); }
                    if let Some(f) = api.set_focus_point { check("focus point", f(h, p)); }
                    if let Some(f) = api.set_exposure_mode { check("exposure mode", f(h, 1)); }
                    if let Some(f) = api.set_metering_point { check("metering point", f(h, p)); }
                }
                Control::ContinuousFocus => { if let Some(f) = api.set_focus_mode { check("focus mode", f(h, 1)); } if let Some(f) = api.set_exposure_mode { check("exposure mode", f(h, 2)); } }
                Control::Zoom(z) => {
                    let (mut lo, mut hi) = (1.0f32, z.max(1.0));
                    if let Some(f) = api.get_zoom_range { if f(h, &mut lo, &mut hi) != 0 || hi < lo { lo = 1.0; hi = z.max(1.0); } }
                    if let Some(f) = api.set_zoom { check("zoom", f(h, z.clamp(lo, hi))); }
                }
                Control::ExposureBias(ev) => {
                    let (mut lo, mut hi, mut step) = (-4.0f32, 4.0f32, 0.0f32);
                    if let Some(f) = api.get_ev_range { if f(h, &mut lo, &mut hi, &mut step) != 0 || hi < lo { lo = -4.0; hi = 4.0; } }
                    if let Some(f) = api.set_ev { check("exposure bias", f(h, ev.clamp(lo, hi))); }
                }
                Control::Flash(mode) => { if let Some(f) = api.set_flash_mode { check("flash mode", f(h, mode)); } }
            }
        }
    }

    pub fn capture(&mut self, request: Capture) {
        let fail = |what: &str, error: String| push_result(CaptureResult::Failed { what: what.into(), error });
        let Some(api) = api() else { fail("capture", "camera NDK unavailable".into()); return };
        if self.session.is_none() { fail("capture", "no camera session".into()); return; }
        match request {
            Capture::Photo { path } => {
                let s = self.session.as_ref().unwrap();
                let (Some(shared), Some(capture)) = (s.photo.as_ref(), api.photo_capture) else { fail("photo", "this session has no still output".into()); return };
                let mut rotation = 90;
                if let Some(get) = api.photo_rotation { let mut r = 0; if unsafe { get(s.photo_output, 0, &mut r) } == 0 { rotation = r; } }
                if let Ok(mut p) = shared.pending.lock() { p.push_back(path.clone()); }
                let rc = unsafe { capture(s.photo_output, CameraPhotoCaptureSetting { quality: 0, rotation, location: std::ptr::null_mut(), mirror: s.front }) };
                if rc != 0 { if let Ok(mut p) = shared.pending.lock() { p.retain(|q| *q != path); } fail("photo", cam_error(rc)); } else { info!("camera-oh: capturing {path}"); }
            }
            Capture::StartVideo { path, audio } => match self.start_video(path.clone(), audio) { Ok(()) => push_result(CaptureResult::VideoStarted { path }), Err(e) => fail("video", e) },
            Capture::StopVideo => match self.stop_video() { Ok(path) => push_result(CaptureResult::VideoStopped { path }), Err(e) => fail("video", e) },
            Capture::PauseVideo | Capture::ResumeVideo => {
                let resume = matches!(request, Capture::ResumeVideo);
                let s = self.session.as_ref().unwrap();
                let (Some(rec), Some(f)) = (s.video.as_ref(), if resume { api.recorder_resume } else { api.recorder_pause }) else { fail("video", "not recording".into()); return };
                let rc = unsafe { f(rec.recorder) };
                if rc != 0 { fail("video", format!("recorder {}: {rc}", if resume { "resume" } else { "pause" })); } else { push_result(if resume { CaptureResult::VideoResumed } else { CaptureResult::VideoPaused }); }
            }
        }
    }

    fn start_video(&mut self, path: String, audio: bool) -> Result<(), String> {
        let api = api().ok_or("camera NDK unavailable")?;
        let s = self.session.as_mut().ok_or("no camera session")?;
        if s.video.is_some() { return Err("already recording".into()); }
        let (Some(create), Some(prepare), Some(get_surface), Some(start), Some(surface_id), Some(create_output), Some(add_output), Some(out_start)) =
            (api.recorder_create, api.recorder_prepare, api.recorder_surface, api.recorder_start, api.window_surface_id, api.create_video_output, api.add_video_output, api.video_output_start) else { return Err("the AV recorder NDK is not available".into()) };
        let device = self.devices.iter().find(|d| d.front == s.front).ok_or("unknown camera")?;
        let size = s.size;
        let profile = device.videos.iter().filter(|p| p.size.width == size.width && p.size.height == size.height && p.range.min <= 30 && p.range.max >= 30).max_by_key(|p| p.range.max)
            .or_else(|| device.videos.iter().find(|p| p.size.width == size.width && p.size.height == size.height)).copied().ok_or_else(|| format!("no video profile at {}x{}", size.width, size.height))?;
        let fps = 30i32.clamp(profile.range.min as i32, profile.range.max as i32);
        std::path::Path::new(&path).parent().map(std::fs::create_dir_all);
        let file = std::fs::OpenOptions::new().read(true).write(true).create(true).truncate(true).open(&path).map_err(|e| format!("create {path}: {e}"))?;
        let strings: Vec<CString> = [format!("fd://{}", file.as_raw_fd()), "90".into(), String::new(), String::new(), String::new()].iter().map(|t| CString::new(t.as_str()).unwrap()).collect();
        let recorder = unsafe { create() };
        if recorder.is_null() { return Err("recorder create".into()); }
        let release = api.recorder_release.unwrap_or(noop_release);
        let mut config = AvConfig {
            audio_source_type: if audio { 1 } else { -1 }, video_source_type: 0,
            profile: AvProfile { audio_bitrate: if audio { 96_000 } else { 0 }, audio_channels: if audio { 2 } else { 0 }, audio_codec: if audio { 3 } else { 0 }, audio_sample_rate: if audio { 48_000 } else { 0 }, file_format: 2, video_bitrate: 12_000_000, video_codec: 2, video_frame_width: size.width as i32, video_frame_height: size.height as i32, video_frame_rate: fps, is_hdr: false, enable_temporal_scale: false },
            url: strings[0].as_ptr() as *mut c_char, file_generation_mode: 0,
            metadata: AvMetadata { genre: strings[2].as_ptr() as *mut c_char, video_orientation: strings[1].as_ptr() as *mut c_char, location: AvLocation { latitude: 0.0, longitude: 0.0 }, custom_info: AvMetadataTemplate { key: strings[3].as_ptr() as *mut c_char, value: strings[4].as_ptr() as *mut c_char } },
            max_duration: 3600,
        };
        unsafe {
            if let Some(f) = api.recorder_set_state_cb { f(recorder, on_recorder_state, std::ptr::null_mut()); }
            if let Some(f) = api.recorder_set_error_cb { f(recorder, on_recorder_error, std::ptr::null_mut()); }
            let rc = prepare(recorder, &mut config);
            if rc != 0 { release(recorder); return Err(format!("recorder prepare: {rc}")); }
            if let Some(rot) = api.recorder_rotation { rot(recorder, 90); }
            let mut window = std::ptr::null_mut(); let mut sid = 0u64;
            if get_surface(recorder, &mut window) != 0 || window.is_null() || surface_id(window, &mut sid) != 0 { release(recorder); return Err("recorder surface".into()); }
            let sid = CString::new(sid.to_string()).unwrap();
            let mut output = std::ptr::null_mut();
            let rc = create_output(self.manager, &profile, sid.as_ptr(), &mut output);
            if rc != 0 || output.is_null() { release(recorder); return Err(format!("video output: {}", cam_error(rc))); }
            let out_release = api.video_output_release.unwrap_or(noop_release);
            let steps = [("beginConfig", api.begin_config.unwrap()(s.session)), ("addVideoOutput", add_output(s.session, output)), ("commitConfig", api.commit_config.unwrap()(s.session)), ("start", api.session_start.unwrap()(s.session))];
            for (what, rc) in steps { if rc != 0 { out_release(output); release(recorder); return Err(format!("{what}: {}", cam_error(rc))); } }
            let rc = out_start(output);
            if rc != 0 { out_release(output); release(recorder); return Err(format!("video output start: {}", cam_error(rc))); }
            let rc = start(recorder);
            if rc != 0 { api.video_output_stop.map(|f| f(output)); out_release(output); release(recorder); return Err(format!("recorder start: {rc}")); }
            info!("camera-oh: recording {path} at {}x{} {fps} fps{}", size.width, size.height, if audio { " with audio" } else { "" });
            s.video = Some(VideoRec { recorder, output, path, _file: file, _strings: strings });
        }
        Ok(())
    }

    fn stop_video(&mut self) -> Result<String, String> {
        let api = api().ok_or("camera NDK unavailable")?;
        let s = self.session.as_mut().ok_or("no camera session")?;
        let rec = s.video.take().ok_or("not recording")?;
        unsafe {
            api.recorder_stop.map(|f| f(rec.recorder));
            api.video_output_stop.map(|f| f(rec.output));
            if let Some(remove) = api.remove_video_output { api.begin_config.map(|f| f(s.session)); remove(s.session, rec.output); api.commit_config.map(|f| f(s.session)); api.session_start.map(|f| f(s.session)); }
            api.video_output_release.map(|f| f(rec.output));
            api.recorder_release.map(|f| f(rec.recorder));
        }
        info!("camera-oh: recording stopped: {}", rec.path);
        Ok(rec.path)
    }
}

unsafe extern "C" fn on_recorder_state(_r: *mut c_void, state: i32, reason: i32, _u: *mut c_void) { info!("camera-oh: recorder state {state} (reason {reason})"); }
unsafe extern "C" fn on_recorder_error(_r: *mut c_void, code: i32, msg: *const c_char, _u: *mut c_void) {
    let msg = if msg.is_null() { String::new() } else { CStr::from_ptr(msg).to_string_lossy().into_owned() };
    error!("camera-oh: recorder error {code}: {msg}");
}

unsafe extern "C" fn on_photo_arrived(receiver: *mut c_void) {
    let shared = ACTIVE_PHOTO.lock().ok().and_then(|a| a.clone());
    let Some(shared) = shared else { return };
    if shared.receiver.load(Ordering::Acquire) != receiver as usize as u64 { return; }
    let Some(api) = api() else { return };
    let Some(path) = shared.pending.lock().ok().and_then(|mut p| p.pop_front()) else { return };
    let mut image = std::ptr::null_mut();
    let Some(read) = api.receiver_read_latest else { return };
    if read(receiver, &mut image) != 0 || image.is_null() { push_result(CaptureResult::Failed { what: "photo".into(), error: "read still".into() }); return; }
    let result = read_jpeg(api, image, &path);
    api.image_release.map(|f| f(image));
    match result {
        Ok((w, h)) => { info!("camera-oh: still written to {path}"); push_result(CaptureResult::Photo { path, width: w, height: h }); }
        Err(e) => push_result(CaptureResult::Failed { what: "photo".into(), error: e }),
    }
}

unsafe fn read_jpeg(api: &Api, image: *mut c_void, path: &str) -> Result<(u32, u32), String> {
    let mut size = ImageSize { width: 0, height: 0 };
    api.image_size.map(|f| f(image, &mut size));
    let types_fn = api.image_component_types.ok_or("no component api")?;
    let mut n = 0usize;
    if types_fn(image, std::ptr::null_mut(), &mut n) != 0 || n == 0 { return Err("still has no components".into()); }
    let mut types = vec![0u32; n]; let mut out = types.as_mut_ptr();
    if types_fn(image, &mut out, &mut n) != 0 { return Err("component types".into()); }
    let mut buffer = std::ptr::null_mut();
    if api.image_byte_buffer.ok_or("no byte buffer api")?(image, types[0], &mut buffer) != 0 || buffer.is_null() { return Err("byte buffer".into()); }
    let mut len = 0usize;
    if let Some(f) = api.image_buffer_size { f(image, types[0], &mut len); }
    let mut base: *mut c_void = std::ptr::null_mut();
    if api.buffer_map.ok_or("no map api")?(buffer, &mut base) != 0 || base.is_null() { return Err("buffer map".into()); }
    if len == 0 { len = size.width as usize * size.height as usize; }
    let bytes = std::slice::from_raw_parts(base as *const u8, len);
    let end = bytes.windows(2).rposition(|w| w == [0xFF, 0xD9]).map(|i| i + 2).unwrap_or(bytes.len());
    let written = std::path::Path::new(path).parent().map(std::fs::create_dir_all).transpose().and_then(|_| std::fs::write(path, &bytes[..end]));
    api.buffer_unmap.map(|f| f(buffer));
    written.map_err(|e| format!("write {path}: {e}"))?;
    Ok((size.width, size.height))
}
