use super::misc::ImagePosition;

use serde::{Deserialize, Serialize};


pub trait Annotation {
    fn get_position(&self) -> ImagePosition;
    fn set_position(&mut self, new_position: ImagePosition);
}

pub trait Shape: Annotation {    
    fn get_area(&self) -> f64;
    fn get_positions(&self) -> Box<dyn Iterator<Item = ImagePosition>>;
}

#[derive(Serialize, Deserialize, specta::Type)]
pub struct TextAnnotation {
    position: ImagePosition,
    content: String,
}

impl Annotation for TextAnnotation {
    fn get_position(&self) -> ImagePosition {
        self.position
    }

    fn set_position(&mut self, new_position: ImagePosition) {
        self.position = new_position;
    }
}

#[derive(Serialize, Deserialize, specta::Type)]
struct Circle {
    position: ImagePosition,
    radius: f64,
}

impl Annotation for Circle {
    fn get_position(&self) -> ImagePosition {
        self.position
    }

    fn set_position(&mut self, new_position: ImagePosition) {
        self.position = new_position;
    }
}

#[derive(Serialize, Deserialize, specta::Type)]
pub struct Line {
    start_position: ImagePosition,
    end_position: ImagePosition,
}

#[derive(Copy, Clone, Serialize, Deserialize, specta::Type)]
pub struct Rectangle {
    position: ImagePosition,
    width: u32,
    height: u32
}

pub struct RectangleIterator {
    rectangle: Rectangle,
    current_x: u32,
    current_y: u32
}

impl Annotation for Rectangle  {
    fn get_position(&self) -> ImagePosition {
        self.position
    }

    fn set_position(&mut self, new_position: ImagePosition) {
        self.position = new_position;
    }
}

impl Shape for Rectangle {
    fn get_area(&self) -> f64 {
        (self.width * self.height).into()
    }

    fn get_positions(&self) -> Box<dyn Iterator<Item = ImagePosition>> {
        Box::new(self.iter())
    }
}

impl Rectangle {
    fn iter(&self) -> RectangleIterator {
        RectangleIterator {
            rectangle: *self,
            current_x: 0,
            current_y: 0,
        }
    }
}

impl Iterator for RectangleIterator {
    type Item = ImagePosition;

    fn next(&mut self) -> Option<Self::Item> {
        None
        // if self.current_y < self.rectangle.height {
        //     let result = Some(ImagePosition((self.rectangle.position.0.x as u32 + self.current_x, self.rectangle.position.0.y as u32 + self.current_y)));
        //     self.current_x += 1;
        //     if self.current_x == self.rectangle.width {
        //         self.current_x = 0;
        //         self.current_y += 1;
        //     }
        //     result
        // } else {
        //     None
        // }
    }
}

#[derive(Serialize, Deserialize, specta::Type)]
#[serde(tag = "type")]
pub enum AnnotationEnum {
    Text(TextAnnotation),
    Shape(ShapeEnum)
}

#[derive(Serialize, Deserialize, specta::Type)]
pub enum ShapeEnum {
    Circle(Circle),
    Rectangle(Rectangle)
}