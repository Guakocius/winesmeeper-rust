use crate::model::board::Board;
use crate::observer::{Event, EventData, Observer};

pub struct Game {
    board: Board,
    observer: Observer,
}

impl Game {
    pub fn turn(&mut self, x: usize, y: usize) {
        let is_bomb = self.board.get_field(x, y) == -2;

        self.observer.notify(
            &Event::Turn,
            EventData::Turn { x, y},
        );

        self.observer.notify(
            &Event::FieldOpened,
            EventData::FieldOpened { x, y, bomb: is_bomb,},
        )
    }
}
