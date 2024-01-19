pub use crate::utility_traits::{MessageHandler};

pub use crate::messages::dialog::select_detector_dialog::{SelectDetectorDialogMessage, SelectDetectorMessageHandler};
pub use crate::messages::dialog::{DialogMessage, DialogMessageHandler};
//pub use crate::messages::debug::{DebugMessage, DebugMessageHandler};
pub use crate::messages::detector::{DetectorMessage, DetectorMessageHandler};
pub use crate::messages::frontend::FrontendMessage;
pub use crate::messages::menu_bar::{MenuBarMessage, MenuBarMessageHandler};
pub use crate::messages::layout::{LayoutMessage, LayoutMessageHandler};
pub use crate::messages::portfolio::image::{ImageMessage, ImageMessageHandler};
pub use crate::messages::portfolio::{PortfolioMessage};
pub use crate::messages::tool::{ToolMessage, ToolMessageHandler};
pub use crate::messages::tool::tool_messages::line_tool::LineToolMessage;
pub use crate::messages::tool::tool_messages::select_tool::SelectToolMessage;
pub use crate::messages::tool::tool_messages::rectangle_tool::RectangleToolMessage;

pub use crate::messages::message::{Message};

pub use std::collections::{VecDeque};

pub trait Responses {
	fn add(&mut self, message: impl Into<Message>);

	fn add_front(&mut self, message: impl Into<Message>);

	fn try_add(&mut self, message: Option<impl Into<Message>>) {
		if let Some(message) = message {
			self.add(message);
		}
	}
}

impl Responses for VecDeque<Message> {
	fn add(&mut self, message: impl Into<Message>) {
		self.push_back(message.into());
	}

	fn add_front(&mut self, message: impl Into<Message>) {
		self.push_front(message.into());
	}
}