use crate::messages::prelude::*;

#[derive(Debug)]
pub enum Message {
    NoOp,
    Init,

    Debug(DebugMessage),
    Detector(DetectorMessage),
    Tool(ToolMessage)
}