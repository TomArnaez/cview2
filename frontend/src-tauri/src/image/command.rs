pub trait Command {
    fn execute(&mut self);
    fn undo(&mut self);
}

pub enum FlipDirection {
    X,
    Y
}

impl Command for FlipDirection {
    fn execute(&mut self) {

    }

    fn undo(&mut self) {

    }
}