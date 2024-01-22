use super::utility_types::{annotations::{Annotation, AnnotationEnum}, misc::AnnotationId};

pub enum ImageMessage {
    AddAnnotation {
        annotation: AnnotationEnum
    },
    AdjustBrightness {
        new_brightness: f32
    },
    RemoveAnnotation {
        annotation_id: AnnotationId
    },
    SetValue {
    },
    SubtractValue {},
    Undo,
    ZoomCanvasTo100Perecent,
    ZoomCanvasTo200Percent,
    ZoomCanvasToFitAll,
}