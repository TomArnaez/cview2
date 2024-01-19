use crate::messages::prelude::*;

use serde::{Deserialize, Serialize};

#[derive(PartialEq, Clone, Debug, Serialize, Deserialize, specta::Type)]
pub enum DialogMessage {
    SelectDetectorDialog(SelectDetectorDialogMessage),
}