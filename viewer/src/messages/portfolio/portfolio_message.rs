use serde::{Deserialize, Serialize};

#[derive(PartialEq, Clone, Debug, Serialize, Deserialize)]
pub enum PortfolioMessage {
    OpenStack,
    PrevDocument
}