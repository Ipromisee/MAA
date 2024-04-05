/* automatically generated by rust-bindgen 0.63.0 */

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct AsstExtAPI {
    _unused: [u8; 0],
}
pub type AsstHandle = *mut AsstExtAPI;
pub type AsstBool = u8;
pub type AsstSize = u64;
pub type AsstMsgId = i32;
pub type AsstTaskId = i32;
pub type AsstAsyncCallId = i32;
pub type AsstStaticOptionKey = i32;
pub type AsstInstanceOptionKey = i32;
pub type AsstApiCallback = ::std::option::Option<
    unsafe extern "C" fn(
        msg: AsstMsgId,
        detail_json: *const ::std::os::raw::c_char,
        custom_arg: *mut ::std::os::raw::c_void,
    ),
>;
extern "C" {
    pub fn AsstSetUserDir(path: *const ::std::os::raw::c_char) -> AsstBool;
}
extern "C" {
    pub fn AsstLoadResource(path: *const ::std::os::raw::c_char) -> AsstBool;
}
extern "C" {
    pub fn AsstSetStaticOption(
        key: AsstStaticOptionKey,
        value: *const ::std::os::raw::c_char,
    ) -> AsstBool;
}
extern "C" {
    pub fn AsstCreate() -> AsstHandle;
}
extern "C" {
    pub fn AsstCreateEx(
        callback: AsstApiCallback,
        custom_arg: *mut ::std::os::raw::c_void,
    ) -> AsstHandle;
}
extern "C" {
    pub fn AsstDestroy(handle: AsstHandle);
}
extern "C" {
    pub fn AsstSetInstanceOption(
        handle: AsstHandle,
        key: AsstInstanceOptionKey,
        value: *const ::std::os::raw::c_char,
    ) -> AsstBool;
}
extern "C" {
    pub fn AsstConnect(
        handle: AsstHandle,
        adb_path: *const ::std::os::raw::c_char,
        address: *const ::std::os::raw::c_char,
        config: *const ::std::os::raw::c_char,
    ) -> AsstBool;
}
extern "C" {
    pub fn AsstAppendTask(
        handle: AsstHandle,
        type_: *const ::std::os::raw::c_char,
        params: *const ::std::os::raw::c_char,
    ) -> AsstTaskId;
}
extern "C" {
    pub fn AsstSetTaskParams(
        handle: AsstHandle,
        id: AsstTaskId,
        params: *const ::std::os::raw::c_char,
    ) -> AsstBool;
}
extern "C" {
    pub fn AsstStart(handle: AsstHandle) -> AsstBool;
}
extern "C" {
    pub fn AsstStop(handle: AsstHandle) -> AsstBool;
}
extern "C" {
    pub fn AsstRunning(handle: AsstHandle) -> AsstBool;
}
extern "C" {
    pub fn AsstAsyncConnect(
        handle: AsstHandle,
        adb_path: *const ::std::os::raw::c_char,
        address: *const ::std::os::raw::c_char,
        config: *const ::std::os::raw::c_char,
        block: AsstBool,
    ) -> AsstAsyncCallId;
}
extern "C" {
    pub fn AsstAsyncClick(handle: AsstHandle, x: i32, y: i32, block: AsstBool) -> AsstAsyncCallId;
}
extern "C" {
    pub fn AsstAsyncScreencap(handle: AsstHandle, block: AsstBool) -> AsstAsyncCallId;
}
extern "C" {
    pub fn AsstGetImage(
        handle: AsstHandle,
        buff: *mut ::std::os::raw::c_void,
        buff_size: AsstSize,
    ) -> AsstSize;
}
extern "C" {
    pub fn AsstGetUUID(
        handle: AsstHandle,
        buff: *mut ::std::os::raw::c_char,
        buff_size: AsstSize,
    ) -> AsstSize;
}
extern "C" {
    pub fn AsstGetTasksList(
        handle: AsstHandle,
        buff: *mut AsstTaskId,
        buff_size: AsstSize,
    ) -> AsstSize;
}
extern "C" {
    pub fn AsstGetNullSize() -> AsstSize;
}
extern "C" {
    pub fn AsstGetVersion() -> *const ::std::os::raw::c_char;
}
extern "C" {
    pub fn AsstLog(level: *const ::std::os::raw::c_char, message: *const ::std::os::raw::c_char);
}
