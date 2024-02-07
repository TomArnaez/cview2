use serde::{Deserialize, Serialize};
use uuid::Uuid;
use glam::IVec2;

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
    pub min: u32,
    pub max: u32,
    pub brightness: Percentage,
    pub contrast: Percentage
}

pub trait Command {
    fn execute(&mut self);
    fn undo(&self);
}

#[derive(Copy, Clone, Serialize, Deserialize, specta::Type)]
pub struct ImagePosition(pub IVec2);

#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq, Hash, specta::Type)]
pub struct ImageId(pub Uuid);

#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq, Hash, specta::Type)]
pub struct AnnotationId(pub Uuid);