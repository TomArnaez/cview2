use log::error;
use std::{marker::PhantomData, ops::Deref};
use tauri::{AppHandle, Manager, Runtime};
use windows_core::{ComInterface, HSTRING, PCWSTR};
use webview2_com::Microsoft::Web::WebView2::Win32::{ICoreWebView2Environment12, ICoreWebView2SharedBuffer, ICoreWebView2_19, COREWEBVIEW2_SHARED_BUFFER_ACCESS_READ_ONLY, COREWEBVIEW2_SHARED_BUFFER_ACCESS_READ_WRITE};
use tokio::sync::oneshot;
use uuid::Uuid;

pub struct SharedBuffer<T> {
    uuid: Uuid,
    len: usize, 
    shared_buffer: ICoreWebView2SharedBuffer,
    buffer: *mut T,
    _marker: PhantomData<T>
}

unsafe impl<T> Send for SharedBuffer<T> {}

impl<T> Drop for SharedBuffer<T> {
    fn drop(&mut self) {
        unsafe { self.shared_buffer.Close().unwrap(); }
    }
}

impl <T> Deref for SharedBuffer<T> {
    type Target = [T];

    fn deref(&self) -> &Self::Target {
        unsafe { std::slice::from_raw_parts(self.buffer, self.len) }
    }
}

impl<T: 'static> SharedBuffer<T> {
    pub async fn new<R: Runtime>(len: usize, app: AppHandle<R>) -> Self {
        let size = (std::mem::size_of::<T>() * len) as u64;
        let (tx, rx) = oneshot::channel();
        app.get_webview_window("main").unwrap().with_webview(move |webview| {
                let webview2 = unsafe {webview.controller().CoreWebView2() }.unwrap().cast::<ICoreWebView2_19>().unwrap();
                let environment: ICoreWebView2Environment12 = unsafe { webview2.Environment() }.unwrap().cast::<ICoreWebView2Environment12>().unwrap();
                let shared_buffer: webview2_com::Microsoft::Web::WebView2::Win32::ICoreWebView2SharedBuffer = unsafe {environment.CreateSharedBuffer(size).unwrap()};
                let mut buffer: *mut u8 = std::ptr::null_mut();
                unsafe {shared_buffer.Buffer(&mut buffer as *mut *mut u8)}.unwrap();
                let uuid = Uuid::new_v4();
                let additional_data_json = format!(r#"{{"uuid":"{}"}}"#, uuid.to_string());
                unsafe {webview2.PostSharedBufferToScript(&shared_buffer, COREWEBVIEW2_SHARED_BUFFER_ACCESS_READ_WRITE, PCWSTR(HSTRING::from(additional_data_json).as_ptr())).unwrap();}
                match tx.send(SharedBuffer {
                    uuid,
                    len,
                    shared_buffer,
                    buffer: buffer as *mut T,
                    _marker: PhantomData
                }) {
                    Err(_) => error!("Failed to send shared buffer"),
                    _ => {}
                }
            }).unwrap();

        rx.await.unwrap()
    }
}