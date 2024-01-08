use editor::messages::{prelude::*, tool::utility_types::ToolType};

fn dispatch(message: Message) -> Vec<FrontendMessage> {
    let mut guard = crate::EDITOR.lock().unwrap();
    let editor = (*guard).as_mut().unwrap();
    let responses = editor.handle_message(message);
    responses
}

#[tauri::command]
#[specta::specta]
pub fn init_after_frontend_ready() {
}

/// Mouse movement within the screenspace bounds of the viewport
#[tauri::command]
#[specta::specta]
pub fn on_mouse_move(x: f64, y: f64, mouse_keys: u8, modifiers: u8) {

}

/// Mouse scrolling within the screenspace bounds of the viewport
#[tauri::command]
#[specta::specta]
pub fn on_wheel_scroll(x: f64, y: f64, mouse_keys: u8, wheel_delta_x: f32, wheel_delta_y: f32, wheel_delta_z: f32, modifiers: u8) {
}

/// A mouse button depressed within screenspace the bounds of the viewport
#[tauri::command]
#[specta::specta]
pub fn on_mouse_down(x: f64, y: f64, mouse_keys: u8, modifiers: u8) {
}

/// A mouse button released
#[tauri::command]
#[specta::specta]
pub fn on_mouse_up(x: f64, y: f64, mouse_keys: u8, modifiers: u8) {
}

/// Mouse double clicked
#[tauri::command]
#[specta::specta]
pub fn on_double_click(x: f64, y: f64, mouse_keys: u8, modifiers: u8) {
}

/// A keyboard button depressed within screenspace the bounds of the viewport
#[tauri::command]
#[specta::specta]
pub fn on_key_down(name: String, modifiers: u8) {
}

/// A keyboard button released
#[tauri::command]
#[specta::specta]
pub fn on_key_up(name: String, modifiers: u8) {

}

// Tools

#[tauri::command]
#[specta::specta]
pub fn activate_tool(tool_type: ToolType) -> Vec<FrontendMessage> {
    dispatch(editor::messages::message::Message::Tool(ToolMessage::ActivateTool { tool_type }))
}