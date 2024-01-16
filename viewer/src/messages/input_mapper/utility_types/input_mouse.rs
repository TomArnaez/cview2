use std::ops::Div;

use bitflags::bitflags;
use nalgebra::Vector2;
use serde::{Deserialize, Serialize};

// Origin is top left
pub type ViewportPosition = Vector2<f64>;
pub type EditorPosition = Vector2<f64>;

#[derive(Debug, Default)]
pub struct ViewportBounds {
    pub top_left: Vector2<f64>,
    pub bottom_right: Vector2<f64>
}

impl ViewportBounds {
    pub fn size(&self) -> Vector2<f64> {
		self.bottom_right - self.top_left
	}

    pub fn centre(&self) -> Vector2<f64> {
        (self.bottom_right - self.top_left).div(2.0)
    }

	pub fn in_bounds(&self, position: ViewportPosition) -> bool {
		position.x >= 0. && position.y >= 0. && position.x <= self.bottom_right.x && position.y <= self.bottom_right.y
	}
}

#[derive(Copy, Debug, Clone, Default, Serialize, Deserialize)]
pub struct ScrollDelta {
    pub x: i32,
    pub y: i32
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct MouseState {
    pub position: ViewportPosition,
    pub mouse_keys: MouseKeys,
    pub scroll_delta: ScrollDelta
}

impl MouseState {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn from_position(x: f64, y: f64) -> Self {
		Self {
			position: Vector2::new(x, y),
			mouse_keys: MouseKeys::default(),
			scroll_delta: ScrollDelta::default(),
		}
    }
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct EditorMouseState {
	pub editor_position: EditorPosition,
	pub mouse_keys: MouseKeys,
	pub scroll_delta: ScrollDelta,
}

impl EditorMouseState {
	pub fn new() -> Self {
		Self::default()
	}

	pub fn from_editor_position(x: f64, y: f64) -> Self {
		Self {
			editor_position: Vector2::new(x, y),
			mouse_keys: MouseKeys::default(),
			scroll_delta: ScrollDelta::default(),
		}
	}

	pub fn to_mouse_state(&self, active_viewport_bounds: &ViewportBounds) -> MouseState {
		MouseState {
			position: self.editor_position - active_viewport_bounds.top_left,
			mouse_keys: self.mouse_keys,
			scroll_delta: self.scroll_delta,
		}
	}
}


bitflags! {
    #[derive(Copy, Debug, Clone, Default, Deserialize, Serialize)]
    pub struct MouseKeys: u8 {
		const LEFT   = 0b0000_0001;
		const RIGHT  = 0b0000_0010;
		const MIDDLE = 0b0000_0100;
	}
}
pub enum MouseButton {
    Left,
    Right,
    Middle
}