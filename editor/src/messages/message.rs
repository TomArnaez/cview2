use crate::messages::prelude::*;

#[derive(Debug)]
pub enum Message {
    Debug(DebugMessage),
    Detector(DetectorMessage),
    Tool(ToolMessage)
}