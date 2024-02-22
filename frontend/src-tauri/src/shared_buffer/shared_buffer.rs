use log::error;
use tauri_specta::Event;
use std::{error, marker::PhantomData, ops::Deref};
use serde::Serialize;
use tauri::{AppHandle, Manager, Runtime};
use windows_core::{w, ComInterface, HSTRING, PCWSTR};
use webview2_com::Microsoft::Web::WebView2::Win32::{COREWEBVIEW2_SHARED_BUFFER_ACCESS_READ_ONLY, ICoreWebView2Environment12, ICoreWebView2SharedBuffer, ICoreWebView2_19,};
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
        unsafe { self.shared_buffer.Close(); }
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
                let mut size: *mut u64 = std::ptr::null_mut();
                unsafe { shared_buffer.Size(size)}.unwrap();
                unsafe { println!{"{:?}", size} };
                let mut buffer: *mut u8 = std::ptr::null_mut();
                unsafe {shared_buffer.Buffer(&mut buffer as *mut *mut u8) }.unwrap();
                unsafe {webview2.PostSharedBufferToScript(&shared_buffer, COREWEBVIEW2_SHARED_BUFFER_ACCESS_READ_ONLY, w!("Test")).unwrap();}
                match tx.send(SharedBuffer {
                    uuid: Uuid::new_v4(),
                    len,
                    shared_buffer,
                    buffer: buffer as *mut T,
                    _marker: PhantomData
                }) {
                    Err(_) => error!("Failed to send shared buffer"),
                    _ => {}
                }
            }).unwrap();

        let shared_buffer = rx.await.unwrap();
        NewSharedBufferEvent(shared_buffer.uuid, shared_buffer.len as u32).emit_all(&app).unwrap();
        shared_buffer
    }
}

#[derive(Debug, Clone, Serialize, specta::Type, tauri_specta::Event)]
pub struct NewSharedBufferEvent(Uuid, u32);