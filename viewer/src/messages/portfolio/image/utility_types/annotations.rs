use serde::{Deserialize, Serialize};

use super::misc::ImagePosition;

pub trait Annotation {
    fn get_position(&self) -> ImagePosition;
    fn set_position(&self, new_position: ImagePosition);
}

pub trait Shape: Annotation {
    fn get_area(&self) -> f64;
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

    fn set_position(&self, new_position: ImagePosition) {
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

    fn set_position(&self, new_position: ImagePosition) {
        self.position = new_position;
    }
}

impl Shape for Circle {
    fn get_area(&self) -> f64 {
        3.14159 * self.radius * self.radius
    }
}

#[derive(Serialize, Deserialize, specta::Type)]
pub struct Line {
    start_position: ImagePosition,
    end_position: ImagePosition,
}

#[derive(Serialize, Deserialize, specta::Type)]
pub struct Rectangle {
    position: ImagePosition,
    width: u32,
    height: u32
}

impl Annotation for Rectangle  {
    fn get_position(&self) -> ImagePosition {
        self.position
    }

    fn set_position(&self, new_position: ImagePosition) {
        self.position = new_position;
    }
}

impl Shape for Rectangle {
    fn get_area(&self) -> f64 {
        (self.width * self.height).into()
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