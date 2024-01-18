use crate::messages::layout::utility_types::widget_prelude::*;

use proc_macros::WidgetBuilder;

use derivative::*;
use serde::{Deserialize, Serialize};

#[derive(Clone, Default, Derivative, Serialize, Deserialize, WidgetBuilder, specta::Type)]
#[derivative(Debug, PartialEq)]
pub struct IconButton {
	#[widget_builder(constructor)]
	pub icon: String,
	
	#[widget_builder(constructor)]
	pub size: u32, // TODO: Convert to an `IconSize` enum

	pub disabled: bool,

	pub active: bool,

	pub tooltip: String,

	// Callbacks
	#[serde(skip)]
	#[derivative(Debug = "ignore", PartialEq = "ignore")]
	pub on_update: WidgetCallback<IconButton>,
}

#[derive(Clone, Serialize, Deserialize, Derivative, specta::Type)]
#[derivative(Debug, PartialEq, Default)]
pub struct PopoverButton {
	pub style: Option<String>,

	pub icon: Option<String>,

	pub disabled: bool,

	// Placeholder popover content heading
	pub header: String,

	// Placeholder popover content paragraph
	pub text: String,

	pub tooltip: String,

	#[serde(rename = "optionsWidget")]
	pub options_widget: SubLayout,

	#[serde(rename = "popoverMinWidth")]
	pub popover_min_width: Option<u32>,
}

#[derive(Clone, Serialize, Deserialize, Derivative, Default, specta::Type)]
#[derivative(Debug, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct TextButton {
	pub label: String,

	pub icon: Option<String>,

	pub flush: bool,

	pub emphasized: bool,

	#[serde(rename = "minWidth")]
	pub min_width: u32,

	pub disabled: bool,

	pub tooltip: String,

	// Callbacks
	#[serde(skip)]
	#[derivative(Debug = "ignore", PartialEq = "ignore")]
	pub on_update: WidgetCallback<TextButton>,
}