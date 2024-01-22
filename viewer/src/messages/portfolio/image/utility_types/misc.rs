use serde::{Deserialize, Serialize};
use image::{ImageBuffer, Luma};
use uuid::Uuid;

#[derive(Serialize, Deserialize, specta::Type)]
pub struct Percentage(u32);

impl TryFrom<u32> for Percentage {
    type Error = String;

    fn try_from(value: u32) -> Result<Self, Self::Error> {
        if (0..=100).contains(&value) {
            Ok(Percentage(value))
        } else {
            Err("Percentage must be between 0 and 100".into())
        }
    }
}

impl Percentage {
    pub fn get(&self) -> u32 {
        self.0
    }
}

#[derive(Serialize, Deserialize, specta::Type)]
pub struct AdjustmentLevels {
    min: u32,
    max: u32,
    brightness: Percentage,
    contrast: Percentage
}

pub trait Command {
    fn execute(&mut self);
    fn undo(&self);
}

#[derive(Serialize, Deserialize, specta::Type)]
pub struct ImagePosition(pub (u32, u32));

#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq, Hash, specta::Type)]
pub struct ImageId(pub Uuid);

#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq, Hash, specta::Type)]
pub struct AnnotationId(pub Uuid);

pub trait ImageType {
}

impl ImageType for ImageBuffer<Luma<u16>, Vec<u16>> {
}

impl ImageType for ImageBuffer<Luma<f32>, Vec<f32>> {
}
