pub trait Fsm {
    type toolData;

    type ToolOptions;
}

pub trait ToolMetadata {
    fn icon_name(&self) -> String;
    fn tooltip(&self) -> String;
    fn tool_type(&self) -> ToolType;
}

#[derive(Default, Debug)]
pub struct ToolData {
    pub active_tool_type: ToolType,
}

#[derive(Default, Debug)]
pub struct ToolFsmState {
    pub tool_data: ToolData,
}

#[derive(Default, Debug)]
pub enum ToolType {
    #[default]
    Select
}