use crate::messages::prelude::*;

use serde::{Deserialize, Serialize};

#[derive(PartialEq, Clone, Debug, Serialize, Deserialize)]
pub enum DialogMessage {
    SelectDetectorDialog(SelectDetectorDialogMessage),
}