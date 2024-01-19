use core::fmt;

use serde::{Serialize, Deserialize};

use crate::messages::prelude::*;

#[derive(Debug, Clone, Copy, PartialEq, Deserialize, Serialize)]
#[repr(u8)]
pub enum Key {
	// Writing system keys
	Digit0,
	Digit1,
	Digit2,
	Digit3,
	Digit4,
	Digit5,
	Digit6,
	Digit7,
	Digit8,
	Digit9,
	//
	KeyA,
	KeyB,
	KeyC,
	KeyD,
	KeyE,
	KeyF,
	KeyG,
	KeyH,
	KeyI,
	KeyJ,
	KeyK,
	KeyL,
	KeyM,
	KeyN,
	KeyO,
	KeyP,
	KeyQ,
	KeyR,
	KeyS,
	KeyT,
	KeyU,
	KeyV,
	KeyW,
	KeyX,
	KeyY,
	KeyZ,
	//
	Backquote,
	Backslash,
	BracketLeft,
	BracketRight,
	Comma,
	Equal,
	Minus,
	Period,
	Quote,
	Semicolon,
	Slash,

	// Functional keys
	Alt,
	Meta,
	Shift,
	Control,
	Backspace,
	CapsLock,
	ContextMenu,
	Enter,
	Space,
	Tab,

	// Control pad keys
	Delete,
	End,
	Help,
	Home,
	Insert,
	PageDown,
	PageUp,

	// Arrow pad keys
	ArrowDown,
	ArrowLeft,
	ArrowRight,
	ArrowUp,

	// Numpad keys
	// Numpad0,
	// Numpad1,
	// Numpad2,
	// Numpad3,
	// Numpad4,
	// Numpad5,
	// Numpad6,
	// Numpad7,
	// Numpad8,
	// Numpad9,
	NumLock,
	NumpadAdd,
	// NumpadBackspace,
	// NumpadClear,
	// NumpadClearEntry,
	// NumpadComma,
	// NumpadDecimal,
	// NumpadDivide,
	// NumpadEnter,
	// NumpadEqual,
	NumpadHash,
	// NumpadMemoryAdd,
	// NumpadMemoryClear,
	// NumpadMemoryRecall,
	// NumpadMemoryStore,
	// NumpadMemorySubtract,
	NumpadMultiply,
	NumpadParenLeft,
	NumpadParenRight,
	// NumpadStar,
	// NumpadSubtract,

	// Function keys
	Escape,
	F1,
	F2,
	F3,
	F4,
	F5,
	F6,
	F7,
	F8,
	F9,
	F10,
	F11,
	F12,
	F13,
	F14,
	F15,
	F16,
	F17,
	F18,
	F19,
	F20,
	F21,
	F22,
	F23,
	F24,
	Fn,
	FnLock,
	PrintScreen,
	ScrollLock,
	Pause,

	// Unidentified keys
	Unidentified,

	// Other keys that aren't part of the W3C spec
	Command,
	/// "Ctrl" on Windows/Linux, "Cmd" on Mac
	Accel,
	Lmb,
	Rmb,
	Mmb,

	// This has to be the last element in the enum
	NumKeys,
}

impl fmt::Display for Key {
	// TODO: Relevant key labels should be localized when we get around to implementing localization/internationalization
	fn fmt(&self, f: &mut fmt::Formatter) -> std::fmt::Result {
		let key_name = format!("{self:?}");

		// Writing system keys
		const DIGIT_PREFIX: &str = "Digit";
		if key_name.len() == DIGIT_PREFIX.len() + 1 && &key_name[0..DIGIT_PREFIX.len()] == "Digit" {
			return write!(f, "{}", key_name.chars().skip(DIGIT_PREFIX.len()).collect::<String>());
		}
		const KEY_PREFIX: &str = "Key";
		if key_name.len() == KEY_PREFIX.len() + 1 && &key_name[0..KEY_PREFIX.len()] == "Key" {
			return write!(f, "{}", key_name.chars().skip(KEY_PREFIX.len()).collect::<String>());
		}

		let name = match self {
			// Writing system keys
			Self::Backquote => "`",
			Self::Backslash => "\\",
			Self::BracketLeft => "[",
			Self::BracketRight => "]",
			Self::Comma => ",",
			Self::Equal => "=",
			Self::Minus => "-",
			Self::Period => ".",
			Self::Quote => "'",
			Self::Semicolon => ";",
			Self::Slash => "/",

			Self::Backspace => "⌫",

			// Control pad keys
			Self::Delete => "Del",
			Self::PageDown => "PgDn",
			Self::PageUp => "PgUp",

			// Arrow pad keys
			Self::ArrowDown => "↓",
			Self::ArrowLeft => "←",
			Self::ArrowRight => "→",
			Self::ArrowUp => "↑",

			// Numpad keys
			Self::NumpadAdd => "Numpad +",
			Self::NumpadHash => "Numpad #",
			Self::NumpadMultiply => "Numpad *",
			Self::NumpadParenLeft => "Numpad (",
			Self::NumpadParenRight => "Numpad )",

			// Function keys
			Self::Escape => "Esc",
			Self::PrintScreen => "PrtScr",

			// Other keys that aren't part of the W3C spec
			Self::Command => "⌘",
			Self::Accel =>  "Ctrl",

			_ => key_name.as_str(),
		};

		write!(f, "{name}")
	}
}