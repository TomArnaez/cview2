use log::error;
use serde::{ser::SerializeStruct, Serialize};
use specta::Type;
use std::ops::{Deref, DerefMut};
use tauri::{AppHandle, Manager, Runtime};
use windows_core::{ComInterface, HSTRING, PCWSTR};
use webview2_com::Microsoft::Web::WebView2::Win32::{ICoreWebView2Environment12, ICoreWebView2SharedBuffer, ICoreWebView2_19, COREWEBVIEW2_SHARED_BUFFER_ACCESS_READ_WRITE};
use tokio::sync::oneshot;
use uuid::Uuid;

#[derive(Clone, Serialize, Debug, Type)] 
pub enum TypeTag {
    U16,
}

pub trait HasTypeTag {
    fn type_tag() -> TypeTag;
}

impl HasTypeTag for u16 {
    fn type_tag() -> TypeTag {
        TypeTag::U16
    }
}

#[derive(Debug, Type)]
pub struct SharedBuffer<T: HasTypeTag> {
    uuid: Uuid,
    len: usize,
    #[specta(skip)]
    shared_buffer: ICoreWebView2SharedBuffer,
    buffer: *mut T,
}

impl<T: HasTypeTag> Serialize for SharedBuffer<T> {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
        where
            S: serde::Serializer {
                let mut state = serializer.serialize_struct("SharedBuffer", 3)?;
                state.serialize_field("uuid", &self.uuid)?;
                state.serialize_field("len", &self.len)?;
                state.serialize_field("type", &T::type_tag())?;
                state.end()

    }
}

unsafe impl<T: HasTypeTag> Send for SharedBuffer<T> {}

impl<T: HasTypeTag> Drop for SharedBuffer<T> {
    fn drop(&mut self) {
        unsafe { self.shared_buffer.Close().unwrap(); }
    }
}

impl<T: HasTypeTag> Deref for SharedBuffer<T> {
    type Target = [T];

    fn deref(&self) -> &Self::Target {
        unsafe { std::slice::from_raw_parts(self.buffer, self.len) }
    }
}

impl<T: HasTypeTag> DerefMut for SharedBuffer<T> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { std::slice::from_raw_parts_mut(self.buffer, self.len)}
    }
}

impl<T: HasTypeTag + 'static> SharedBuffer<T> {
    pub fn new<R: Runtime>(len: usize, app: AppHandle<R>) -> Self {
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
                    buffer: buffer as *mut T
                }) {
                    Err(_) => error!("Failed to send shared buffer"),
                    _ => {}
                }
            }).unwrap();

        rx.blocking_recv().unwrap()
    }
}

impl<T: HasTypeTag + Clone + 'static> From<(&[T], AppHandle)> for SharedBuffer<T> where T: HasTypeTag {
    fn from(item: (&[T], AppHandle)) -> Self {
        let mut buffer = SharedBuffer::<T>::new(item.0.len(), item.1);
        buffer.clone_from_slice(&item.0);
        buffer
    }
}