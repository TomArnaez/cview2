use crates::messages::prelude::Message;

use serde::{Serialize, Deserialize};

#[derive(PartialEq, Clone, Debug, Serialize, Deserialize, specta::Type)]
pub enum ToolColourType {
    Primary,
    Secondary,
}