use crate::messages::prelude::*;
use serde::{Deserialize, Serialize};


#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, specta::Type)]
pub struct WidgetHolder {
	#[serde(rename = "widgetId")]
	pub widget_id: WidgetId,
	pub widget: Widget,
}

#[derive(Clone)]
pub struct WidgetCallback<T> {
	pub callback: Arc<dyn Fn(&T) -> Message + 'static + Send + Sync>,
}

impl<T> WidgetCallback<T> {
	pub fn new(callback: impl Fn(&T) -> Message + 'static + Send + Sync) -> Self {
		Self { callback: Arc::new(callback) }
	}
}

impl<T> Default for WidgetCallback<T> {
	fn default() -> Self {
		Self::new(|_| Message::NoOp)
	}
}

#[remain::sorted]
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, specta::Type)]
pub enum Widget {
	CheckboxInput(CheckboxInput),
}