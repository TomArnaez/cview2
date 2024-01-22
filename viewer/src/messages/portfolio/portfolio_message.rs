use serde::{Deserialize, Serialize};

use super::image::utility_types::misc::ImageId;

#[derive(PartialEq, Clone, Debug, Serialize, Deserialize)]
pub enum PortfolioMessage {
    SelectImage {
        image_id: ImageId
    }
}