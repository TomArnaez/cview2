use serde::{Deserialize, Serialize};

use crate::messages::prelude::*;

#[derive(Debug, Serialize, Deserialize, specta::Type)]
pub enum Message {
    NoOp,
    Init,

    //Debug(DebugMessage),
    // Detector(DetectorMessage),
    Dialog(DialogMessage),
    Frontend(FrontendMessage),
    Tool(ToolMessage)
}

pub struct MessageDiscriminant(pub u8);