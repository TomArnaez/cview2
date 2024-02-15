use std::collections::HashMap;

use std::sync::{Arc, Mutex};
use serde::ser::SerializeStruct;
use serde::{Deserialize, Deserializer, Serialize, Serializer};
use tauri::{AppHandle, ICoreWebView2SharedBuffer, Manager, State};
use uuid::Uuid;

mod capture;

// All shared buffers are backed by Rust, due to not being able to pass SharedArrayBuffer from TS -> Rust
#[derive(Debug, Default)]
struct SharedBufferManager {
    buffers: HashMap<Uuid, ICoreWebView2SharedBuffer>,
}

impl SharedBufferManager {
    fn new() -> Self {
        Self {
            ..Default::default()
        }
    }

    fn new_buffer(&mut self, size: u64, app: AppHandle) -> SharedBuffer {
        let buffer = app.get_webview_window("main").unwrap().create_shared_buffer(size);
        let mut data_ptr: *mut u8 = std::ptr::null_mut();
        unsafe { 
            buffer.Buffer(&mut data_ptr as *mut *mut u8).unwrap();
            println!("{:?}", data_ptr);
            let shared_buffer = SharedBuffer {
                uuid: Uuid::new_v4(),
                size,
                buffer: data_ptr

            };
            self.buffers.insert(shared_buffer.uuid, buffer);
            shared_buffer
        }
    }
}
unsafe impl Send for SharedBufferManager {}
unsafe impl Sync for SharedBufferManager {}


#[derive(Debug)]
struct SharedBuffer {
    uuid: Uuid,
    size: u64, 
    buffer: *mut u8,
}

impl SharedBuffer {


    // pub fn get_data(&self) -> &[u8] {
    //     self.buffer
    // }

    // pub fn get_data_mut(&mut self) -> &mut [u8] {
    //     &mut self.buffer
    // }
}

unsafe impl Send for SharedBuffer {}

impl Serialize for SharedBuffer {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        let mut state = serializer.serialize_struct("SharedBuffer", 2)?;
        state.serialize_field("size", &self.size);
        state.serialize_field("uuid", &self.uuid);
        state.serialize_field("buffer", &(self.buffer as usize));
        state.end()
    }
}

impl<'de> Deserialize<'de> for SharedBuffer {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        #[derive(Deserialize)]
        struct BufferData {
            uuid: Uuid,
            size: u64, 
            buffer: usize,
        }

        let BufferData { uuid, size, buffer } = BufferData::deserialize(deserializer)?;
        Ok(SharedBuffer {
            uuid,
            size,
            buffer: buffer as *mut u8,
        })
    }
}

#[tauri::command]
fn create_shared_buffer(size: u64, shared_buffer_manager: State<Mutex<SharedBufferManager>>, app: AppHandle) {
    let buffer = shared_buffer_manager.lock().unwrap().new_buffer(size, app);
    println!("Original {:?}", buffer);
    let serialized_data = serde_json::to_string(&buffer).unwrap();
    println!("serialized, {serialized_data}");
    let deserialized: SharedBuffer = serde_json::from_str(&serialized_data).unwrap();
    println!("Deserialize {:?}", deserialized);
}


#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .setup(|app| {
            app.manage(Mutex::new(SharedBufferManager::new()));
            Ok(())
        })
        .invoke_handler(tauri::generate_handler![create_shared_buffer])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}