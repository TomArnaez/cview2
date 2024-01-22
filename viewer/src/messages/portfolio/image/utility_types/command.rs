use crate::messages::prelude::*;

use super::{misc::{AnnotationId, Command}, annotations::{Annotation, Shape}};

pub struct AddAnnotationCommand<'a> {
    pub annotation: Box<dyn Annotation>,
    pub image_handler: &'a ImageMessageHandler,
}

impl<'a> Command for AddAnnotationCommand<'a> {
    fn execute(&mut self) {
    }

    fn undo(&self) {
        
    }
}

pub struct SetValue {
    pub shape: Box<dyn Shape>,
    pub value: u32,
}