use image::{Luma, ImageBuffer};

use crate::messages::prelude::*;

use super::utility_types::{misc::{Command, AnnotationId, AdjustmentLevels}, annotations::{Annotation, Shape}};

pub struct ImageMessageHandler {
    image_buffer: ImageBuffer<Luma<u16>, Vec<u16>>,
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

    fn get_mean() {
    }

    fn get_mean_in_shape(&self, shape: &dyn Shape) -> f64 {
        let positions = shape.get_positions();
        let mut mean = 0.0;
        mean
    }


    fn get_annotation(&self, id: AnnotationId) -> Option<&dyn Annotation> {
        self.annotations.get(&id).map(|boxed| boxed.as_ref())
    }

    fn get_image(&self) -> ImageBuffer<Luma<u16>, Vec<u16>> {
        let mut adjusted_image = self.image_buffer.clone();
        self.adjust_brightness(&mut adjusted_image);
        adjusted_image
    }

    fn adjust_brightness(&self, image: &mut ImageBuffer<Luma<u16>, Vec<u16>>) {
        let brightness_factor = self.adjustment_levels.brightness.get() as f32 / 100.0;
        for pixel in image.pixels_mut() {
            let Luma([luma]) = *pixel;
            let adjusted_luma = (luma as f32 * brightness_factor) as u16;
            *pixel = Luma([adjusted_luma]);
        }
    }
}