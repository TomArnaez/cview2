use crate::utility_traits::MessageHandler;
use crate::messages::prelude::*;
use super::image::utility_types::misc::ImageId;

pub struct PortfolioMessageHandler {
    images: HashMap<ImageId, ImageMessageHandler>,
    image_ids: Vec<ImageId>,
    active_image_id: Option<ImageId>
}

impl MessageHandler<PortfolioMessage, ()> for PortfolioMessageHandler {
    fn process_message(&mut self, message: PortfolioMessage, responses: &mut VecDeque<Message>, data: ()) {
        match message {
            PortfolioMessage::SelectImage { image_id } => {
                self.active_image_id = Some(image_id);
            }
        }
    }
}

impl PortfolioMessageHandler {
    pub fn image(&self, image_id: ImageId) -> Option<&ImageMessageHandler> {
        self.images.get(&image_id)
    }

    pub fn image_mut(&self, image_id: ImageId) -> Option<&ImageMessageHandler> {
        self.images.get_mut(&image_id)
    }

    pub fn active_image(&self) -> Option<&ImageMessageHandler> {
        self.active_image_id.and_then(|id| self.images.get(&id))
    }

    pub fn active_image_mut(&mut self) -> Option<&ImageMessageHandler> {
        self.active_image_id.and_then(|id| self.images.get_mut(&id))
    }

    pub fn active_image_id(&self) -> Option<ImageId> {
        self.active_image_id
    }
}