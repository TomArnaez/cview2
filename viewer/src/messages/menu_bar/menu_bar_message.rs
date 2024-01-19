use crate::messages::prelude::*;

use serde::{Deserialize, Serialize};

#[derive(PartialEq, Eq, Clone, Debug, Hash, Serialize, Deserialize)]
pub enum MenuBarMessage {
	// Messages
	SendLayout,
}