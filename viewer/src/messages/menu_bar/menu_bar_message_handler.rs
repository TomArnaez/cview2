use crate::messages::layout::utility_types::widgets::menu_widgets::{MenuBarEntry, MenuBarEntryChildren, MenuLayout};
use crate::messages::prelude::*;
use crate::messages::layout::utility_types::widget_prelude::*;

#[derive(Debug, Clone, Default)]
pub struct MenuBarMessageHandler {
    has_active_image: bool,
}

impl MessageHandler<MenuBarMessage, ()> for MenuBarMessageHandler {
    fn process_message(&mut self, message: MenuBarMessage, responses: &mut VecDeque<Message>, _: ()) {
        match message {
            MenuBarMessage::SendLayout => self.send_layout(responses, LayoutTarget::MenuBar),
        }
    }
}

impl LayoutHolder for MenuBarMessageHandler {
    fn layout(&self) -> Layout {
        let no_active_image = !self.has_active_image;

        let menu_bar_entries = vec![
            MenuBarEntry::new_root("File".into(), false, MenuBarEntryChildren(vec![
                vec![
                    MenuBarEntry {
                        label: "Open...".into(),
                        icon: Some("Open".into()),
                        children: MenuBarEntryChildren::empty(),
                        ..MenuBarEntry::default()
                    },
                ],
                vec![
                    MenuBarEntry {
                        label: "Save".into(),
                        disabled: no_active_image,
                        ..MenuBarEntry::default()
                    }
                ],
                vec![
                    MenuBarEntry {
                        label: "Close".into(),
                        disabled: no_active_image,
                        ..MenuBarEntry::default()
                    },
                    MenuBarEntry {
                        label: "Close All".into(),
                        disabled: no_active_image,
                        ..MenuBarEntry::default()
                    }
                ],
                vec![
                    MenuBarEntry {
                        label: "Preference...".into(),
                        icon: Some("Preferences".into()),
                        ..MenuBarEntry::default()
                    }
                ]
            ])),
            MenuBarEntry::new_root("Edit".into(), false, MenuBarEntryChildren(vec![
                vec![
                    MenuBarEntry {
                        label: "Undo".into(),
                        disabled: no_active_image,
                        ..MenuBarEntry::default()
                    },
                    MenuBarEntry {
                        label: "Redo".into(),
                        disabled: no_active_image,
                        ..MenuBarEntry::default()
                    }
                ]
            ]))
        ];
        Layout::MenuLayout(MenuLayout::new(menu_bar_entries))
    }
}