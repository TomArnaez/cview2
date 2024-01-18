use crate::messages::prelude::*;

#[derive(Debug)]
pub enum Message {
    NoOp,
    Init,

    //Debug(DebugMessage),
    Detector(DetectorMessage),
    Dialog(DialogMessage),
    Frontend(FrontendMessage),
    Layout(LayoutMessage),
    Tool(ToolMessage)
}