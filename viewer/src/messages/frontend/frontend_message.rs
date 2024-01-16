use serde::{Serialize, Deserialize};

#[derive(PartialEq, Clone, Debug, Serialize, Deserialize, specta::Type)]
pub enum FrontendMessage {
    TriggerViewportResize,
}