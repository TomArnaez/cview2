use tauri::{AppHandle, Manager};
use uuid::Uuid;
use tokio::sync::oneshot;
use windows_core::ComInterface;
use webview2_com::Microsoft::Web::WebView2::Win32::{ICoreWebView2_19, ICoreWebView2Environment12};

mod capture;

#[derive(Debug)]
struct SharedBuffer {
    uuid: Uuid,
    size: u64, 
    buffer: *mut u8,
}

unsafe impl Send for SharedBuffer {}

#[tauri::command]
fn create_shared_buffer(size: u64, app: AppHandle) -> SharedBuffer {
    let (tx, rx) = oneshot::channel();
    app.get_webview_window("main").unwrap().with_webview(move |webview| {
            let webview2 = unsafe {webview.controller().CoreWebView2() }.unwrap().cast::<ICoreWebView2_19>().unwrap();
            let environment: ICoreWebView2Environment12 = unsafe { webview2.Environment() }.unwrap().cast::<ICoreWebView2Environment12>().unwrap();
            let shared_buffer = unsafe {environment.CreateSharedBuffer(size) }.unwrap();
            let mut buffer: *mut u8 = std::ptr::null_mut();
            unsafe {shared_buffer.Buffer(&mut buffer as *mut *mut u8) }.unwrap();
            tx.send(SharedBuffer {
                uuid: Uuid::new_v4(),
                size,
                buffer
            }).unwrap();
        }).unwrap();

    rx.blocking_recv().unwrap()
}

#[tauri::command]
fn scan_cameras() {

}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_shell::init())
        .invoke_handler(tauri::generate_handler![])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
