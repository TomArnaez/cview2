use super::image::{DynImage, FlipDirection};

pub trait Command {
    fn execute(&self, image: &mut dyn DynImage);
    fn undo(&self, image: &mut dyn DynImage);
}


impl Command for FlipDirection {
    fn execute(&self, image: &mut dyn DynImage) {
        match self {
            FlipDirection::X => image.flip(FlipDirection::X),
            FlipDirection::Y => image.flip(FlipDirection::Y),
        }
    }

    fn undo(&self, image: &mut dyn DynImage) {
        match self {
            FlipDirection::X => image.flip(FlipDirection::Y),
            FlipDirection::Y => image.flip(FlipDirection::X),
        }
    }
}