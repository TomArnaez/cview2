use std::collections::VecDeque;

use crate::messages::prelude::*;

pub trait MessageHandler<M, D>
    {
        fn process_message(&mut self, message: M, responses: &mut VecDeque<Message>, data: D);
    }