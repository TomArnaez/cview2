use crate::messages::layout::utility_types::widget_prelude::*;
use crate::messages::prelude::*;

use serde::{Deserialize, Serialize};

#[derive(PartialEq, Clone, Debug, Serialize, Deserialize)]
pub enum LayoutMessage {
	ResendActiveWidget {
		layout_target: LayoutTarget,
		widget_id: WidgetId,
	},
	SendLayout {
		layout: Layout,
		layout_target: LayoutTarget,
	},
	UpdateLayout {
		layout_target: LayoutTarget,
		widget_id: WidgetId,
		value: serde_json::Value,
	},
}