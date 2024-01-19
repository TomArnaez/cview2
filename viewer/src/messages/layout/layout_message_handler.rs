use log::warn;

use crate::messages::layout::utility_types::widget_prelude::*;
use crate::messages::prelude::*;

#[derive(Debug, Clone, Default)]
pub struct LayoutMessageHandler {
	layouts: [Layout; LayoutTarget::LayoutTargetLength as usize],
}

impl LayoutMessageHandler {
	/// Get the widget path for the widget with the specified id
	fn get_widget_path(widget_layout: &WidgetLayout, widget_id: WidgetId) -> Option<(&WidgetHolder, Vec<usize>)> {
		let mut stack = widget_layout.layout.iter().enumerate().map(|(index, val)| (vec![index], val)).collect::<Vec<_>>();
		while let Some((mut widget_path, group)) = stack.pop() {
			match group {
				// Check if any of the widgets in the current column or row have the correct id
				LayoutGroup::Column { widgets } | LayoutGroup::Row { widgets } => {
					for (index, widget) in widgets.iter().enumerate() {
						// Return if this is the correct ID
						if widget.widget_id == widget_id {
							widget_path.push(index);
							return Some((widget, widget_path));
						}

						if let Widget::PopoverButton(popover) = &widget.widget {
							stack.extend(popover.options_widget.iter().enumerate().map(|(child, val)| ([widget_path.as_slice(), &[index, child]].concat(), val)));
						}
					}
				}
				// A section contains more LayoutGroups which we add to the stack.
				LayoutGroup::Section { layout, .. } => {
					stack.extend(layout.iter().enumerate().map(|(index, val)| ([widget_path.as_slice(), &[index]].concat(), val)));
				}
			}
		}
		None
	}
}

impl<F: Fn(&MessageDiscriminant) -> Vec<KeysGroup>> MessageHandler<LayoutMessage, F> for LayoutMessageHandler {
	fn process_message(&mut self, message: LayoutMessage, responses: &mut VecDeque<Message>, action_input_mapping: ()) {
		use LayoutMessage::*;
		match message {
			ResendActiveWidget { layout_target, widget_id } => {
				// Find the updated diff based on the specified layout target
				let Some(diff) = (match &self.layouts[layout_target as usize] {
					Layout::MenuLayout(_) => return,
					Layout::WidgetLayout(layout) => Self::get_widget_path(layout, widget_id).map(|(widget, widget_path)| {
						// Create a widget update diff for the relevant id
						let new_value = DiffUpdate::Widget(widget.clone());
						WidgetDiff { widget_path, new_value }
					}),
				}) else {
					return;
				};
				// Resend that diff
				self.send_diff(vec![diff], layout_target, responses, &action_input_mapping);
			},
			SendLayout { layout, layout_target } => self.diff_and_send_layout_to_frontend(layout_target, layout, responses, &action_input_mapping),
			UpdateLayout { layout_target, widget_id, value } => {
				// Look up the layout
				let layout = if let Some(layout) = self.layouts.get_mut(layout_target as usize) {
					layout
				} else {
					warn!("UpdateLayout was called referencing an invalid layout. `widget_id: {widget_id}`, `layout_target: {layout_target:?}`",);
					return;
				};

				let widget_holder = if let Some(widget_holder) = layout.iter_mut().find(|widget| widget.widget_id == widget_id) {
					widget_holder
				} else {
					warn!("UpdateLayout was called referencing an invalid widget ID, although the layout target was valid. `widget_id: {widget_id}`, `layout_target: {layout_target:?}`",);
					return;
				};

				match &mut widget_holder.widget {
					Widget::IconButton(icon_button) => {
						let callback_message = (icon_button.on_update.callback)(icon_button);
						responses.add(callback_message);
					},
					Widget::PopoverButton(_) => {},
					Widget::TextButton(text_button) => {
						let callback_message = (text_button.on_update.callback)(text_button);
						responses.add(callback_message);
					}
				};
				responses.add(Message::Layout(ResendActiveWidget { layout_target, widget_id: widget_id }));
			}
		}
	}
}

impl LayoutMessageHandler {
	fn diff_and_send_layout_to_frontend(
		&mut self,
		layout_target: LayoutTarget,
		new_layout: Layout,
		responses: &mut VecDeque<Message>,
		//action_input_mapping: &impl Fn(&MessageDiscriminant) -> Vec<KeysGroup>,
	) {
		// We don't diff the menu bar layout yet.
		if matches!(new_layout, Layout::MenuLayout(_)) {
			// Skip update if the same
			if self.layouts[layout_target as usize] == new_layout {
				return;
			}
			// Update the backend storage
			self.layouts[layout_target as usize] = new_layout;
			// Update the UI
			responses.add(FrontendMessage::UpdateMenuBarLayout {
				layout_target,
				layout: self.layouts[layout_target as usize].clone().unwrap_menu_layout(action_input_mapping).layout,
			});
			return;
		}

		let mut widget_diffs = Vec::new();
		self.layouts[layout_target as usize].diff(new_layout, &mut Vec::new(), &mut widget_diffs);
		// Skip sending if no diff.
		if widget_diffs.is_empty() {
			return;
		}

		self.send_diff(widget_diffs, layout_target, responses, action_input_mapping);
	}}