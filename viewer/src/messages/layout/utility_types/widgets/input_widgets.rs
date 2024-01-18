use crate::messages::input_mapper::utility_types::misc::ActionKeys;
use crate::messages::layout::utility_types::widget_prelude::*;

use serde::{Deserialize, Serialize};

#[derive(Clone, Serialize, Deserialize, specta::Type)]
pub struct CheckboxInput {
	pub checked: bool,

	pub disabled: bool,

	pub icon: String,

	pub tooltip: String,

	#[serde(skip)]
	pub tooltip_shortcut: Option<ActionKeys>,

	// Callbacks
	#[serde(skip)]
	pub on_update: WidgetCallback<CheckboxInput>,
}

impl Default for CheckboxInput {
	fn default() -> Self {
		Self {
			checked: false,
			disabled: false,
			icon: "Checkmark".into(),
			tooltip: Default::default(),
			tooltip_shortcut: Default::default(),
			on_update: Default::default(),
		}
	}
}