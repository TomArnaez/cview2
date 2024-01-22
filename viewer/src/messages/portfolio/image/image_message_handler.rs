use crate::messages::prelude::*;

use super::utility_types::{misc::{Command, ImageType, ImagePosition, AnnotationId, AdjustmentLevels}, annotations::Annotation, command::AddAnnotationCommand};

pub struct ImageMessageHandler {
    image_buffer: Box<dyn ImageType>,
    annotations: HashMap<AnnotationId, Box<dyn Annotation>>,
    annotation_ids: Vec<AnnotationId>,
    adjustment_levels: AdjustmentLevels,
    image_redo_history: Vec<Box<dyn Command>>,
    image_undo_history: Vec<Box<dyn Command>>,
}

impl MessageHandler<ImageMessage, ()> for ImageMessageHandler {
    fn process_message(&mut self, message: ImageMessage, responses: &mut VecDeque<Message>, data: ()) {
        match message {
            ImageMessage::AddAnnotation { annotation } => {
                todo!();
            },
            ImageMessage::RemoveAnnotation { annotation_id } => {
                self.annotations.remove(&annotation_id);
            }
            _ => {}
        }
    }
}

impl ImageMessageHandler {
    fn execute_command(&mut self, mut command: Box<dyn Command>) {
        command.execute();
        self.image_undo_history.push(command);
    }

    fn undo(&mut self, respones: &mut VecDeque<Message>) {
        let Some(mut command) = self.image_undo_history.pop() else { return };
        command.undo();
        self.image_redo_history.push(command);
    }

    fn redo(&mut self) {
        let Some(mut command) = self.image_redo_history.pop() else { return };
        command.execute();
        self.image_undo_history.push(command);
    }

    fn get_pixel(&self, position: ImagePosition) {
    }

    fn get_annotation(&self, id: AnnotationId) -> Option<&dyn Annotation> {
        self.annotations.get(&id).map(|boxed| boxed.as_ref())
    }
}